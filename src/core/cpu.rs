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
  emulator.registers.pc += emulator.cpu.read_next_byte() as u16;
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

// 0x1F
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

// 0x20
fn jr_nz_n(emulator: &mut Emulator) {
  let operand = emulator.cpu.read_next_byte();
  if emulator.registers.is_flag_set(FLAG_ZERO) {
    emulator.cpu.ticks += 8;
  } else {
    emulator.registers.pc += operand as u16;
    // TODO: Debug JUMP
    emulator.cpu.ticks += 12;
  }
}

// 0x21
fn ld_hl_nn(emulator: &mut Emulator) {
  emulator.registers.set_hl(emulator.cpu.read_next_short());
}

// 0x22
fn ldi_hlp_a(emulator: &mut Emulator) {
  let hl_plus_one = emulator.registers.get_hl() + 1;
  emulator.registers.set_hl(hl_plus_one);
  emulator.memory.write_byte(hl_plus_one, emulator.registers.a);
}

// 0x23
fn inc_hl(emulator: &mut Emulator) {
  let hl_plus_one = emulator.registers.get_hl() + 1;
  emulator.registers.set_hl(hl_plus_one);
}

// 0x24
fn inc_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  emulator.registers.h = increment(emulator, h);
}

// 0x25
fn dec_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  emulator.registers.h = decrement(emulator, h);
}

// 0x26
fn ld_h_n(emulator: &mut Emulator) {
  emulator.registers.h = emulator.cpu.read_next_byte();
}

// 0x27
fn daa(emulator: &mut Emulator) {
  let mut a = emulator.registers.a;
  if emulator.registers.is_flag_set(FLAG_NEGATIVE) {
    if emulator.registers.is_flag_set(FLAG_HALF_CARRY) {
      a = (a - 0x06) & 0xFF;
    }
    if emulator.registers.is_flag_set(FLAG_CARRY) {
      a -= 0x60;
    }
  } else {
    if emulator.registers.is_flag_set(FLAG_HALF_CARRY) || (a & 0xF) > 9 {
      a += 0x06;
    }
    if emulator.registers.is_flag_set(FLAG_CARRY) || a > 0x9F {
      a += 0x60;
    }
  }
  emulator.registers.a = a;
  emulator.registers.clear_flag(FLAG_HALF_CARRY);
  if a > 0 {
    emulator.registers.clear_flag(FLAG_ZERO);
  } else {
    emulator.registers.set_flag(FLAG_ZERO);
  }
  if a >= 0x100 {
    emulator.registers.set_flag(FLAG_CARRY);
  }
}

// 0x28
fn jr_z_n(emulator: &mut Emulator) {
  if emulator.registers.is_flag_set(FLAG_ZERO) {
    emulator.registers.pc += emulator.cpu.read_next_byte() as u16;
    // TODO: Debug JMP
    emulator.cpu.ticks += 12;
  } else {
    emulator.cpu.ticks += 8;
  }
}

// 0x29
fn add_hl_hl(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let result = add_short(emulator, hl, hl);
  emulator.registers.set_hl(result);
}

// 0x2A
fn ldi_a_hlp(emulator: &mut Emulator) {
  let hl_plus_one = emulator.registers.get_hl() + 1;
  emulator.registers.set_hl(hl_plus_one);
  emulator.registers.a = emulator.memory.read_byte(hl_plus_one);
}

// 0x2B
fn dec_hl(emulator: &mut Emulator) {
  let hl_minus_one = emulator.registers.get_hl() - 1;
  emulator.registers.set_hl(hl_minus_one);
}

// 0x2C
fn inc_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  emulator.registers.l = increment(emulator, l);
}

// 0x2D
fn dec_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  emulator.registers.l = decrement(emulator, l);
}

// 0x2E
fn ld_l_n(emulator: &mut Emulator) {
  emulator.registers.l = emulator.cpu.read_next_byte();
}

// 0x2F
fn cpl(emulator: &mut Emulator) {
  emulator.registers.a = !emulator.registers.a;
  emulator.registers.set_flag(FLAG_NEGATIVE | FLAG_HALF_CARRY);
}

