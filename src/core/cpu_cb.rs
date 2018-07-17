use {
  core::{
    emulator::Emulator,
    registers::{ FLAG_CARRY, FLAG_HALF_CARRY, FLAG_NEGATIVE, FLAG_ZERO }
  },
};

pub fn cpu_cb_n(emulator: &mut Emulator) {
  let instruction = emulator.cpu.read_next_byte();
  // TODO: Implement
}

fn rlc(emulator: &mut Emulator, value: u8) -> u8 {
  let carry = (value & 0x80) >> 7;
  if value & 0x80 != 0 {
    emulator.registers.set_flag(FLAG_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_CARRY);
  }
  let result = (value << 1) + carry;
  if result != 0 {
    emulator.registers.clear_flag(FLAG_ZERO);
  } else {
    emulator.registers.set_flag(FLAG_ZERO);
  }
  emulator.registers.clear_flag(FLAG_NEGATIVE | FLAG_HALF_CARRY);
  result
}

fn rrc(emulator: &mut Emulator, value: u8) -> u8 {
  let carry = value & 0x01;
  let mut result = value >> 1;
  if carry != 0 {
    emulator.registers.set_flag(FLAG_CARRY);
    result |= 0x80;
  } else {
    emulator.registers.clear_flag(FLAG_CARRY);
  }
  if result != 0 {
    emulator.registers.clear_flag(FLAG_ZERO);
  } else {
    emulator.registers.set_flag(FLAG_ZERO);
  }
  emulator.registers.clear_flag(FLAG_NEGATIVE | FLAG_HALF_CARRY);
  result
}

fn rl(emulator: &mut Emulator, value: u8) -> u8 {
  let carry = if emulator.registers.is_flag_set(FLAG_CARRY) {
    1
  } else {
    0
  };
  if value & 0x80 != 0 {
    emulator.registers.set_flag(FLAG_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_CARRY);
  }
  let result = (value << 1) + carry;
  if result != 0 {
    emulator.registers.clear_flag(FLAG_ZERO);
  } else {
    emulator.registers.set_flag(FLAG_ZERO);
  }
  emulator.registers.clear_flag(FLAG_NEGATIVE | FLAG_HALF_CARRY);
  result
}

fn rr(emulator: &mut Emulator, value: u8) -> u8 {
  let mut result = value >> 1;
  if emulator.registers.is_flag_set(FLAG_CARRY) {
    result |= 0x80;
  }
  if result & 0x01 != 0 {
    emulator.registers.set_flag(FLAG_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_CARRY);
  }
  if result != 0 {
    emulator.registers.clear_flag(FLAG_ZERO);
  } else {
    emulator.registers.set_flag(FLAG_ZERO);
  }
  result
}

fn sla(emulator: &mut Emulator, value: u8) -> u8 {
  if value & 0x80 != 0 {
    emulator.registers.set_flag(FLAG_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_CARRY);
  }
  let result = value << 0;
  if result != 0 {
    emulator.registers.clear_flag(FLAG_ZERO);
  } else {
    emulator.registers.set_flag(FLAG_ZERO);
  }
  emulator.registers.clear_flag(FLAG_NEGATIVE | FLAG_HALF_CARRY);
  result
}

fn sra(emulator: &mut Emulator, value: u8) -> u8 {
  if value & 0x01 != 0 {
    emulator.registers.set_flag(FLAG_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_CARRY);
  }
  let result = (value & 0x80) | (value >> 1);
  if result != 0 {
    emulator.registers.clear_flag(FLAG_ZERO);
  } else {
    emulator.registers.set_flag(FLAG_ZERO);
  }
  emulator.registers.clear_flag(FLAG_NEGATIVE | FLAG_HALF_CARRY);
  result
}

fn swap(emulator: &mut Emulator, value: u8) -> u8 {
  let result = (value & 0xF) << 4 | (value & 0xF0) >> 4;
  if result != 0 {
    emulator.registers.clear_flag(FLAG_ZERO);
  } else {
    emulator.registers.set_flag(FLAG_ZERO);
  }
  emulator.registers.clear_flag(FLAG_NEGATIVE | FLAG_HALF_CARRY | FLAG_CARRY);
  result
}

fn srl(emulator: &mut Emulator, value: u8) -> u8 {
  if value & 0x01 != 0 {
    emulator.registers.set_flag(FLAG_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_CARRY);
  }
  let result = value >> 1;
  if result != 0 {
    emulator.registers.clear_flag(FLAG_ZERO);
  } else {
    emulator.registers.set_flag(FLAG_ZERO);
  }
  emulator.registers.clear_flag(FLAG_NEGATIVE | FLAG_HALF_CARRY);
  result
}

fn bit(emulator: &mut Emulator, bit: u8, value: u8) {
  if (value & bit) != 0 {
    emulator.registers.clear_flag(FLAG_ZERO);
  } else {
    emulator.registers.set_flag(FLAG_ZERO);
  }
  emulator.registers.clear_flag(FLAG_NEGATIVE);
  emulator.registers.set_flag(FLAG_HALF_CARRY);
}

fn set(emulator: &mut Emulator, bit: u8, value: u8) -> u8 {
  value | bit
}
