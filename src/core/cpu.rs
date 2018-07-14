use {
  core::{
    memory::Memory,
    instruction::Instruction,
    interrupt::Interrupt,
    registers::Registers,
  },
};

pub struct Cpu {
  pub instructions: [Instruction; 1],
  pub interrupt: Interrupt,
  pub memory: Memory,
  pub registers: Registers,
  pub ticks: u64,
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      instructions: get_instructions(),
      interrupt: Interrupt::new(),
      memory: Memory::new(),
      registers: Registers::new(),
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