// 0x30
fn jr_nc_n(emulator: &mut Emulator) {
  if emulator.registers.is_flag_set(FLAG_CARRY) {
    emulator.cpu.ticks += 8;
  } else {
    emulator.registers.pc += emulator.cpu.read_next_byte() as u16;
    // TODO: Debug JUMP
    emulator.cpu.ticks += 12;
  }
}

// 0x31
fn ld_sp_nn(emulator: &mut Emulator) {
  emulator.registers.sp = emulator.cpu.read_next_short();
}

// 0x32
fn ldd_hlp_a(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let a = emulator.registers.a;
  emulator.memory.write_byte(hl, a);
  emulator.registers.set_hl(hl - 1);
}

// 0x33
fn inc_sp(emulator: &mut Emulator) {
  emulator.registers.sp += 1;
}

// 0x34
fn inc_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let value = emulator.memory.read_byte(hl);
  let inc_result = increment(emulator, value);
  emulator.memory.write_byte(hl, inc_result);
}

// 0x35
fn dec_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let value = emulator.memory.read_byte(hl);
  let inc_result = decrement(emulator, value);
  emulator.memory.write_byte(hl, inc_result);
}

// 0x36
fn ld_hlp_n(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let value = emulator.cpu.read_next_byte();
  emulator.memory.write_byte(hl, value);
}

// 0x37
fn scf(emulator: &mut Emulator) {
  emulator.registers.set_flag(FLAG_CARRY);
  emulator.registers.clear_flag(FLAG_NEGATIVE | FLAG_HALF_CARRY);
}

// 0x38
fn jr_c_n(emulator: &mut Emulator) {
  let operand = emulator.cpu.read_next_byte();
  if emulator.registers.is_flag_set(FLAG_CARRY) {
    emulator.registers.pc += operand as u16;
    emulator.cpu.ticks += 12;
  } else {
    emulator.cpu.ticks += 8;
  }
}

// 0x39
fn add_hl_sp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let sp = emulator.registers.sp;
  let result = add_short(emulator, hl, sp);
  emulator.registers.set_hl(result);
}

// 0x3A
fn ldd_a_hlp(emulator: &mut Emulator) {
  let hl_minus_one = emulator.registers.get_hl() - 1;
  emulator.registers.set_hl(hl_minus_one);
  emulator.registers.a = emulator.memory.read_byte(hl_minus_one);
}

// 0x3B
fn dec_sp(emulator: &mut Emulator) {
  emulator.registers.sp -= 1;
}

// 0x3C
fn inc_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  let result = increment(emulator, a);
  emulator.registers.a = result;
}

// 0x3D
fn dec_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  let result = decrement(emulator, a);
  emulator.registers.a = result;
}

// 0x3E
fn ld_a_n(emulator: &mut Emulator) {
  emulator.registers.a = emulator.cpu.read_next_byte();
}

// 0x3F
fn ccf(emulator: &mut Emulator) {
  if emulator.registers.is_flag_set(FLAG_CARRY) {
    emulator.registers.clear_flag(FLAG_CARRY);
  } else {
    emulator.registers.set_flag(FLAG_CARRY);
  }
  emulator.registers.clear_flag(FLAG_NEGATIVE | FLAG_HALF_CARRY);
}

// 0x41
fn ld_b_c(emulator: &mut Emulator) {
  emulator.registers.b = emulator.registers.c;
}

// 0x42
fn ld_b_d(emulator: &mut Emulator) {
  emulator.registers.b = emulator.registers.d;
}

// 0x43
fn ld_b_e(emulator: &mut Emulator) {
  emulator.registers.b = emulator.registers.e;
}

// 0x44
fn ld_b_h(emulator: &mut Emulator) {
  emulator.registers.b = emulator.registers.h;
}

// 0x45
fn ld_b_l(emulator: &mut Emulator) {
  emulator.registers.b = emulator.registers.l;
}

// 0x46
fn ld_b_hlp(emulator: &mut Emulator) {
  emulator.registers.b = emulator.memory.read_byte(emulator.registers.get_hl());
}

// 0x47
fn ld_b_a(emulator: &mut Emulator) {
  emulator.registers.b = emulator.registers.a;
}

// 0x48
fn ld_c_b(emulator: &mut Emulator) {
  emulator.registers.c = emulator.registers.b;
}

