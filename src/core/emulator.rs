use {
  core::{
    colour::Colour,
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
  pub palette: [Colour; 4],
  pub registers: Registers,
}

impl Emulator {
  pub fn new() -> Emulator {
    let mut emulator = Emulator {
      cpu: Cpu::new(),
      interrupt: Interrupt::new(),
      memory: Memory::new(),
      palette: [
        // TODO: Fix this
        Colour::new(0, 0, 0),
        Colour::new(0, 0, 0),
        Colour::new(0, 0, 0),
        Colour::new(0, 0, 0),
      ],
      registers: Registers::new(),
    };
    emulator.cpu.emulator = &mut emulator;
    emulator.interrupt.emulator = &mut emulator;
    emulator.memory.emulator = &mut emulator;
    emulator
  }

  pub fn reset(&mut self) {
    self.cpu = Cpu::new();
    self.cpu.emulator = self;
    self.interrupt = Interrupt::new();
    self.interrupt.emulator = self;
    self.memory = Memory::new();
    self.memory.emulator = self;
    self.registers = Registers::new();
  }
}
