extern crate rppal;

use {
    rppal::gpio::{Gpio, Level, Mode},
    std::{
        collections::LinkedList,
        fs,
        thread::sleep,
        time::Duration,
    },
};

// Game Boy Cartridge pinout
// GPIO 23 = PIN 13 = RB PIN 16
// GPIO 18 = ANALOG PIN 5 = RB PIN 12
// GPIO 24 = PIN 11 = RB PIN 18
// GPIO 25 = PIN 10 = RB PIN 22
// GPIO 12 = PIN 12 = RB PIN 32
// --------- DATA ------------
// GPIO 17 = PIN 2 = RB PIN 11
// GPIO 27 = PIN 3 = RB PIN 13
// GPIO 22 = PIN 4 = RB PIN 15
// GPIO 5  = PIN 5 = RB PIN 29
// GPIO 6  = PIN 6 = RB PIN 31
// GPIO 13 = PIN 7 = RB PIN 33
// GPIO 19 = PIN 8 = RB PIN 35
// GPIO 26 = PIN 9 = RB PIN 37

const SLEEP: u64 = 50;

const LATCH_PIN_ID: u8 = 22;
const DATA_PIN_ID: u8 = 18;
const CLOCK_PIN_ID: u8 = 32;
const READ_PIN_ID: u8 = 12;
const WRITE_PIN_ID: u8 = 16;

const DATA_0_ID: u8 = 11;
const DATA_1_ID: u8 = 13;
const DATA_2_ID: u8 = 15;
const DATA_3_ID: u8 = 29;
const DATA_4_ID: u8 = 31;
const DATA_5_ID: u8 = 33;
const DATA_6_ID: u8 = 35;
const DATA_7_ID: u8 = 37;

const data_pins: [u8; 8] = [
    DATA_0_ID,
    DATA_1_ID,
    DATA_2_ID,
    DATA_3_ID,
    DATA_4_ID,
    DATA_5_ID,
    DATA_6_ID,
    DATA_7_ID,
];

fn shift_out(gpio: &Gpio, data_pin: u8, clock_pin: u8, value: u8) {
    for number in (0..8).rev() {
        gpio.write(clock_pin, Level::Low);
        sleep(Duration::from_micros(50));
        gpio.write(data_pin, match value >> number & 1 {
            1 => Level::High,
            0 => Level::Low,
            _ => unreachable!(),
        });
        sleep(Duration::from_micros(50));
        gpio.write(clock_pin, Level::High);
        sleep(Duration::from_micros(50));
    }
}

fn main() {
    println!("Start reading");
    let mut gpio = Gpio::new().unwrap();
    // Setup output pins
    gpio.set_mode(LATCH_PIN_ID, Mode::Output);
    gpio.set_mode(CLOCK_PIN_ID, Mode::Output);
    gpio.set_mode(DATA_PIN_ID, Mode::Output);
    gpio.set_mode(READ_PIN_ID, Mode::Output);
    gpio.set_mode(WRITE_PIN_ID, Mode::Output);
    // Setup input pins
    for pin in data_pins.iter() {
        gpio.set_mode(pin.clone(), Mode::Input);
    }
    gpio.write(READ_PIN_ID, Level::Low);
    gpio.write(WRITE_PIN_ID, Level::High);
    sleep(Duration::from_micros(SLEEP));
    let mut data: Vec<u8> = Vec::new();
    // Read the data from bank 0
    read_next_rom_bank(&gpio, &mut data, 0);
    let banks_count = get_banks_per_rom(&data);
    println!("Banks count: {}", banks_count);
    for bank in 1..banks_count {
        println!("Reading bank {}..", bank);
        select_rom_bank(&mut gpio, bank);
        read_next_rom_bank(&gpio, &mut data, 0x4000);
    }
    fs::write("rom.dat", data);
    println!("Done");
}

fn get_banks_per_rom(data: &[u8]) -> u16 {
    println!("Rom type: {:#04x}", data[0x0148]);
    match data[0x0148] {
        0x00 => 2,
        0x01 => 4,
        0x02 => 8,
        0x03 => 16,
        0x04 => 32,
        0x05 => 64,
        0x06 => 128,
        0x07 => 256,
        0x52 => 72,
        0x53 => 80,
        0x54 => 96,
        _ => 0,
    }
}

fn read_next_rom_bank(gpio: &Gpio, data: &mut Vec<u8>, start_address: u16) {
    for address in start_address..=(start_address + 0x3FFF) {
        write_address(&gpio, address);
        sleep(Duration::from_micros(SLEEP));
        let mut value = 0u8;
        for (bit, pin) in data_pins.iter().enumerate() {
            if gpio.read(pin.clone()).unwrap() == Level::High {
                value |= (1 << bit) as u8;
            }
        }
        data.push(value);
    }
}

fn select_rom_bank(gpio: &mut Gpio, bank: u16) {
    gpio.write(READ_PIN_ID, Level::High);
    gpio.write(WRITE_PIN_ID, Level::Low);
    sleep(Duration::from_micros(5));
    for pin in data_pins.iter() {
        gpio.set_mode(pin.clone(), Mode::Output);
    }
    write_address(&gpio, 0x2100);
    sleep(Duration::from_micros(5));
    for (index, pin) in data_pins.iter().enumerate() {
        if bank & (1 << index) != 0 {
            gpio.write(pin.clone(), Level::High);
        } else {
            gpio.write(pin.clone(), Level::Low);
        }
    }
    sleep(Duration::from_micros(5));
    // Set back to reading ROM
    gpio.write(READ_PIN_ID, Level::Low);
    gpio.write(WRITE_PIN_ID, Level::High);
    for pin in data_pins.iter() {
        gpio.write(pin.clone(), Level::Low);
        gpio.set_mode(pin.clone(), Mode::Input);
    }
}

fn write_address(gpio: &Gpio, address: u16) {
    gpio.write(LATCH_PIN_ID, Level::Low);
    shift_out(&gpio, DATA_PIN_ID, CLOCK_PIN_ID, ((address >> 8) as u8));
    shift_out(&gpio, DATA_PIN_ID, CLOCK_PIN_ID, ((address & 0xFF) as u8));
    gpio.write(LATCH_PIN_ID, Level::High);
}