// 0x4A
fn ld_c_d(emulator: &mut Emulator) {
  emulator.registers.c = emulator.registers.d;
}

// 0x4B
fn ld_c_e(emulator: &mut Emulator) {
  emulator.registers.c = emulator.registers.e;
}

// 0x4C
fn ld_c_h(emulator: &mut Emulator) {
  emulator.registers.c = emulator.registers.h;
}

// 0x4D
fn ld_c_l(emulator: &mut Emulator) {
  emulator.registers.c = emulator.registers.l;
}

// 0x4E
fn ld_c_hlp(emulator: &mut Emulator) {
  emulator.registers.c = emulator.memory.read_byte(emulator.registers.get_hl());
}

// 0x4F
fn ld_c_a(emulator: &mut Emulator) {
  emulator.registers.c = emulator.registers.a;
}

// 0x50
fn ld_d_b(emulator: &mut Emulator) {
  emulator.registers.d = emulator.registers.b;
}

// 0x51
fn ld_d_c(emulator: &mut Emulator) {
  emulator.registers.d = emulator.registers.c;
}

// 0x53
fn ld_d_e(emulator: &mut Emulator) {
  emulator.registers.d = emulator.registers.e;
}

// 0x54
fn ld_d_h(emulator: &mut Emulator) {
  emulator.registers.d = emulator.registers.h;
}

// 0x55
fn ld_d_l(emulator: &mut Emulator) {
  emulator.registers.d = emulator.registers.l;
}

// 0x56
fn ld_d_hlp(emulator: &mut Emulator) {
  emulator.registers.d = emulator.memory.read_byte(emulator.registers.get_hl());
}

// 0x57
fn ld_d_a(emulator: &mut Emulator) {
  emulator.registers.d = emulator.registers.a;
}

// 0x58
fn ld_e_b(emulator: &mut Emulator) {
  emulator.registers.e = emulator.registers.b;
}

// 0x59
fn ld_e_c(emulator: &mut Emulator) {
  emulator.registers.e = emulator.registers.c;
}

// 0x5A
fn ld_e_d(emulator: &mut Emulator) {
  emulator.registers.e = emulator.registers.d;
}

// 0x5C
fn ld_e_h(emulator: &mut Emulator) {
  emulator.registers.e = emulator.registers.h;
}

// 0x5D
fn ld_e_l(emulator: &mut Emulator) {
  emulator.registers.e = emulator.registers.l;
}

// 0x5E
fn ld_e_hlp(emulator: &mut Emulator) {
  emulator.registers.e = emulator.memory.read_byte(emulator.registers.get_hl());
}

// 0x5F
fn ld_e_a(emulator: &mut Emulator) {
  emulator.registers.e = emulator.registers.a;
}

// 0x60
fn ld_h_b(emulator: &mut Emulator) {
  emulator.registers.h = emulator.registers.b;
}

// 0x61
fn ld_h_c(emulator: &mut Emulator) {
  emulator.registers.h = emulator.registers.c;
}

// 0x62
fn ld_h_d(emulator: &mut Emulator) {
  emulator.registers.h = emulator.registers.d;
}

// 0x63
fn ld_h_e(emulator: &mut Emulator) {
  emulator.registers.h = emulator.registers.e;
}

// 0x65
fn ld_h_l(emulator: &mut Emulator) {
  emulator.registers.h = emulator.registers.l;
}

// 0x66
fn ld_h_hlp(emulator: &mut Emulator) {
  emulator.registers.h = emulator.memory.read_byte(emulator.registers.get_hl());
}

// 0x67
fn ld_h_a(emulator: &mut Emulator) {
  emulator.registers.h = emulator.registers.a;
}

// 0x68
fn ld_l_b(emulator: &mut Emulator) {
  emulator.registers.l = emulator.registers.b;
}

// 0x69
fn ld_l_c(emulator: &mut Emulator) {
  emulator.registers.l = emulator.registers.c;
}

// 0x6A
fn ld_l_d(emulator: &mut Emulator) {
  emulator.registers.l = emulator.registers.d;
}

// 0x6B
fn ld_l_e(emulator: &mut Emulator) {
  emulator.registers.l = emulator.registers.e;
}

