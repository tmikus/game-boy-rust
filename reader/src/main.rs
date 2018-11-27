extern crate rppal;

use {
  rppal::gpio::{ Gpio, Level, Mode },
  std::{
    collections::LinkedList,
    fs,
    thread::sleep,
    time::Duration,
  },
};

struct ShiftRegister {
  data: usize,
  pins: u8,
}

impl ShiftRegister {
  fn set(&mut self, data: usize) {
    self.data = data;
  }
}

struct Shifter<'a> {
  pub data_pin: u8,
  pub latch_pin: u8,
  pub clock_pin: u8,
  gpio: &'a Gpio,
  shift_registers: LinkedList<ShiftRegister>,
  invert: bool,
}

impl<'a> Shifter<'a> {
  pub fn new(gpio: &'a Gpio, data_pin: u8, latch_pin: u8, clock_pin: u8) -> Shifter {
    Shifter {
      data_pin,
      latch_pin,
      clock_pin,
      gpio,
      shift_registers: LinkedList::new(),
      invert: false,
    }
  }

  pub fn add(&mut self, pins: u8) -> usize {
    let register = ShiftRegister { data: 0, pins };
    self.shift_registers.push_back(register);
    self.shift_registers.len() - 1
  }

  pub fn set(&mut self, register_index: usize, data: usize) {
    for (index, register) in self.shift_registers.iter_mut().enumerate() {
      if index == register_index {
        register.set(data);
        break;
      }
    }
  }

  pub fn invert(&mut self) {
    self.invert = !self.invert;
  }
  
  pub fn write(&mut self) {
    self.gpio.write(self.latch_pin, Level::Low);
    for register in self.shift_registers.iter() {
      for number in 0..register.pins {
        self.gpio.write(self.clock_pin, Level::Low);
        if self.invert {
          self.gpio.write(self.data_pin, match register.data >> number & 1 {
            1 => Level::Low,
            0 => Level::High,
            _ => unreachable!(),
          });
        } else {
          self.gpio.write(self.data_pin, match register.data >> number & 1 {
            1 => Level::High,
            0 => Level::Low,
            _ => unreachable!(),
          });
        }
        self.gpio.write(self.clock_pin, Level::High);
      }
    }
    self.gpio.write(self.latch_pin, Level::High);
  }
}

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
  let data_pins = [
    DATA_0_ID,
    DATA_1_ID,
    DATA_2_ID,
    DATA_3_ID,
    DATA_4_ID,
    DATA_5_ID,
    DATA_6_ID,
    DATA_7_ID,
  ];
  for pin in data_pins.iter() {
    gpio.set_mode(pin.clone(), Mode::Input);
  }
  // Initialise shifter
  let mut shifter = Shifter::new(&gpio, DATA_PIN_ID, LATCH_PIN_ID, CLOCK_PIN_ID);
  shifter.add(8);
  shifter.add(8);
  gpio.write(READ_PIN_ID, Level::Low);
  gpio.write(WRITE_PIN_ID, Level::High);

  // Read data
  let mut data: Vec<u8> = Vec::new();
  for addr in 0..= 0x3FFF {
    shifter.set(0, addr >> 8);
    shifter.set(1, addr & 0xFF);
    shifter.write();
    sleep(Duration::from_micros(50));
    let mut value = 0u8;
    for (bit, pin) in data_pins.iter().enumerate() {
      if gpio.read(pin.clone()).unwrap() == Level::High {
        value |= 1 << bit;
      }
    }
    data.push(value);
  }
  fs::write("rom.dat", data);
  println!("Done");
}
