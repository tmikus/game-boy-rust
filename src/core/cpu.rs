use {
  core::{
    emulator::Emulator,
    instruction::Instruction,
    registers::{
      FLAG_CARRY,
      FLAG_HALF_CARRY,
      FLAG_NEGATIVE,
      FLAG_ZERO,
    },
  },
  std::ptr,
};

pub struct Cpu {
  pub emulator: *mut Emulator,
  pub instructions: [Instruction; 3],
  pub stopped: bool,
  pub ticks: u64,
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      emulator: ptr::null_mut(),
      instructions: get_instructions(),
      stopped: false,
      ticks: 0,
    }
  }

  pub fn read_next_byte(&mut self) -> u8 {
    let emulator = unsafe { &mut *self.emulator };
    let value = emulator.memory.read_byte(emulator.registers.pc);
    emulator.registers.pc += 1;
    value
  }

  pub fn read_next_short(&mut self) -> u16 {
    let emulator = unsafe { &mut *self.emulator };
    let value = emulator.memory.read_short(emulator.registers.pc);
    emulator.registers.pc += 2;
    value
  }

  pub fn run_next(&mut self) {

  }


}

fn get_instructions() -> [Instruction; 3] {
  [
    // 0x00
    Instruction::new("NOP", 2, nop),
    // 0x01
    Instruction::new("LD BC, {:06X}", 6, ld_bc_nn),
    // 0x02
    Instruction::new("", 4, ld_bcp_a),
  ]
}

// 0x00
fn nop(emulator: &mut Emulator) {
  // This instruction does nothing ..
}

// 0x01
fn ld_bc_nn(emulator: &mut Emulator) {
  emulator.registers.set_bc(emulator.cpu.read_next_short());
}

// 0x02
fn ld_bcp_a(emulator: &mut Emulator) {
  emulator.memory.write_byte(emulator.registers.get_bc(), emulator.registers.a);
}

// 0x03
fn inc_bc(emulator: &mut Emulator) {
  let bc = emulator.registers.get_bc();
  emulator.registers.set_bc(bc + 1);
}

// 0x04
fn inc_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  emulator.registers.b = increment(emulator, b);
}

// 0x05
fn dec_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  emulator.registers.b = decrement(emulator, b);
}

// 0x06
fn ld_b_n(emulator: &mut Emulator) {
  emulator.registers.b = emulator.cpu.read_next_byte();
}

// 0x07
fn rlca(emulator: &mut Emulator) {
  let carry = (emulator.registers.a & 0x80) >> 7;
  if carry != 0 {
    emulator.registers.set_flag(FLAG_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_CARRY);
  }
  emulator.registers.a <<= 1;
  emulator.registers.a += 1;
  emulator.registers.clear_flag(FLAG_NEGATIVE | FLAG_ZERO | FLAG_HALF_CARRY);
}

// 0x08
fn ld_nnp_sp(emulator: &mut Emulator) {
  emulator.memory.write_short(emulator.cpu.read_next_short(), emulator.registers.sp);
}

// 0x09
fn add_hl_bc(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let bc = emulator.registers.get_bc();
  let result = add_short(emulator, hl, bc);
  emulator.registers.set_hl(result);
}

// 0x0A
fn ld_a_bcp(emulator: &mut Emulator) {
  emulator.registers.a = emulator.memory.read_byte(emulator.registers.get_bc());
}

// 0x0B
fn dec_bc(emulator: &mut Emulator) {
  let bc = emulator.registers.get_bc();
  emulator.registers.set_bc(bc - 1);
}

// 0x0C
fn inc_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  emulator.registers.c = increment(emulator, c);
}

// 0x0D
fn dec_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  emulator.registers.c = decrement(emulator, c);
}

// 0x0E
fn ld_c_n(emulator: &mut Emulator) {
  emulator.registers.c = emulator.cpu.read_next_byte();
}

// 0x0F
fn rrca(emulator: &mut Emulator) {
  let carry = emulator.registers.a & 0x01;
  if carry != 0 {
    emulator.registers.set_flag(FLAG_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_CARRY);
  }
  emulator.registers.a >>= 1;
  if carry != 0 {
    emulator.registers.a |= 0x80;
  }
  emulator.registers.clear_flag(FLAG_NEGATIVE | FLAG_ZERO | FLAG_HALF_CARRY);
}

// 0x10
fn stop(emulator: &mut Emulator) {
  emulator.cpu.read_next_byte();
  emulator.cpu.stopped = true;
}

// 0x11
fn ld_de_nn(emulator: &mut Emulator) {
  emulator.registers.set_de(emulator.cpu.read_next_short());
}

