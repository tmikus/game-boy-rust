use {
  core::{
    cpu::Cpu,
    gpu::Gpu,
    interrupt::Interrupt,
    memory::Memory,
    registers::Registers,
  },
  glium::Display,
};

pub struct Emulator {
  pub cpu: Cpu,
  pub gpu: Gpu,
  pub interrupt: Interrupt,
  pub memory: Memory,
  pub registers: Registers,
}

impl Emulator {
  pub fn new() -> Emulator {
    Emulator {
      cpu: Cpu::new(),
      gpu: Gpu::new(),
      interrupt: Interrupt::new(),
      memory: Memory::new(),
      registers: Registers::new(),
    }
  }

  pub fn init(&mut self) {
    self.cpu.emulator = self;
    self.gpu.emulator = self;
    self.interrupt.emulator = self;
    self.memory.emulator = self;
  }

  pub fn reset(&mut self) {
    self.cpu.reset();
    self.gpu.reset();
    self.interrupt.reset();
    self.memory.reset();
    self.registers.reset();
  }

  pub fn run_tick(&mut self, display: Display) {
    self.cpu.run_tick();
    self.gpu.run_tick();
    self.interrupt.run_tick(display);
  }
}
