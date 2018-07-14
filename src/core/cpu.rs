use {
  core::{
    memory::Memory,
    instruction::Instruction,
    registers::Registers,
  },
};

pub struct Cpu {
  instructions: [Instruction; 1],
  memory: Memory,
  registers: Registers,
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      instructions: get_instructions(),
      memory: Memory::new(),
      registers: Registers::new(),
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