// 0x12
fn ld_dep_a(emulator: &mut Emulator) {
  emulator.memory.write_byte(emulator.registers.get_de(), emulator.registers.a);
}

// 0x13
fn inc_de(emulator: &mut Emulator) {
  let de = emulator.registers.get_de();
  emulator.registers.set_de(de + 1);
}

// 0x14
fn inc_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  emulator.registers.d = increment(emulator, d);
}

// 0x15
fn dec_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  emulator.registers.d = decrement(emulator, d);
}

// 0x16
fn ld_d_n(emulator: &mut Emulator) {
  emulator.registers.d = emulator.cpu.read_next_byte();
}

// 0x17
fn rla(emulator: &mut Emulator) {
  let is_carry_set = emulator.registers.is_flag_set(FLAG_CARRY);
  if emulator.registers.a & 0x80 != 0 {
    emulator.registers.set_flag(FLAG_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_CARRY);
  }
  emulator.registers.a <<= 1;
  if is_carry_set {
    emulator.registers.a += 1;
  }
  emulator.registers.clear_flag(FLAG_NEGATIVE | FLAG_ZERO | FLAG_HALF_CARRY);
}

// 0x18
fn jr_n(emulator: &mut Emulator) {
  let pc = emulator.registers.pc as i16;
  let result = pc + emulator.cpu.read_next_byte() as i16;
  emulator.registers.pc = result as u16;
  // TODO: Debug JUMP
}

// 0x19
fn add_hl_de(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let de = emulator.registers.get_de();
  let result = add_short(emulator, hl, de);
  emulator.registers.set_hl(result);
}

// 0x1A
fn ld_a_dep(emulator: &mut Emulator) {
  emulator.registers.a = emulator.memory.read_byte(emulator.registers.get_de());
}

// 0x1B
fn dec_de(emulator: &mut Emulator) {
  let de = emulator.registers.get_de();
  emulator.registers.set_de(de - 1);
}

// 0x1C
fn inc_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  emulator.registers.e = increment(emulator, e);
}

// 0x1D
fn dec_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  emulator.registers.e = decrement(emulator, e);
}

// 0x1E
fn ld_e_n(emulator: &mut Emulator) {
  emulator.registers.e = emulator.cpu.read_next_byte();
}

// 0x20
fn rra(emulator: &mut Emulator) {
  let carry = if emulator.registers.is_flag_set(FLAG_CARRY) {
    1 << 7
  } else {
    0
  };
  if emulator.registers.a & 0x01 != 0 {
    emulator.registers.set_flag(FLAG_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_CARRY);
  }
  emulator.registers.a >>= 1;
  emulator.registers.a += carry;
  emulator.registers.clear_flag(FLAG_NEGATIVE | FLAG_ZERO | FLAG_HALF_CARRY);
}

fn decrement(emulator: &mut Emulator, value: u8) -> u8 {
  if (value & 0x0F) != 0 {
    emulator.registers.clear_flag(FLAG_HALF_CARRY);
  } else {
    emulator.registers.set_flag(FLAG_HALF_CARRY);
  }
  let decremented_value = value - 1;
  if decremented_value != 0 {
    emulator.registers.clear_flag(FLAG_ZERO)
  } else {
    emulator.registers.set_flag(FLAG_ZERO);
  }
  emulator.registers.set_flag(FLAG_NEGATIVE);
  decremented_value
}

fn increment(emulator: &mut Emulator, value: u8) -> u8 {
  if (value & 0x0F) == 0x0f {
    emulator.registers.set_flag(FLAG_HALF_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_HALF_CARRY);
  }
  let incremented_value = value + 1;
  if incremented_value != 0 {
    emulator.registers.clear_flag(FLAG_ZERO);
  } else {
    emulator.registers.set_flag(FLAG_ZERO);
  }
  emulator.registers.clear_flag(FLAG_NEGATIVE);
  value
}

fn add_byte(emulator: &mut Emulator, left: u8, right: u8) -> u8 {
  let result = (left as u16) + (right as u16);
  if (result & 0xFF00) != 0 {
    emulator.registers.set_flag(FLAG_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_CARRY);
  }
  let clamped_result = (result & 0xFF) as u8;
  if clamped_result != 0 {
    emulator.registers.clear_flag(FLAG_ZERO);
  } else {
    emulator.registers.set_flag(FLAG_ZERO);
  }
  if (clamped_result & 0x0F) + (right & 0x0F) > 0x0F {
    emulator.registers.set_flag(FLAG_HALF_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_HALF_CARRY);
  }
  emulator.registers.clear_flag(FLAG_NEGATIVE);
  clamped_result
}

