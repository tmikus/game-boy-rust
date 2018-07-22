use {
  std::u16,
};

pub const FLAG_ZERO: u8 = 1 << 7;
pub const FLAG_NEGATIVE: u8 = 1 << 6;
pub const FLAG_HALF_CARRY: u8 = 1 << 5;
pub const FLAG_CARRY: u8 = 1 << 4;

pub struct Registers {
  pub a: u8,
  // This is the flag register. Flag register consists of following bits:
  // 7 6 5 4 3 2 1 0
  // Z N H C 0 0 0 0
  //
  // Z - Zero flag: This bit is set when the result of a math operation is zero or two value
  //     match when using the CP instruction
  //
  // N - Subtract flag: This bit is set if a subtraction was performed in the last math instruction.
  //
  // H - This bit is set if a carry occurred from the lower nibble in the last math operation.
  //
  // C - Carry flag: This bit is set if a carry occurred from the last math operation or if
  //     register A is the smaller value when executing the CP instruction.
  pub f: u8,
  pub b: u8,
  pub c: u8,
  pub d: u8,
  pub e: u8,
  pub h: u8,
  pub l: u8,
  pub sp: u16,
  pub pc: u16,
}

impl Registers {
  pub fn new() -> Registers {
    Registers {
      a: 0x11,
      f: 0x80,
      b: 0x00,
      c: 0x00,
      d: 0xFF,
      e: 0x56,
      h: 0x00,
      l: 0x0D,
      sp: 0xfffe,
      pc: 0x100,
    }
  }

  pub fn print_registers(&self) {
    println!(
      "AF={:04X} BC={:04X} DE={:04X} HL={:04X} SP={:04X} PC={:04X}",
      self.get_af(),
      self.get_bc(),
      self.get_de(),
      self.get_hl(),
      self.sp,
      self.pc,
    );
  }

  pub fn reset(&mut self) {
    self.a = 0x11;
    self.f = 0x80;
    self.b = 0x00;
    self.c = 0x00;
    self.d = 0xFF;
    self.e = 0x56;
    self.h = 0x00;
    self.l = 0x0D;
    self.sp = 0xfffe;
    self.pc = 0x100;
  }

  pub fn get_af(&self) -> u16 {
    ((self.a as u16) << 8) | self.f as u16
  }

  pub fn get_bc(&self) -> u16 {
    ((self.b as u16) << 8) | self.c as u16
  }

  pub fn get_de(&self) -> u16 {
    ((self.d as u16) << 8) | self.e as u16
  }

  pub fn get_hl(&self) -> u16 {
    ((self.h as u16) << 8) | self.l as u16
  }

  pub fn set_af(&mut self, value: u16) {
    self.a = ((value & 0xFF00) >> 8) as u8;
    self.f = (value & 0x00FF) as u8;
  }

  pub fn set_bc(&mut self, value: u16) {
    self.b = ((value & 0xFF00) >> 8) as u8;
    self.c = (value & 0x00FF) as u8;
  }

  pub fn set_de(&mut self, value: u16) {
    self.d = ((value & 0xFF00) >> 8) as u8;
    self.e = (value & 0x00FF) as u8;
  }

  pub fn set_hl(&mut self, value: u16) {
    self.h = ((value & 0xFF00) >> 8) as u8;
    self.l = (value & 0x00FF) as u8;
  }

  pub fn clear_flag(&mut self, flag: u8) {
    self.f &= !flag;
  }

  pub fn is_flag_set(&self, flag: u8) -> bool {
    (self.f & flag) != 0
  }

  pub fn set_flag(&mut self, flag: u8) {
    self.f |= flag;
  }
}