// 0x6C
fn ld_l_h(emulator: &mut Emulator) {
  emulator.registers.l = emulator.registers.h;
}

// 0x6E
fn ld_l_hlp(emulator: &mut Emulator) {
  emulator.registers.l = emulator.memory.read_byte(emulator.registers.get_hl());
}

// 0x6F
fn ld_l_a(emulator: &mut Emulator) {
  emulator.registers.l = emulator.registers.a;
}

// 0x70
fn ld_hlp_b(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let b = emulator.registers.b;
  emulator.memory.write_byte(hl, b);
}

// 0x71
fn ld_hlp_c(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let c = emulator.registers.c;
  emulator.memory.write_byte(hl, c);
}

// 0x72
fn ld_hlp_d(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let d = emulator.registers.d;
  emulator.memory.write_byte(hl, d);
}

// 0x73
fn ld_hlp_e(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let e = emulator.registers.e;
  emulator.memory.write_byte(hl, e);
}

// 0x74
fn ld_hlp_h(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let h = emulator.registers.h;
  emulator.memory.write_byte(hl, h);
}

// 0x75
fn ld_hlp_l(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let l = emulator.registers.l;
  emulator.memory.write_byte(hl, l);
}

// 0x76
fn halt(emulator: &mut Emulator) {
  if emulator.interrupt.master != 0 {
    // Halt execution until an interrupt occurs
  } else {
    emulator.registers.pc += 1;
  }
}

// 0x77
fn ld_hlp_a(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let a = emulator.registers.a;
  emulator.memory.write_byte(hl, a);
}

// 0x78
fn ld_a_b(emulator: &mut Emulator) {
  emulator.registers.a = emulator.registers.b;
}

// 0x79
fn ld_a_c(emulator: &mut Emulator) {
  emulator.registers.a = emulator.registers.c;
}

// 0x7A
fn ld_a_d(emulator: &mut Emulator) {
  emulator.registers.a = emulator.registers.d;
}

// 0x7B
fn ld_a_e(emulator: &mut Emulator) {
  emulator.registers.a = emulator.registers.e;
}

// 0x7C
fn ld_a_h(emulator: &mut Emulator) {
  emulator.registers.a = emulator.registers.h;
}

// 0x7D
fn ld_a_l(emulator: &mut Emulator) {
  emulator.registers.a = emulator.registers.l;
}

// 0x7E
fn ld_a_hlp(emulator: &mut Emulator) {
  emulator.registers.a = emulator.memory.read_byte(emulator.registers.get_hl());
}

// 0x80
fn add_a_b(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  let b = emulator.registers.b;
  emulator.registers.a = add_byte(emulator, a, b);
}

// 0x81
fn add_a_c(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  let c = emulator.registers.c;
  emulator.registers.a = add_byte(emulator, a, c);
}

// 0x82
fn add_a_d(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  let d = emulator.registers.d;
  emulator.registers.a = add_byte(emulator, a, d);
}

// 0x83
fn add_a_e(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  let e = emulator.registers.e;
  emulator.registers.a = add_byte(emulator, a, e);
}

// 0x84
fn add_a_h(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  let h = emulator.registers.h;
  emulator.registers.a = add_byte(emulator, a, h);
}

// 0x85
fn add_a_l(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  let l = emulator.registers.l;
  emulator.registers.a = add_byte(emulator, a, l);
}

// 0x86
fn add_a_hlp(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  let value = emulator.memory.read_byte(emulator.registers.get_hl());
  emulator.registers.a = add_byte(emulator, a, value);
}

// 0x87
fn add_a_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  emulator.registers.a = add_byte(emulator, a, a);
}

// 0x88
fn adc_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  add_with_carry(emulator, b);
}

// 0x89
fn adc_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  add_with_carry(emulator, c);
}

// 0x8A
fn adc_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  add_with_carry(emulator, d);
}

// 0x8B
fn adc_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  add_with_carry(emulator, e);
}

// 0x8C
fn adc_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  add_with_carry(emulator, h);
}

// 0x8D
fn adc_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  add_with_carry(emulator, l);
}

// 0x8E
fn adc_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let value = emulator.memory.read_byte(hl);
  add_with_carry(emulator, value);
}