fn add_short(emulator: &mut Emulator, left: u16, right: u16) -> u16 {
  let result = (left as u32) + (right as u32);
  if (result & 0xFFFF0000) != 0 {
    emulator.registers.set_flag(FLAG_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_CARRY);
  }
  let clamped_result = (result & 0xFFFF) as u16;
  if (clamped_result & 0x0F) + (right & 0x0F) > 0x0F {
    emulator.registers.set_flag(FLAG_HALF_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_HALF_CARRY);
  }
  emulator.registers.clear_flag(FLAG_NEGATIVE);
  clamped_result
}

fn subtract(emulator: &mut Emulator, value: u8) {
  emulator.registers.set_flag(FLAG_NEGATIVE);
  if value > emulator.registers.a {
    emulator.registers.set_flag(FLAG_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_CARRY);
  }
  if value & 0x0F > emulator.registers.a & 0x0F {
    emulator.registers.set_flag(FLAG_HALF_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_HALF_CARRY);
  }
  emulator.registers.a -= value;

}

fn add_with_carry(emulator: &mut Emulator, value: u8) {
  let value = if emulator.registers.is_flag_set(FLAG_CARRY) {
    value + 1
  } else {
    value
  };
  let result = emulator.registers.a as u16 + value as u16;
  if (result & 0xFF00) != 0 {
    emulator.registers.set_flag(FLAG_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_CARRY);
  }
  if value == emulator.registers.a {
    emulator.registers.set_flag(FLAG_ZERO);
  } else {
    emulator.registers.clear_flag(FLAG_ZERO);
  }
  if (value & 0x0F) + (emulator.registers.a & 0x0F) > 0x0F {
    emulator.registers.set_flag(FLAG_HALF_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_HALF_CARRY);
  }
  emulator.registers.set_flag(FLAG_NEGATIVE);
  emulator.registers.a = (result & 0xFF) as u8;
}

fn subtract_with_carry(emulator: &mut Emulator, value: u8) {
  let value = if emulator.registers.is_flag_set(FLAG_CARRY) {
    value + 1
  } else {
    value
  };
  emulator.registers.set_flag(FLAG_NEGATIVE);
  if value > emulator.registers.a {
    emulator.registers.set_flag(FLAG_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_CARRY);
  }
  if value == emulator.registers.a {
    emulator.registers.set_flag(FLAG_ZERO);
  } else {
    emulator.registers.clear_flag(FLAG_ZERO);
  }
  if value & 0x0F > emulator.registers.a & 0x0F {
    emulator.registers.set_flag(FLAG_HALF_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_HALF_CARRY);
  }
  emulator.registers.a -= value;
}

fn and(emulator: &mut Emulator, value: u8) {
  emulator.registers.a &= value;
  if emulator.registers.a != 0 {
    emulator.registers.clear_flag(FLAG_ZERO);
  } else {
    emulator.registers.set_flag(FLAG_ZERO);
  }
  emulator.registers.clear_flag(FLAG_CARRY | FLAG_NEGATIVE);
  emulator.registers.set_flag(FLAG_HALF_CARRY);
}

fn or(emulator: &mut Emulator, value: u8) {
  emulator.registers.a |= value;
  if emulator.registers.a != 0 {
    emulator.registers.clear_flag(FLAG_ZERO);
  } else {
    emulator.registers.set_flag(FLAG_ZERO);
  }
  emulator.registers.clear_flag(FLAG_CARRY | FLAG_NEGATIVE | FLAG_HALF_CARRY);
}

fn xor(emulator: &mut Emulator, value: u8) {
  emulator.registers.a ^= value;
  if emulator.registers.a != 0 {
    emulator.registers.clear_flag(FLAG_ZERO);
  } else {
    emulator.registers.set_flag(FLAG_ZERO);
  }
  emulator.registers.clear_flag(FLAG_CARRY | FLAG_NEGATIVE | FLAG_HALF_CARRY);
}

fn compare(emulator: &mut Emulator, value: u8) {
  if emulator.registers.a == value {
    emulator.registers.set_flag(FLAG_ZERO);
  } else {
    emulator.registers.clear_flag(FLAG_ZERO);
  }
  if value > emulator.registers.a {
    emulator.registers.set_flag(FLAG_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_CARRY);
  }
  if value & 0x0F > emulator.registers.a & 0x0F {
    emulator.registers.set_flag(FLAG_HALF_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_HALF_CARRY);
  }
  emulator.registers.set_flag(FLAG_NEGATIVE);
}
