extern crate rppal;

use std::arch::asm;
use std::clone::Clone;
use rppal::gpio::{IoPin, Mode, OutputPin};
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
const DATA_3_ID: u8 = 12;
const DATA_4_ID: u8 = 16;
const DATA_5_ID: u8 = 20;
const DATA_6_ID: u8 = 21;
const DATA_7_ID: u8 = 26;

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
        if (value >> number) & 1 == 1 {
            gpio.data_pin.set_high();
        } else {
            gpio.data_pin.set_low();
        }
        sleep(Duration::from_micros(SLEEP));
        gpio.clock_pin.set_high();
        sleep(Duration::from_micros(SLEEP));
    }
}

struct GpioReader {
    pub gpio: Gpio,
    pub cs_pin: OutputPin,
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
        let cs_pin = gpio.get(CS_PIN_ID)?.into_output();
        let latch_pin = gpio.get(LATCH_PIN_ID)?.into_output();
        let clock_pin = gpio.get(CLOCK_PIN_ID)?.into_output();
        let data_pin = gpio.get(DATA_PIN_ID)?.into_output();
        let read_pin = gpio.get(READ_PIN_ID)?.into_output();
        let write_pin = gpio.get(WRITE_PIN_ID)?.into_output();
        let data_pins_mode = Mode::Input;
        let data_pins = DATA_PINS.iter().map(|pin| gpio.get(*pin).unwrap().into_io(data_pins_mode)).collect();
        Ok(GpioReader {
            gpio,
            cs_pin,
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
                pin.set_low();
            }
        }
        for pin in self.data_pins.iter_mut() {
            pin.set_mode(mode);
        }
        self.data_pins_mode = mode;
    }
}

pub fn read_cartridge() -> Vec<u8> {
    println!("Start reading");
    let mut gpio = GpioReader::new().unwrap();
    gpio.cs_pin.set_high();
    gpio.read_pin.set_high();
    gpio.write_pin.set_high();
    sleep(Duration::from_micros(SLEEP));
    let mut data: Vec<u8> = Vec::new();
    // Read the data from bank 0
    read_next_rom_bank(&mut gpio, &mut data, 0);
    let mut banks_count = get_banks_per_rom(&data);
    disable_ram(&mut gpio);
    println!("Banks count: {}", banks_count);
    for bank in 1..banks_count {
        println!("Reading bank {}..", bank);
        select_rom_bank(&mut gpio, bank);
        read_next_rom_bank(&mut gpio, &mut data, 0x4000);
    }
    println!("Done");
    return data
}

fn disable_ram(gpio: &mut GpioReader) {
    write_value_to_address(gpio, 0x0000, 0);
}

fn get_banks_per_rom(data: &[u8]) -> u16 {
    println!("Cartridge type: {:#04x}", data[0x0147]);
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
        data.push(read_byte(gpio, address));
    }
}

fn read_byte(gpio: &mut GpioReader, address: u16) -> u8 {
    write_address(gpio, address);
    gpio.cs_pin.set_low();
    gpio.read_pin.set_low();
    sleep(Duration::from_micros(SLEEP));
    let mut value = 0u8;
    for (bit, pin) in gpio.data_pins.iter().enumerate() {
        if pin.read() == Level::High {
            value |= (1 << bit) as u8;
        }
    }
    gpio.read_pin.set_high();
    gpio.cs_pin.set_high();
    value
}

fn select_rom_bank(gpio: &mut GpioReader, bank: u16) {
    write_value_to_address(gpio, 0x2000, bank &0xff);
    write_value_to_address(gpio, 0x3000, (bank >> 8) & 1);
}

fn write_address(gpio: &mut GpioReader, address: u16) {
    gpio.latch_pin.set_low();
    shift_out(gpio, ((address >> 8) as u8));
    shift_out(gpio, ((address & 0xFF) as u8));
    gpio.latch_pin.set_high();
}

fn write_value_to_address(gpio: &mut GpioReader, address: u16, value: u16) {
    write_address(gpio, address);
    gpio.change_data_pins_mode(Mode::Output);
    for (index, mut pin) in gpio.data_pins.iter_mut().enumerate() {
        if value & (1 << index) != 0 {
            pin.set_high()
        } else {
            pin.set_low();
        }
    }
    gpio.cs_pin.set_low();
    gpio.write_pin.set_low();
    sleep(Duration::from_micros(SLEEP));
    gpio.write_pin.set_high();
    gpio.cs_pin.set_high();
    gpio.change_data_pins_mode(Mode::Input);
    sleep(Duration::from_micros(SLEEP));
}
