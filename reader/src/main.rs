extern crate rppal;

use {
  rppal::gpio::{ Gpio, Level, Mode },
  std::{ thread, time },
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

const LATCH_PIN_ID: u64 = 22;
const DATA_PIN_ID: u64 = 18;
const CLOCK_PIN_ID: u64 = 32;
const READ_PIN_ID: u64 = 12;
const WRITE_PIN_ID: u64 = 16;

const DATA_0_ID: u64 = 11;
const DATA_1_ID: u64 = 13;
const DATA_2_ID: u64 = 15;
const DATA_3_ID: u64 = 29;
const DATA_4_ID: u64 = 31;
const DATA_5_ID: u64 = 33;
const DATA_6_ID: u64 = 35;
const DATA_7_ID: u64 = 37;

fn main() {
  let mut gpio = Gpio::new().unwrap();
}
