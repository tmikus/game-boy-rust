use std::thread::sleep;
use std::time::Duration;
use rppal::gpio::{Error, Gpio, InputPin, Level, OutputPin};
use crate::StrResult;

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

fn gpio_err_to_string(error: Error) -> String {
    error.to_string()
}

pub struct CartridgeReader {
    gpio: Gpio,
    latch_pin: OutputPin,
    clock_pin: OutputPin,
    data_pin: OutputPin,
    read_pin: OutputPin,
    write_pin: OutputPin,

    current_rom_bank: u16,
}

impl CartridgeReader {
    pub fn new() -> CartridgeReader {
        let gpio = Gpio::new().unwrap();
        let latch_pin = gpio.get(LATCH_PIN_ID).unwrap().into_output();
        let clock_pin = gpio.get(CLOCK_PIN_ID).unwrap().into_output();
        let data_pin = gpio.get(DATA_PIN_ID).unwrap().into_output();
        let read_pin = gpio.get(READ_PIN_ID).unwrap().into_output();
        let write_pin = gpio.get(WRITE_PIN_ID).unwrap().into_output();
        CartridgeReader {
            gpio,
            latch_pin,
            clock_pin,
            data_pin,
            read_pin,
            write_pin,
            current_rom_bank: 0,
        }
    }

    fn get_input_data_pins(&self) -> Vec<InputPin> {
        DATA_PINS.iter().map(|pin| self.gpio.get(*pin).unwrap().into_input()).collect()
    }

    fn get_output_data_pins(&self) -> Vec<OutputPin> {
        DATA_PINS.iter().map(|pin| self.gpio.get(*pin).unwrap().into_output()).collect()
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        let data_pins = self.get_input_data_pins();
        self.write_address(address);
        let mut value = 0u8;
        for (bit, pin) in data_pins.iter().enumerate() {
            if pin.read() == Level::High {
                value |= (1 << bit) as u8;
            }
        }
        value
    }

    pub fn read_byte_from_bank(&mut self, bank: u16, address: u16) -> u8 {
        self.select_rom_bank(bank);
        self.read_byte(address)
    }

    fn select_rom_bank(&mut self, bank: u16) {
        if self.current_rom_bank == bank {
            return
        }
        self.current_rom_bank = bank;
        self.read_pin.write(Level::High);
        self.write_pin.write(Level::Low);
        sleep(Duration::from_micros(SLEEP));
        let mut output_pins = self.get_output_data_pins();
        self.write_address(0x2100);
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
        self.read_pin.write(Level::Low);
        self.write_pin.write(Level::High);
        for mut pin in output_pins {
            pin.write(Level::Low);
        }
    }

    fn shift_out(&mut self, value: u8) {
        for number in (0..8).rev() {
            self.clock_pin.set_low();
            // sleep(Duration::from_micros(SLEEP));
            self.data_pin.write(match value >> number & 1 {
                1 => Level::High,
                0 => Level::Low,
                _ => unreachable!(),
            });
            sleep(Duration::from_micros(SLEEP));
            self.clock_pin.set_high();
            sleep(Duration::from_micros(SLEEP));
        }
    }

    fn write_address(&mut self, address: u16) {
        self.latch_pin.set_low();
        self.shift_out(((address >> 8) as u8));
        self.shift_out(((address & 0xFF) as u8));
        self.latch_pin.set_high();
    }
}