// 0x8F
fn adc_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  add_with_carry(emulator, a);
}

// 0x90
fn sub_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  subtract(emulator, b);
}

// 0x91
fn sub_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  subtract(emulator, c);
}

// 0x92
fn sub_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  subtract(emulator, d);
}

// 0x93
fn sub_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  subtract(emulator, e);
}

// 0x94
fn sub_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  subtract(emulator, h);
}

// 0x95
fn sub_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  subtract(emulator, l);
}

// 0x96
fn sub_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let value = emulator.memory.read_byte(hl);
  subtract(emulator, value);
}

// 0x97
fn sub_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  subtract_with_carry(emulator, a);
}

// 0x98
fn sbc_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  subtract_with_carry(emulator, b);
}

// 0x99
fn sbc_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  subtract_with_carry(emulator, c);
}

// 0x9A
fn sbc_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  subtract_with_carry(emulator, d);
}

// 0x9B
fn sbc_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  subtract_with_carry(emulator, e);
}

// 0x9C
fn sbc_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  subtract_with_carry(emulator, h);
}

// 0x9D
fn sbc_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  subtract_with_carry(emulator, l);
}

// 0x9E
fn sbc_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let value = emulator.memory.read_byte(hl);
  subtract_with_carry(emulator, value);
}

// 0x9F
fn sbc_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  subtract_with_carry(emulator, a);
}

// 0xA0
fn and_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  and(emulator, b);
}

// 0xA1
fn and_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  and(emulator, c);
}

// 0xA2
fn and_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  and(emulator, d);
}

// 0xA3
fn and_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  and(emulator, e);
}

// 0xA4
fn and_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  and(emulator, h);
}

// 0xA5
fn and_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  and(emulator, l);
}

// 0xA6
fn and_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let value = emulator.memory.read_byte(hl);
  and(emulator, value);
}

// 0xA7
fn and_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  and(emulator, a);
}

// 0xA8
fn xor_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  xor(emulator, b);
}

// 0xA9
fn xor_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  xor(emulator, c);
}

// 0xAA
fn xor_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  xor(emulator, d);
}

// 0xAB
fn xor_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  xor(emulator, e);
}

// 0xAC
fn xor_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  xor(emulator, h);
}

// 0xAD
fn xor_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  xor(emulator, l);
}

// 0xAE
fn xor_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let value = emulator.memory.read_byte(hl);
  xor(emulator, value);
}

// 0xAF
fn xor_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  xor(emulator, a);
}

// 0xB0
fn or_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  or(emulator, b);
}

// 0xB1
fn or_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  or(emulator, c);
}

// 0xB2
fn or_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  or(emulator, d);
}

// 0xB3
fn or_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  or(emulator, e);
}

// 0xB4
fn or_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  or(emulator, h);
}

// 0xB5
fn or_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  or(emulator, l);
}

// 0xB6
fn or_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let value = emulator.memory.read_byte(hl);
  or(emulator, value);
}

// 0xB7
fn or_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  or(emulator, a);
}

// 0xB8
fn cp_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  compare(emulator, b);
}

// 0xB9
fn cp_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  compare(emulator, c);
}

// 0xBA
fn cp_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  compare(emulator, d);
}

// 0xBB
fn cp_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  compare(emulator, e);
}

// 0xBC
fn cp_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  compare(emulator, h);
}

// 0xBD
fn cp_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  compare(emulator, l);
}

// 0xBE
fn cp_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let value = emulator.memory.read_byte(hl);
  compare(emulator, value);
}

// 0xBF
fn cp_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  compare(emulator, a);
}

// 0xC0
fn ret_nz(emulator: &mut Emulator) {
  if emulator.registers.is_flag_set(FLAG_ZERO) {
    emulator.cpu.ticks += 8;
  } else {
    emulator.registers.pc = emulator.memory.read_short_from_stack();
    // TODO: Debug JUMP
    emulator.cpu.ticks += 20;
  }
}

// 0xC1
fn pop_bc(emulator: &mut Emulator) {
  emulator.registers.set_bc(emulator.memory.read_short_from_stack());
}

