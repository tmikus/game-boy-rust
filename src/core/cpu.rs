use {
  core::{
    emulator::Emulator,
    instruction::Instruction,
  },
  std::ptr,
};

pub struct Cpu {
  pub emulator: *mut Emulator,
  pub instructions: [Instruction; 1],
  pub ticks: u64,
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      emulator: ptr::null_mut(),
      instructions: get_instructions(),
      ticks: 0,
    }
  }

  pub fn get_byte(&mut self) -> u8 {
    // TODO: Implement this
    0
  }

  pub fn get_short(&mut self) -> u16 {
    // TODO: Implement this
    0
  }

  pub fn run_next(&mut self) {

  }


}

fn get_instructions() -> [Instruction; 1] {
  [
    Instruction::new("NOP", 2, operation_nop),
  ]
}


fn operation_nop(_cpu: &mut Cpu) {
  // This instruction does nothing ..
}

