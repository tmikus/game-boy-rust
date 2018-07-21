use {
  core::{
    colour::Colour,
    cpu::Cpu,
    gpu::Gpu,
    interrupt::Interrupt,
    memory::Memory,
    registers::Registers,
  },
};

pub struct Emulator {
  pub cpu: Cpu,
  pub gpu: Gpu,
  pub interrupt: Interrupt,
  pub memory: Memory,
  pub palette: [Colour; 4],
  pub registers: Registers,
}

impl Emulator {
  pub fn new() -> Emulator {
    let mut emulator = Emulator {
      cpu: Cpu::new(),
      gpu: Gpu::new(),
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

  pub fn main_loop(&mut self) {
    loop {
      self.cpu.run_next();
      self.gpu.next_tick();
      self.interrupt.step();
    }
  }
}
