extern crate rand;

mod core;

use {
  core::{
    emulator::Emulator,
    keys::Keys,
  },
};

fn main() {
  let mut emulator = Emulator::new();
  let mut keys = Keys::new();
  keys.value = 0xF0;
  println!("Combined Keys: {:#08b}", keys.value);
  println!("Keys 1: {:#06b}", keys.get_keys_1());
  println!("Keys 2: {:#06b}", keys.get_keys_2());
  keys.set_keys_1(0xF);
  println!("Keys 1: {:#06b}", keys.get_keys_1());
  println!("Keys 2: {:#06b}", keys.get_keys_2());
  keys.set_keys_2(0x0);
  println!("Keys 1: {:#06b}", keys.get_keys_1());
  println!("Keys 2: {:#06b}", keys.get_keys_2());
}
