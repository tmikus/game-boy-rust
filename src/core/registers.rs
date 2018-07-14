use {
  std::u16,
};

pub union AFRegister {
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
  pub af: u16,
}

pub union BCRegister {
  pub b: u8,
  pub c: u8,
  pub bc: u16,
}

pub union DERegister {
  pub d: u8,
  pub e: u8,
  pub de: u16,
}

pub union HLRegister {
  pub h: u8,
  pub l: u8,
  pub hl: u16,
}

pub struct Registers {
  pub af: AFRegister,
  pub bc: BCRegister,
  pub de: DERegister,
  pub hl: HLRegister,
  pub sp: u16,
  pub pc: u16,
}

impl Registers {
  pub fn new() -> Registers {
    Registers {
      af: AFRegister { af: 0 },
      bc: BCRegister { bc: 0 },
      de: DERegister { de: 0 },
      hl: HLRegister { hl: 0 },
      sp: 0xFFFE,
      pc: 0x100,
    }
  }
}
