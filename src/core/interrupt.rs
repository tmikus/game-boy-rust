use {
  core::emulator::Emulator,
  std::ptr,
};

const INTERRUPT_VBLANK: u8 = 1 << 0;
const INTERRUPT_LCDSTAT: u8 = 1 << 1;
const INTERRUPT_TIMER: u8 = 1 << 2;
const INTERRUPT_SERIAL: u8 = 1 << 3;
const INTERRUPT_JOYPAD: u8 = 1 << 4;

pub struct Interrupt {
  pub emulator: *mut Emulator,
  pub master: u8,
  pub enable: u8,
  pub flags: u8,
}

impl Interrupt {
  pub fn new() -> Interrupt {
    Interrupt {
      emulator: ptr::null_mut(),
      master: 1,
      enable: 0,
      flags: 0,
    }
  }

  pub fn reset(&mut self) {
    self.master = 1;
    self.enable = 0;
    self.flags = 0;
  }

  pub fn step(&mut self) {
    if self.master == 0 || self.enable == 0 || self.flags == 0 {
      return;
    }
    let enabled_flags = self.enable & self.flags;
    if (enabled_flags & INTERRUPT_VBLANK) != 0 {
      self.flags &= !INTERRUPT_VBLANK;
      self.vblank();
    }
    if (enabled_flags & INTERRUPT_LCDSTAT) != 0 {
      self.flags &= !INTERRUPT_LCDSTAT;
      self.lcd_stat();
    }
    if (enabled_flags & INTERRUPT_TIMER) != 0 {
      self.flags &= !INTERRUPT_TIMER;
      self.timer();
    }
    if (enabled_flags & INTERRUPT_SERIAL) != 0 {
      self.flags &= !INTERRUPT_SERIAL;
      self.serial();
    }
    if (enabled_flags & INTERRUPT_JOYPAD) != 0 {
      self.flags &= !INTERRUPT_JOYPAD;
      self.joypad();
    }
  }

  fn lcd_stat(&mut self) {
    self.master = 0;
    let emulator = unsafe { &mut *self.emulator };
    emulator.memory.write_short_to_stack(emulator.registers.pc);
    emulator.registers.pc = 0x48;
    emulator.cpu.ticks += 12;
  }

  fn joypad(&mut self) {
    self.master = 0;
    let emulator = unsafe { &mut *self.emulator };
    emulator.memory.write_short_to_stack(emulator.registers.pc);
    emulator.registers.pc = 0x60;
    emulator.cpu.ticks += 12;
  }

  pub fn return_from_interrupt(&mut self) {
    self.master = 1;
    let emulator = unsafe { &mut *self.emulator };
    emulator.registers.pc = emulator.memory.read_short_from_stack();
  }

  fn serial(&mut self) {
    self.master = 0;
    let emulator = unsafe { &mut *self.emulator };
    emulator.memory.write_short_to_stack(emulator.registers.pc);
    emulator.registers.pc = 0x58;
    emulator.cpu.ticks += 12;
  }

  fn timer(&mut self) {
    self.master = 0;
    let emulator = unsafe { &mut *self.emulator };
    emulator.memory.write_short_to_stack(emulator.registers.pc);
    emulator.registers.pc = 0x50;
    emulator.cpu.ticks += 12;
  }

  fn vblank(&mut self) {
    // TODO: Implement the `drawFramebuffer()` or `VIDEO_WaitVSync()`
    self.master = 0;
    let emulator = unsafe { &mut *self.emulator };
    emulator.memory.write_short_to_stack(emulator.registers.pc);
    emulator.registers.pc = 0x40;
    emulator.cpu.ticks += 12;
  }
}
