use std::arch::asm;
use std::time::Duration;
use rppal::gpio::{Error, Gpio, InputPin, Level, OutputPin};

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
    for _ in 0..50 {
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

fn gpio_err_to_string(error: Error) -> String {
    error.to_string()
}

pub struct CartridgeReader {
    gpio: Gpio,
    latch_pin: OutputPin,
    clock_pin: OutputPin,
    cs_pin: OutputPin,
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
        let mut cs_pin = gpio.get(CS_PIN_ID).unwrap().into_output();
        let data_pin = gpio.get(DATA_PIN_ID).unwrap().into_output();
        let mut read_pin = gpio.get(READ_PIN_ID).unwrap().into_output();
        let mut write_pin = gpio.get(WRITE_PIN_ID).unwrap().into_output();
        cs_pin.set_high();
        read_pin.set_high();
        write_pin.set_high();
        CartridgeReader {
            gpio,
            latch_pin,
            clock_pin,
            cs_pin,
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
        self.cs_pin.set_low();
        self.read_pin.set_low();
        sleep(Duration::from_micros(SLEEP));
        let mut value = 0u8;
        for (bit, pin) in data_pins.iter().enumerate() {
            if pin.read() == Level::High {
                value |= (1 << bit) as u8;
            }
        }
        self.read_pin.set_high();
        self.cs_pin.set_high();
        value
    }

    pub fn read_byte_from_bank(&mut self, bank: u16, address: u16) -> u8 {
        self.select_rom_bank(bank);
        self.read_byte(address)
    }

    fn select_rom_bank(&mut self, bank: u16) {
        if self.current_rom_bank == bank {
            return;
        }
        self.current_rom_bank = bank;
        self.write_value_to_address(0x2000, bank &0xff);
        self.write_value_to_address(0x3000, (bank >> 8) & 1);
    }

    fn shift_out(&mut self, value: u8) {
        for number in (0..8).rev() {
            self.clock_pin.set_low();
            sleep(Duration::from_micros(SLEEP));
            self.data_pin.write(match (value >> number) & 1 {
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

    fn write_value_to_address(&mut self, address: u16, value: u16) {
        self.write_address(address);
        for (index, mut pin) in self.get_output_data_pins().into_iter().enumerate() {
            if value & (1 << index) != 0 {
                pin.set_high()
            } else {
                pin.set_low();
            }
        }
        self.cs_pin.set_low();
        self.write_pin.set_low();
        sleep(Duration::from_micros(SLEEP));
        self.write_pin.set_high();
        self.cs_pin.set_high();
        sleep(Duration::from_micros(SLEEP));
    }
}
