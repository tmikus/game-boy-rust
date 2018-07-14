use {
  core::cpu::Cpu,
};

const INTERRUPT_VBLANK: u8 = 1 << 0;
const INTERRUPT_LCDSTAT: u8 = 1 << 1;
const INTERRUPT_TIMER: u8 = 1 << 2;
const INTERRUPT_SERIAL: u8 = 1 << 3;
const INTERRUPT_JOYPAD: u8 = 1 << 4;

pub struct Interrupt {
  master: u8,
  enable: u8,
  flags: u8,
}

impl Interrupt {
  pub fn new() -> Interrupt {
    Interrupt {
      master: 0,
      enable: 0,
      flags: 0,
    }
  }

  pub fn step(&mut self, cpu: &mut Cpu) {
    if self.master == 0 || self.enable == 0 || self.flags == 0 {
      return;
    }
    let enabled_flags = self.enable & self.flags;
    if (enabled_flags & INTERRUPT_VBLANK) != 0 {
      self.flags &= !INTERRUPT_VBLANK;
      self.vblank(cpu);
    }
    if (enabled_flags & INTERRUPT_LCDSTAT) != 0 {
      self.flags &= !INTERRUPT_LCDSTAT;
      self.lcd_stat(cpu);
    }
    if (enabled_flags & INTERRUPT_TIMER) != 0 {
      self.flags &= !INTERRUPT_TIMER;
      self.timer(cpu);
    }
    if (enabled_flags & INTERRUPT_SERIAL) != 0 {
      self.flags &= !INTERRUPT_SERIAL;
      self.serial(cpu);
    }
    if (enabled_flags & INTERRUPT_JOYPAD) != 0 {
      self.flags &= !INTERRUPT_JOYPAD;
      self.joypad(cpu);
    }
  }

  fn lcd_stat(&mut self, cpu: &mut Cpu) {
    self.master = 0;
    let pc = cpu.registers.pc;
    cpu.memory.write_short_to_stack(cpu, pc);
    cpu.registers.pc = 0x48;
    cpu.ticks += 12;
  }

  fn joypad(&mut self, cpu: &mut Cpu) {
    self.master = 0;
    let pc = cpu.registers.pc;
    cpu.memory.write_short_to_stack(cpu, pc);
    cpu.registers.pc = 0x60;
    cpu.ticks += 12;
  }

  pub fn return_from_interrupt(&mut self, cpu: &mut Cpu) {
    self.master = 1;
    cpu.registers.pc = cpu.memory.read_short_from_stack(cpu)
  }

  fn serial(&mut self, cpu: &mut Cpu) {
    self.master = 0;
    let pc = cpu.registers.pc;
    cpu.memory.write_short_to_stack(cpu, pc);
    cpu.registers.pc = 0x58;
    cpu.ticks += 12;
  }

  fn timer(&mut self, cpu: &mut Cpu) {
    self.master = 0;
    let pc = cpu.registers.pc;
    cpu.memory.write_short_to_stack(cpu, pc);
    cpu.registers.pc = 0x50;
    cpu.ticks += 12;
  }

  fn vblank(&mut self, cpu: &mut Cpu) {
    // TODO: Implement the `drawFramebuffer()` or `VIDEO_WaitVSync()`
    self.master = 0;
    let pc = cpu.registers.pc;
    cpu.memory.write_short_to_stack(cpu, pc);
    cpu.registers.pc = 0x40;
    cpu.ticks += 12;
  }
}
