pub union AFRegister {
  pub a: i8,
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
  pub f: i8,
  pub af: i16,
}

pub union BCRegister {
  pub b: i8,
  pub c: i8,
  pub bc: i16,
}

pub union DERegister {
  pub d: i8,
  pub e: i8,
  pub de: i16,
}

pub union HLRegister {
  pub h: i8,
  pub l: i8,
  pub hl: i16,
}

pub struct Registers {
  pub af: AFRegister,
  pub bc: BCRegister,
  pub de: DERegister,
  pub hl: HLRegister,
  pub sp: i16,
  pub pc: i16,
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
