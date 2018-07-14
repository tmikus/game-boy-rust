use {
  core::{
    cpu::Cpu,
    interrupt::Interrupt,
    memory::Memory,
    registers::Registers,
  },
  std::ptr,
};

pub struct Emulator {
  pub cpu: Cpu,
  pub interrupt: Interrupt,
  pub memory: Memory,
  pub registers: Registers,
}

impl Emulator {
  pub fn new() -> Emulator {
    let mut emulator = Emulator {
      cpu: Cpu::new(),
      interrupt: Interrupt::new(),
      memory: Memory::new(),
      registers: Registers::new(),
    };
    emulator.cpu.emulator = &mut emulator;
    emulator.interrupt.emulator = &mut emulator;
    emulator.memory.emulator = &mut emulator;
    emulator
  }
}