// 0xC2
fn jp_nz_nn(emulator: &mut Emulator) {
  let operand = emulator.cpu.read_next_short();
  if emulator.registers.is_flag_set(FLAG_ZERO) {
    emulator.cpu.ticks += 12;
  } else {
    emulator.registers.pc = operand;
    // TODO: Debug JUMP
    emulator.cpu.ticks += 16;
  }
}

// 0xC3
fn jp_nn(emulator: &mut Emulator) {
  emulator.registers.pc = emulator.cpu.read_next_short();
  // TODO: Debug JUMP
}

// 0xC4
fn call_nz_nn(emulator: &mut Emulator) {
  let operand = emulator.cpu.read_next_short();
  if emulator.registers.is_flag_set(FLAG_ZERO) {
    emulator.cpu.ticks += 12;
  } else {
    emulator.memory.write_short_to_stack(emulator.registers.pc);
    emulator.registers.pc = operand;
    // TODO: Debug JUMP
    emulator.cpu.ticks += 24;
  }
}

// 0xC5
fn push_bc(emulator: &mut Emulator) {
  emulator.memory.write_short_to_stack(emulator.registers.get_bc());
}

// 0xC6
fn add_a_n(emulator: &mut Emulator) {
  let operand = emulator.cpu.read_next_byte();
  let a = emulator.registers.a;
  emulator.registers.a = add_byte(emulator, a, operand);
}

// 0xC7
fn rst_0(emulator: &mut Emulator) {
  emulator.memory.write_short_to_stack(emulator.registers.pc);
  emulator.registers.pc = 0x0000;
}

// 0xC8
fn ret_z(emulator: &mut Emulator) {
  if emulator.registers.is_flag_set(FLAG_ZERO) {
    emulator.registers.pc = emulator.memory.read_short_from_stack();
    emulator.cpu.ticks += 20;
  } else {
    emulator.cpu.ticks += 8;
  }
}

// 0xC9
fn ret(emulator: &mut Emulator) {
  emulator.registers.pc = emulator.memory.read_short_from_stack();
}

// 0xCA
fn jp_z_nn(emulator: &mut Emulator) {
  let operand = emulator.cpu.read_next_short();
  if emulator.registers.is_flag_set(FLAG_ZERO) {
    emulator.registers.pc = operand;
    // TODO: Debug JUMP
    emulator.cpu.ticks += 16;
  } else {
    emulator.cpu.ticks += 12;
  }
}

// 0xCC
fn call_z_nn(emulator: &mut Emulator) {
  let operand = emulator.cpu.read_next_short();
  if emulator.registers.is_flag_set(FLAG_ZERO) {
    emulator.memory.write_short_to_stack(emulator.registers.pc);
    emulator.registers.pc = operand;
    emulator.cpu.ticks += 24;
  } else {
    emulator.cpu.ticks += 12;
  }
}

// 0xCD
fn call_nn(emulator: &mut Emulator) {
  let operand = emulator.cpu.read_next_short();
  emulator.memory.write_short_to_stack(emulator.registers.pc);
  emulator.registers.pc = operand;
}

// 0xCE
fn adc_n(emulator: &mut Emulator) {
  let operand = emulator.cpu.read_next_byte();
  add_with_carry(emulator, operand);
}

// 0xCF
fn rst_08(emulator: &mut Emulator) {
  emulator.memory.write_short_to_stack(emulator.registers.pc);
  emulator.registers.pc = 0x0008;
}

// 0xD0
fn ret_nc(emulator: &mut Emulator) {
  if emulator.registers.is_flag_set(FLAG_CARRY) {
    emulator.cpu.ticks += 8;
  } else {
    emulator.registers.pc = emulator.memory.read_short_from_stack();
    emulator.cpu.ticks += 20;
  }
}

// 0xD1
fn pop_de(emulator: &mut Emulator) {
  emulator.registers.set_de(emulator.memory.read_short_from_stack());
}

// 0xD2
fn jp_nc_nn(emulator: &mut Emulator) {
  let operand = emulator.cpu.read_next_short();
  if emulator.registers.is_flag_set(FLAG_CARRY) {
    emulator.cpu.ticks += 12;
  } else {
    emulator.registers.pc = operand;
    emulator.cpu.ticks += 16;
  }
}

