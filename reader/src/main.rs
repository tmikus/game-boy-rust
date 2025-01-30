extern crate rppal;

use std::clone::Clone;
use rppal::gpio::{InputPin, OutputPin};
use {
    rppal::gpio::{Gpio, Level},
    std::{
        fs,
        thread::sleep,
        time::Duration,
    },
};

const SLEEP: u64 = 10;

const CS_PIN_ID: u8 = 18;

const LATCH_PIN_ID: u8 = 24;
const DATA_PIN_ID: u8 = 23;
const CLOCK_PIN_ID: u8 = 25;
const READ_PIN_ID: u8 = 15;
const WRITE_PIN_ID: u8 = 14;

const DATA_0_ID: u8 = 8;
const DATA_1_ID: u8 = 7;
const DATA_2_ID: u8 = 19;
const DATA_3_ID: u8 = 9;
const DATA_4_ID: u8 = 10;
const DATA_5_ID: u8 = 22;
const DATA_6_ID: u8 = 21;
const DATA_7_ID: u8 = 17;

const DATA_PINS: [u8; 8] = [
    DATA_0_ID,
    DATA_1_ID,
    DATA_2_ID,
    DATA_3_ID,
    DATA_4_ID,
    DATA_5_ID,
    DATA_6_ID,
    DATA_7_ID,
];

fn shift_out(gpio: &mut GpioReader, value: u8) {
    for number in (0..8).rev() {
        gpio.clock_pin.set_low();
        sleep(Duration::from_micros(SLEEP));
        gpio.data_pin.write(match value >> number & 1 {
            1 => Level::High,
            0 => Level::Low,
            _ => unreachable!(),
        });
        sleep(Duration::from_micros(SLEEP));
        gpio.clock_pin.set_high();
        sleep(Duration::from_micros(SLEEP));
    }
}

struct GpioReader {
    pub gpio: Gpio,
    pub latch_pin: OutputPin,
    pub clock_pin: OutputPin,
    pub data_pin: OutputPin,
    pub read_pin: OutputPin,
    pub write_pin: OutputPin,
}

impl GpioReader {
    fn new() -> Result<GpioReader, rppal::gpio::Error> {
        let mut gpio = Gpio::new()?;
        let latch_pin = gpio.get(LATCH_PIN_ID)?.into_output();
        let clock_pin = gpio.get(CLOCK_PIN_ID)?.into_output();
        let data_pin = gpio.get(DATA_PIN_ID)?.into_output();
        let read_pin = gpio.get(READ_PIN_ID)?.into_output();
        let write_pin = gpio.get(WRITE_PIN_ID)?.into_output();
        Ok(GpioReader {
            gpio,
            latch_pin,
            clock_pin,
            data_pin,
            read_pin,
            write_pin,
        })
    }

    fn get_input_data_pins(&self) -> Vec<InputPin> {
        DATA_PINS.iter().map(|pin| self.gpio.get(*pin).unwrap().into_input()).collect()
    }

    fn get_output_data_pins(&self) -> Vec<OutputPin> {
        DATA_PINS.iter().map(|pin| self.gpio.get(*pin).unwrap().into_output()).collect()
    }
}

fn test_pins() {
    // let mut gpio = GpioReader::new().unwrap();
    let mut gpio = Gpio::new().unwrap();
    let pins = vec![14, 15, 18, 23, 24, 25, 8, 7, 12, 16, 20, 21, 26, 19];
    for pin_id in pins.iter() {
        let mut pin = gpio.get(*pin_id).unwrap().into_output();
        pin.set_low();
    }
    sleep(Duration::from_secs(3));
    while true {
        for pin_id in pins.iter() {
            let mut pin = gpio.get(*pin_id).unwrap().into_output();
            pin.set_high();
            sleep(Duration::from_secs(1));
            pin.set_low();
        }
    }

}

fn main() {
    // test_pins();
    // return;
    println!("Start reading");
    let mut gpio = GpioReader::new().unwrap();
    gpio.read_pin.set_low();
    gpio.write_pin.set_high();
    sleep(Duration::from_micros(SLEEP));
    let mut data: Vec<u8> = Vec::new();
    // Read the data from bank 0
    read_next_rom_bank(&mut gpio, &mut data, 0);
    let banks_count = get_banks_per_rom(&data);
    println!("Banks count: {}", banks_count);
    for bank in 1..banks_count {
        println!("Reading bank {}..", bank);
        select_rom_bank(&mut gpio, bank);
        read_next_rom_bank(&mut gpio, &mut data, 0x4000);
    }
    fs::write("rom.dat", data).unwrap();
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

fn read_next_rom_bank(gpio: &mut GpioReader, data: &mut Vec<u8>, start_address: u16) {
    let data_pins = gpio.get_input_data_pins();
    for address in start_address..=(start_address + 0x3FFF) {
        write_address(gpio, address);
        sleep(Duration::from_micros(SLEEP));
        // sleep(Duration::from_millis(300));
        let mut value = 0u8;
        for (bit, pin) in data_pins.iter().enumerate() {
            if pin.read() == Level::High {
                value |= (1 << bit) as u8;
            }
        }
        data.push(value);
    }
}

fn select_rom_bank(gpio: &mut GpioReader, bank: u16) {
    gpio.read_pin.write(Level::High);
    gpio.write_pin.write(Level::Low);
    sleep(Duration::from_micros(SLEEP));
    let mut output_pins = gpio.get_output_data_pins();
    write_address(gpio, 0x2100);
    sleep(Duration::from_micros(SLEEP));
    for (index, mut pin) in output_pins.iter_mut().enumerate() {
        if bank & (1 << index) != 0 {
            pin.write(Level::High);
        } else {
            pin.write(Level::Low);
        }
    }
    sleep(Duration::from_micros(SLEEP));
    // Set back to reading ROM
    gpio.read_pin.write(Level::Low);
    gpio.write_pin.write(Level::High);
    for mut pin in output_pins {
        pin.write(Level::Low);
    }
}

fn write_address(gpio: &mut GpioReader, address: u16) {
    gpio.latch_pin.set_low();
    shift_out(gpio, ((address >> 8) as u8));
    shift_out(gpio, ((address & 0xFF) as u8));
    gpio.latch_pin.set_high();
}
