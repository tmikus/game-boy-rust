extern crate rppal;

use std::arch::asm;
use std::clone::Clone;
use std::slice::{Iter, IterMut};
use rppal::gpio::{InputPin, IoPin, Mode, OutputPin};
use {
    rppal::gpio::{Gpio, Level},
    std::{
        fs,
        time::Duration,
    },
};

const SLEEP: u64 = 2;

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

fn sleep(duration: Duration) {
    // std::thread::sleep(duration);
    // std::thread::sleep(Duration::from_nanos(10));
    // shuteye::sleep(Duration::from_nanos(1));
    for _ in 0..100 {
        unsafe {
            asm!(
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
                "nop",
            );
        }
    }
}

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
    pub data_pins: Vec<IoPin>,
    data_pins_mode: Mode,
}

impl GpioReader {
    fn new() -> Result<GpioReader, rppal::gpio::Error> {
        let gpio = Gpio::new()?;
        let latch_pin = gpio.get(LATCH_PIN_ID)?.into_output();
        let clock_pin = gpio.get(CLOCK_PIN_ID)?.into_output();
        let data_pin = gpio.get(DATA_PIN_ID)?.into_output();
        let read_pin = gpio.get(READ_PIN_ID)?.into_output();
        let write_pin = gpio.get(WRITE_PIN_ID)?.into_output();
        let data_pins_mode = Mode::Input;
        let data_pins = DATA_PINS.iter().map(|pin| gpio.get(*pin).unwrap().into_io(data_pins_mode)).collect();
        Ok(GpioReader {
            gpio,
            latch_pin,
            clock_pin,
            data_pin,
            read_pin,
            write_pin,
            data_pins,
            data_pins_mode,
        })
    }

    fn change_data_pins_mode(&mut self, mode: Mode) {
        if self.data_pins_mode == mode {
            return;
        }
        if mode == Mode::Input {
            for pin in self.data_pins.iter_mut() {
                pin.set_low()
            }
        }
        for pin in self.data_pins.iter_mut() {
            pin.set_mode(mode);
        }
        self.data_pins_mode = mode;
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
    for address in start_address..=(start_address + 0x3FFF) {
        write_address(gpio, address);
        sleep(Duration::from_micros(SLEEP));
        let mut value = 0u8;
        for (bit, pin) in gpio.data_pins.iter().enumerate() {
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
    gpio.change_data_pins_mode(Mode::Output);
    write_address(gpio, 0x2100);
    sleep(Duration::from_micros(SLEEP));
    for (index, mut pin) in gpio.data_pins.iter_mut().enumerate() {
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
    gpio.change_data_pins_mode(Mode::Input);
}

fn write_address(gpio: &mut GpioReader, address: u16) {
    gpio.latch_pin.set_low();
    shift_out(gpio, ((address >> 8) as u8));
    shift_out(gpio, ((address & 0xFF) as u8));
    gpio.latch_pin.set_high();
}