// 0xD4
fn call_nc_nn(emulator: &mut Emulator) {
  let operand = emulator.cpu.read_next_short();
  if emulator.registers.is_flag_set(FLAG_CARRY) {
    emulator.cpu.ticks += 12;
  } else {
    emulator.memory.write_short_to_stack(emulator.registers.pc);
    emulator.registers.pc = operand;
    emulator.cpu.ticks += 24;
  }
}

// 0xD5
fn push_de(emulator: &mut Emulator) {
  emulator.memory.write_short_to_stack(emulator.registers.get_de());
}

// 0xD6
fn sub_n(emulator: &mut Emulator) {
  let operand = emulator.cpu.read_next_byte();
  subtract(emulator, operand);
}

// 0xD7
fn rst_10(emulator: &mut Emulator) {
  emulator.memory.write_short_to_stack(emulator.registers.pc);
  emulator.registers.pc = 0x0010;
}

// 0xD8
fn ret_c(emulator: &mut Emulator) {
  if emulator.registers.is_flag_set(FLAG_CARRY) {
    emulator.registers.pc = emulator.memory.read_short_from_stack();
    emulator.cpu.ticks += 20;
  } else {
    emulator.cpu.ticks += 8;
  }
}

// 0xDA
fn jp_c_nn(emulator: &mut Emulator) {
  let operand = emulator.cpu.read_next_short();
  if emulator.registers.is_flag_set(FLAG_CARRY) {
    emulator.registers.pc = operand;
    // TODO: Debug JUMP
    emulator.cpu.ticks += 16;
  } else {
    emulator.cpu.ticks += 12;
  }
}

// 0xDC
fn call_c_nn(emulator: &mut Emulator) {
  let operand = emulator.cpu.read_next_short();
  if emulator.registers.is_flag_set(FLAG_CARRY) {
    emulator.memory.write_short_to_stack(emulator.registers.pc);
    emulator.registers.pc = operand;
    emulator.cpu.ticks += 24;
  } else {
    emulator.cpu.ticks += 12;
  }
}

// 0xDE
fn sbc_n(emulator: &mut Emulator) {
  let operand = emulator.cpu.read_next_byte();
  subtract_with_carry(emulator, operand);
}

// 0xDF
fn rst_18(emulator: &mut Emulator) {
  emulator.memory.write_short_to_stack(emulator.registers.pc);
  emulator.registers.pc = 0x0018;
}

// 0xE0
fn ld_ff_n_ap(emulator: &mut Emulator) {
  let operand = emulator.cpu.read_next_byte() as u16;
  let a = emulator.registers.a;
  emulator.memory.write_byte(0xFF00 + operand, a);
}

// 0xE1
fn pop_hl(emulator: &mut Emulator) {
  emulator.registers.set_hl(emulator.memory.read_short_from_stack());
}

// 0xE2
fn ld_ff_c_a(emulator: &mut Emulator) {
  emulator.memory.write_byte(
    0xFF00 + emulator.registers.c as u16,
    emulator.registers.a,
  );
}

// 0xE5
fn push_hl(emulator: &mut Emulator) {
  emulator.memory.write_short_to_stack(emulator.registers.get_hl());
}

// 0xE6
fn and_n(emulator: &mut Emulator) {
  let operand = emulator.cpu.read_next_byte();
  emulator.registers.a &= operand;
  emulator.registers.clear_flag(FLAG_CARRY | FLAG_NEGATIVE);
  emulator.registers.set_flag(FLAG_HALF_CARRY);
  if emulator.registers.a != 0 {
    emulator.registers.clear_flag(FLAG_ZERO);
  } else {
    emulator.registers.set_flag(FLAG_ZERO);
  }
}

// 0xE7
fn rst_20(emulator: &mut Emulator) {
  emulator.memory.write_short_to_stack(emulator.registers.pc);
  emulator.registers.pc = 0x0020;
}

// 0xE8
fn add_sp_n(emulator: &mut Emulator) {
  let operand = emulator.cpu.read_next_byte() as u16;
  let result = emulator.registers.sp + operand;
  if result & 0xFFFF0000 != 0 {
    emulator.registers.set_flag(FLAG_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_CARRY);
  }
  emulator.registers.sp = result & 0xFFFF;
  if (emulator.registers.sp & 0x0F) + (operand & 0x0F) > 0x0F {
    emulator.registers.set_flag(FLAG_HALF_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_HALF_CARRY);
  }
  emulator.registers.clear_flag(FLAG_ZERO | FLAG_NEGATIVE);
}

