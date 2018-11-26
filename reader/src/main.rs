extern crate cupi;
extern crate cupi_shift;

use {
  cupi::CuPi,
  cupi_shift::Shifter,
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

const LATCH_PIN_ID: usize = 22;
const DATA_PIN_ID: usize = 18;
const CLOCK_PIN_ID: usize = 32;
const READ_PIN_ID: usize = 12;
const WRITE_PIN_ID: usize = 16;

const DATA_0_ID: usize = 11;
const DATA_1_ID: usize = 13;
const DATA_2_ID: usize = 15;
const DATA_3_ID: usize = 29;
const DATA_4_ID: usize = 31;
const DATA_5_ID: usize = 33;
const DATA_6_ID: usize = 35;
const DATA_7_ID: usize = 37;

fn main() {
  let cupi = CuPi::new().unwrap();
  let mut read_pin = cupi.pin(READ_PIN_ID).unwrap().output();
  let mut write_pin = cupi.pin(WRITE_PIN_ID).unwrap().output();
  let mut shifter = Shifter::new(DATA_PIN_ID, LATCH_PIN_ID, CLOCK_PIN_ID);
  let mut data_pins = [
    cupi.pin(DATA_0_ID).unwrap().input(),
    cupi.pin(DATA_1_ID).unwrap().input(),
    cupi.pin(DATA_2_ID).unwrap().input(),
    cupi.pin(DATA_3_ID).unwrap().input(),
    cupi.pin(DATA_4_ID).unwrap().input(),
    cupi.pin(DATA_5_ID).unwrap().input(),
    cupi.pin(DATA_6_ID).unwrap().input(),
    cupi.pin(DATA_7_ID).unwrap().input(),
  ];
}