// 0xE9
fn jp_hl(emulator: &mut Emulator) {
  emulator.registers.pc = emulator.registers.get_hl();
  // TODO: Debug JSON
}

// 0xEA
fn ld_nnp_a(emulator: &mut Emulator) {
  let operand = emulator.cpu.read_next_short();
  emulator.memory.write_byte(operand, emulator.registers.a);
}

// 0xEE
fn xor_n(emulator: &mut Emulator) {
  let operand = emulator.cpu.read_next_byte();
  xor(emulator, operand);
}

// 0xEF
fn rst_28(emulator: &mut Emulator) {
  emulator.memory.write_short_to_stack(emulator.registers.pc);
  emulator.registers.pc = 0x0028
}

// 0xF0
fn ld_ff_ap_n(emulator: &mut Emulator) {
  let operand = emulator.cpu.read_next_byte();
  emulator.registers.a = emulator.memory.read_byte(0xFF00 + operand as u16);
}

// 0xF1
fn pop_af(emulator: &mut Emulator) {
  emulator.registers.set_af(emulator.memory.read_short_from_stack());
}

// 0xF2
fn ld_a_ff_c(emulator: &mut Emulator) {
  emulator.registers.a = emulator.memory.read_byte(0xFF00 + emulator.registers.c as u16);
}

// 0xF3
fn di_inst(emulator: &mut Emulator) {
  emulator.interrupt.master = 0;
}

// 0xF5
fn push_af(emulator: &mut Emulator) {
  emulator.memory.write_short_to_stack(emulator.registers.get_af());
}

// 0xF6
fn or_n(emulator: &mut Emulator) {
  let operand = emulator.cpu.read_next_byte();
  or(emulator, operand);
}

// 0xF7
fn rst_30(emulator: &mut Emulator) {
  emulator.memory.write_short_to_stack(emulator.registers.pc);
  emulator.registers.pc = 0x0030;
}

// 0xF8
fn ld_hl_sp_n(emulator: &mut Emulator) {
  let operand = emulator.cpu.read_next_byte();
  let result = emulator.registers.sp + operand as u16;
  if result & 0xFFFF0000 != 0 {
    emulator.registers.set_flag(FLAG_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_CARRY);
  }
  if (emulator.registers.sp & 0x0F) + (operand as u16 & 0x0F) > 0x0F {
    emulator.registers.set_flag(FLAG_HALF_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_HALF_CARRY);
  }
  emulator.registers.clear_flag(FLAG_ZERO | FLAG_NEGATIVE);
  emulator.registers.set_hl(result & 0xFFFF);
}

// 0xF9
fn ld_sp_hl(emulator: &mut Emulator) {
  emulator.registers.sp = emulator.registers.get_hl();
}

// 0xFA
fn ld_a_nnp(emulator: &mut Emulator) {
  let operand = emulator.cpu.read_next_short();
  emulator.registers.a = emulator.memory.read_byte(operand);
}

// 0xFB
fn ei(emulator: &mut Emulator) {
  emulator.interrupt.master = 1;
}

// 0xFE
fn cp_n(emulator: &mut Emulator) {
  let operand = emulator.cpu.read_next_byte();
  emulator.registers.set_flag(FLAG_NEGATIVE);
  if emulator.registers.a == operand {
    emulator.registers.set_flag(FLAG_ZERO);
  } else {
    emulator.registers.clear_flag(FLAG_ZERO);
  }
  if operand > emulator.registers.a {
    emulator.registers.set_flag(FLAG_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_CARRY);
  }
  if (operand & 0x0F) > (emulator.registers.a & 0x0F) {
    emulator.registers.set_flag(FLAG_HALF_CARRY);
  } else {
    emulator.registers.clear_flag(FLAG_HALF_CARRY);
  }
}

// 0xFF
fn rst_38(emulator: &mut Emulator) {
  emulator.memory.write_short_to_stack(emulator.registers.pc);
  emulator.registers.pc = 0x0038;
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
