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

// 0x00
fn rlc_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  emulator.registers.b = rlc(emulator, b);
}

// 0x01
fn rlc_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  emulator.registers.c = rlc(emulator, c);
}

// 0x02
fn rlc_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  emulator.registers.d = rlc(emulator, d);
}

// 0x03
fn rlc_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  emulator.registers.e = rlc(emulator, e);
}

// 0x04
fn rlc_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  emulator.registers.h = rlc(emulator, h);
}

// 0x05
fn rlc_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  emulator.registers.l = rlc(emulator, l);
}

// 0x06
fn rlc_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  let result = rlc(emulator, hl_value);
  emulator.memory.write_byte(emulator.registers.get_hl(), result);
}

// 0x07
fn rlc_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  emulator.registers.a = rlc(emulator, a);
}

// 0x08
fn rrc_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  emulator.registers.b = rrc(emulator, b);
}

// 0x09
fn rrc_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  emulator.registers.c = rrc(emulator, c);
}

// 0x0A
fn rrc_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  emulator.registers.d = rrc(emulator, d);
}

// 0x0B
fn rrc_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  emulator.registers.e = rrc(emulator, e);
}

// 0x0C
fn rrc_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  emulator.registers.h = rrc(emulator, h);
}

// 0x0D
fn rrc_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  emulator.registers.l = rrc(emulator, l);
}

// 0x0E
fn rrc_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  let result = rrc(emulator, hl_value);
  emulator.memory.write_byte(emulator.registers.get_hl(), result);
}

// 0x0F
fn rrc_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  emulator.registers.a = rrc(emulator, a);
}

// 0x10
fn rl_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  emulator.registers.b = rl(emulator, b);
}

// 0x11
fn rl_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  emulator.registers.c = rl(emulator, c);
}

// 0x12
fn rl_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  emulator.registers.d = rl(emulator, d);
}

// 0x13
fn rl_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  emulator.registers.e = rl(emulator, e);
}

// 0x14
fn rl_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  emulator.registers.h = rl(emulator, h);
}

// 0x15
fn rl_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  emulator.registers.l = rl(emulator, l);
}

// 0x16
fn rl_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  let result = rl(emulator, hl_value);
  emulator.memory.write_byte(emulator.registers.get_hl(), result);
}

// 0x17
fn rl_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  emulator.registers.a = rl(emulator, a);
}

// 0x18
fn rr_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  emulator.registers.b = rr(emulator, b);
}

// 0x19
fn rr_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  emulator.registers.c = rr(emulator, c);
}

// 0x1A
fn rr_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  emulator.registers.d = rr(emulator, d);
}

// 0x1B
fn rr_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  emulator.registers.e = rr(emulator, e);
}

// 0x1C
fn rr_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  emulator.registers.h = rr(emulator, h);
}

// 0x1D
fn rr_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  emulator.registers.l = rr(emulator, l);
}

// 0x1E
fn rr_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  let result = rr(emulator, hl_value);
  emulator.memory.write_byte(emulator.registers.get_hl(), result);
}

// 0x1F
fn rr_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  emulator.registers.a = rr(emulator, a);
}

// 0x20
fn sla_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  emulator.registers.b = sla(emulator, b);
}

// 0x21
fn sla_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  emulator.registers.c = sla(emulator, c);
}

// 0x22
fn sla_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  emulator.registers.d = sla(emulator, d);
}

// 0x23
fn sla_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  emulator.registers.e = sla(emulator, e);
}

// 0x24
fn sla_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  emulator.registers.h = sla(emulator, h);
}

// 0x25
fn sla_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  emulator.registers.l = sla(emulator, l);
}

// 0x26
fn sla_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  let result = sla(emulator, hl_value);
  emulator.memory.write_byte(emulator.registers.get_hl(), result);
}

// 0x27
fn sla_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  emulator.registers.a = sla(emulator, a);
}

// 0x28
fn sra_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  emulator.registers.b = sra(emulator, b);
}

// 0x29
fn sra_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  emulator.registers.c = sra(emulator, c);
}

// 0x2A
fn sra_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  emulator.registers.d = sra(emulator, d);
}

// 0x2B
fn sra_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  emulator.registers.e = sra(emulator, e);
}

// 0x2C
fn sra_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  emulator.registers.h = sra(emulator, h);
}

// 0x2D
fn sra_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  emulator.registers.l = sra(emulator, l);
}

// 0x2E
fn sra_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  let result = sra(emulator, hl_value);
  emulator.memory.write_byte(emulator.registers.get_hl(), result);
}

// 0x2F
fn sra_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  emulator.registers.a = sra(emulator, a);
}

// 0x30
fn swap_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  emulator.registers.b = swap(emulator, b);
}

// 0x31
fn swap_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  emulator.registers.c = swap(emulator, c);
}

// 0x32
fn swap_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  emulator.registers.d = swap(emulator, d);
}

// 0x33
fn swap_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  emulator.registers.e = swap(emulator, e);
}

// 0x34
fn swap_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  emulator.registers.h = swap(emulator, h);
}

// 0x35
fn swap_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  emulator.registers.l = swap(emulator, l);
}

// 0x36
fn swap_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  let result = swap(emulator, hl_value);
  emulator.memory.write_byte(emulator.registers.get_hl(), result);
}

// 0x37
fn swap_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  emulator.registers.a = swap(emulator, a);
}

// 0x38
fn srl_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  emulator.registers.b = srl(emulator, b);
}

// 0x39
fn srl_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  emulator.registers.c = srl(emulator, c);
}

// 0x3A
fn srl_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  emulator.registers.d = srl(emulator, d);
}

// 0x3B
fn srl_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  emulator.registers.e = srl(emulator, e);
}

// 0x3C
fn srl_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  emulator.registers.h = srl(emulator, h);
}

// 0x3D
fn srl_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  emulator.registers.l = srl(emulator, l);
}

// 0x3E
fn srl_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  let result = srl(emulator, hl_value);
  emulator.memory.write_byte(emulator.registers.get_hl(), result);
}

// 0x3F
fn srl_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  emulator.registers.a = srl(emulator, a);
}

// 0x40
fn bit_0_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  bit(emulator, 1 << 0, b);
}

// 0x41
fn bit_0_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  bit(emulator, 1 << 0, c);
}

// 0x42
fn bit_0_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  bit(emulator, 1 << 0, d);
}

// 0x43
fn bit_0_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  bit(emulator, 1 << 0, e);
}

// 0x44
fn bit_0_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  bit(emulator, 1 << 0, h);
}

// 0x45
fn bit_0_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  bit(emulator, 1 << 0, l);
}

// 0x46
fn bit_0_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  bit(emulator, 1 << 0, hl_value);
}

// 0x47
fn bit_0_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  bit(emulator, 1 << 0, a);
}

// 0x48
fn bit_1_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  bit(emulator, 1 << 1, b);
}

// 0x49
fn bit_1_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  bit(emulator, 1 << 1, c);
}

// 0x4A
fn bit_1_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  bit(emulator, 1 << 1, d);
}

// 0x4B
fn bit_1_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  bit(emulator, 1 << 1, e);
}

// 0x4C
fn bit_1_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  bit(emulator, 1 << 1, h);
}

// 0x4D
fn bit_1_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  bit(emulator, 1 << 1, l);
}

// 0x4E
fn bit_1_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  bit(emulator, 1 << 1, hl_value);
}

// 0x4F
fn bit_1_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  bit(emulator, 1 << 1, a);
}

// 0x50
fn bit_2_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  bit(emulator, 1 << 2, b);
}

// 0x51
fn bit_2_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  bit(emulator, 1 << 2, c);
}

// 0x52
fn bit_2_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  bit(emulator, 1 << 2, d);
}

// 0x53
fn bit_2_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  bit(emulator, 1 << 2, e);
}

// 0x54
fn bit_2_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  bit(emulator, 1 << 2, h);
}

// 0x55
fn bit_2_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  bit(emulator, 1 << 2, l);
}

// 0x56
fn bit_2_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  bit(emulator, 1 << 2, hl_value);
}

// 0x57
fn bit_2_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  bit(emulator, 1 << 2, a);
}

// 0x58
fn bit_3_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  bit(emulator, 1 << 3, b);
}

// 0x59
fn bit_3_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  bit(emulator, 1 << 3, c);
}

// 0x5A
fn bit_3_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  bit(emulator, 1 << 3, d);
}

// 0x5B
fn bit_3_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  bit(emulator, 1 << 3, e);
}

// 0x5C
fn bit_3_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  bit(emulator, 1 << 3, h);
}

// 0x5D
fn bit_3_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  bit(emulator, 1 << 3, l);
}

// 0x5E
fn bit_3_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  bit(emulator, 1 << 3, hl_value);
}

// 0x5F
fn bit_3_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  bit(emulator, 1 << 3, a);
}

// 0x60
fn bit_4_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  bit(emulator, 1 << 4, b);
}

// 0x61
fn bit_4_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  bit(emulator, 1 << 4, c);
}

// 0x62
fn bit_4_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  bit(emulator, 1 << 4, d);
}

// 0x63
fn bit_4_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  bit(emulator, 1 << 4, e);
}

// 0x64
fn bit_4_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  bit(emulator, 1 << 4, h);
}

// 0x65
fn bit_4_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  bit(emulator, 1 << 4, l);
}

// 0x66
fn bit_4_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  bit(emulator, 1 << 4, hl_value);
}

// 0x67
fn bit_4_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  bit(emulator, 1 << 4, a);
}

// 0x68
fn bit_5_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  bit(emulator, 1 << 5, b);
}

// 0x69
fn bit_5_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  bit(emulator, 1 << 5, c);
}

// 0x6A
fn bit_5_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  bit(emulator, 1 << 5, d);
}

// 0x6B
fn bit_5_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  bit(emulator, 1 << 5, e);
}

// 0x6C
fn bit_5_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  bit(emulator, 1 << 5, h);
}

// 0x6D
fn bit_5_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  bit(emulator, 1 << 5, l);
}

// 0x6E
fn bit_5_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  bit(emulator, 1 << 5, hl_value);
}

// 0x6F
fn bit_5_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  bit(emulator, 1 << 5, a);
}

// 0x70
fn bit_6_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  bit(emulator, 1 << 6, b);
}

// 0x71
fn bit_6_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  bit(emulator, 1 << 6, c);
}

// 0x72
fn bit_6_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  bit(emulator, 1 << 6, d);
}

// 0x73
fn bit_6_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  bit(emulator, 1 << 6, e);
}

// 0x74
fn bit_6_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  bit(emulator, 1 << 6, h);
}

// 0x75
fn bit_6_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  bit(emulator, 1 << 6, l);
}

// 0x76
fn bit_6_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  bit(emulator, 1 << 6, hl_value);
}

// 0x77
fn bit_6_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  bit(emulator, 1 << 6, a);
}

// 0x78
fn bit_7_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  bit(emulator, 1 << 7, b);
}

// 0x79
fn bit_7_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  bit(emulator, 1 << 7, c);
}

// 0x7A
fn bit_7_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  bit(emulator, 1 << 7, d);
}

// 0x7B
fn bit_7_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  bit(emulator, 1 << 7, e);
}

// 0x7C
fn bit_7_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  bit(emulator, 1 << 7, h);
}

// 0x7D
fn bit_7_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  bit(emulator, 1 << 7, l);
}

// 0x7E
fn bit_7_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  bit(emulator, 1 << 7, hl_value);
}

// 0x7F
fn bit_7_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  bit(emulator, 1 << 7, a);
}

// 0x80
fn res_0_b(emulator: &mut Emulator) {
  emulator.registers.b &= !(1 << 0);
}

// 0x81
fn res_0_c(emulator: &mut Emulator) {
  emulator.registers.c &= !(1 << 0);
}

// 0x82
fn res_0_d(emulator: &mut Emulator) {
  emulator.registers.d &= !(1 << 0);
}

// 0x83
fn res_0_e(emulator: &mut Emulator) {
  emulator.registers.e &= !(1 << 0);
}

// 0x84
fn res_0_h(emulator: &mut Emulator) {
  emulator.registers.h &= !(1 << 0);
}

// 0x85
fn res_0_l(emulator: &mut Emulator) {
  emulator.registers.l &= !(1 << 0);
}

// 0x86
fn res_0_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  let result = hl_value & !(1 << 0);
  emulator.memory.write_byte(hl, result);
}

// 0x87
fn res_0_a(emulator: &mut Emulator) {
  emulator.registers.a &= !(1 << 0);
}

// 0x88
fn res_1_b(emulator: &mut Emulator) {
  emulator.registers.b &= !(1 << 1);
}

// 0x89
fn res_1_c(emulator: &mut Emulator) {
  emulator.registers.c &= !(1 << 1);
}

// 0x8A
fn res_1_d(emulator: &mut Emulator) {
  emulator.registers.d &= !(1 << 1);
}

// 0x8B
fn res_1_e(emulator: &mut Emulator) {
  emulator.registers.e &= !(1 << 1);
}

// 0x8C
fn res_1_h(emulator: &mut Emulator) {
  emulator.registers.h &= !(1 << 1);
}

// 0x8D
fn res_1_l(emulator: &mut Emulator) {
  emulator.registers.l &= !(1 << 1);
}

// 0x8E
fn res_1_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  let result = hl_value & !(1 << 1);
  emulator.memory.write_byte(hl, result);
}

// 0x8F
fn res_1_a(emulator: &mut Emulator) {
  emulator.registers.a &= !(1 << 1);
}

// 0x90
fn res_2_b(emulator: &mut Emulator) {
  emulator.registers.b &= !(1 << 2);
}

// 0x91
fn res_2_c(emulator: &mut Emulator) {
  emulator.registers.c &= !(1 << 2);
}

// 0x92
fn res_2_d(emulator: &mut Emulator) {
  emulator.registers.d &= !(1 << 2);
}

// 0x93
fn res_2_e(emulator: &mut Emulator) {
  emulator.registers.e &= !(1 << 2);
}

// 0x94
fn res_2_h(emulator: &mut Emulator) {
  emulator.registers.h &= !(1 << 2);
}

// 0x95
fn res_2_l(emulator: &mut Emulator) {
  emulator.registers.l &= !(1 << 2);
}

// 0x96
fn res_2_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  let result = hl_value & !(1 << 2);
  emulator.memory.write_byte(hl, result);
}

// 0x97
fn res_2_a(emulator: &mut Emulator) {
  emulator.registers.a &= !(1 << 2);
}

// 0x98
fn res_3_b(emulator: &mut Emulator) {
  emulator.registers.b &= !(1 << 3);
}

// 0x99
fn res_3_c(emulator: &mut Emulator) {
  emulator.registers.c &= !(1 << 3);
}

// 0x9A
fn res_3_d(emulator: &mut Emulator) {
  emulator.registers.d &= !(1 << 3);
}

// 0x9B
fn res_3_e(emulator: &mut Emulator) {
  emulator.registers.e &= !(1 << 3);
}

// 0x9C
fn res_3_h(emulator: &mut Emulator) {
  emulator.registers.h &= !(1 << 3);
}

// 0x9D
fn res_3_l(emulator: &mut Emulator) {
  emulator.registers.l &= !(1 << 3);
}

// 0x9E
fn res_3_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  let result = hl_value & !(1 << 3);
  emulator.memory.write_byte(hl, result);
}

// 0x9F
fn res_3_a(emulator: &mut Emulator) {
  emulator.registers.a &= !(1 << 3);
}

// 0xA0
fn res_4_b(emulator: &mut Emulator) {
  emulator.registers.b &= !(1 << 4);
}

// 0xA1
fn res_4_c(emulator: &mut Emulator) {
  emulator.registers.c &= !(1 << 4);
}

// 0xA2
fn res_4_d(emulator: &mut Emulator) {
  emulator.registers.d &= !(1 << 4);
}

// 0xA3
fn res_4_e(emulator: &mut Emulator) {
  emulator.registers.e &= !(1 << 4);
}

// 0xA4
fn res_4_h(emulator: &mut Emulator) {
  emulator.registers.h &= !(1 << 4);
}

// 0xA5
fn res_4_l(emulator: &mut Emulator) {
  emulator.registers.l &= !(1 << 4);
}

// 0xA6
fn res_4_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  let result = hl_value & !(1 << 4);
  emulator.memory.write_byte(hl, result);
}

// 0xA7
fn res_4_a(emulator: &mut Emulator) {
  emulator.registers.a &= !(1 << 4);
}

// 0xA8
fn res_5_b(emulator: &mut Emulator) {
  emulator.registers.b &= !(1 << 5);
}

// 0xA9
fn res_5_c(emulator: &mut Emulator) {
  emulator.registers.c &= !(1 << 5);
}

// 0xAA
fn res_5_d(emulator: &mut Emulator) {
  emulator.registers.d &= !(1 << 5);
}

// 0xAB
fn res_5_e(emulator: &mut Emulator) {
  emulator.registers.e &= !(1 << 5);
}

// 0xAC
fn res_5_h(emulator: &mut Emulator) {
  emulator.registers.h &= !(1 << 5);
}

// 0xAD
fn res_5_l(emulator: &mut Emulator) {
  emulator.registers.l &= !(1 << 5);
}

// 0xAE
fn res_5_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  let result = hl_value & !(1 << 5);
  emulator.memory.write_byte(hl, result);
}

// 0xAF
fn res_5_a(emulator: &mut Emulator) {
  emulator.registers.a &= !(1 << 5);
}

// 0xB0
fn res_6_b(emulator: &mut Emulator) {
  emulator.registers.b &= !(1 << 6);
}

// 0xB1
fn res_6_c(emulator: &mut Emulator) {
  emulator.registers.c &= !(1 << 6);
}

// 0xB2
fn res_6_d(emulator: &mut Emulator) {
  emulator.registers.d &= !(1 << 6);
}

// 0xB3
fn res_6_e(emulator: &mut Emulator) {
  emulator.registers.e &= !(1 << 6);
}

// 0xB4
fn res_6_h(emulator: &mut Emulator) {
  emulator.registers.h &= !(1 << 6);
}

// 0xB5
fn res_6_l(emulator: &mut Emulator) {
  emulator.registers.l &= !(1 << 6);
}

// 0xB6
fn res_6_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  let result = hl_value & !(1 << 6);
  emulator.memory.write_byte(hl, result);
}

// 0xB7
fn res_6_a(emulator: &mut Emulator) {
  emulator.registers.a &= !(1 << 6);
}

// 0xB8
fn res_7_b(emulator: &mut Emulator) {
  emulator.registers.b &= !(1 << 7);
}

// 0xB9
fn res_7_c(emulator: &mut Emulator) {
  emulator.registers.c &= !(1 << 7);
}

// 0xBA
fn res_7_d(emulator: &mut Emulator) {
  emulator.registers.d &= !(1 << 7);
}

// 0xBB
fn res_7_e(emulator: &mut Emulator) {
  emulator.registers.e &= !(1 << 7);
}

// 0xBC
fn res_7_h(emulator: &mut Emulator) {
  emulator.registers.h &= !(1 << 7);
}

// 0xBD
fn res_7_l(emulator: &mut Emulator) {
  emulator.registers.l &= !(1 << 7);
}

// 0xBE
fn res_7_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  let result = hl_value & !(1 << 7);
  emulator.memory.write_byte(hl, result);
}

// 0xBF
fn res_7_a(emulator: &mut Emulator) {
  emulator.registers.a &= !(1 << 7);
}

// 0xC0
fn set_0_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  emulator.registers.b = set(emulator, 1 << 0, b);
}

// 0xC1
fn set_0_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  emulator.registers.c = set(emulator, 1 << 0, c);
}

// 0xC2
fn set_0_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  emulator.registers.d = set(emulator, 1 << 0, d);
}

// 0xC3
fn set_0_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  emulator.registers.e = set(emulator, 1 << 0, e);
}

// 0xC4
fn set_0_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  emulator.registers.h = set(emulator, 1 << 0, h);
}

// 0xC5
fn set_0_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  emulator.registers.l = set(emulator, 1 << 0, l);
}

// 0xC6
fn set_0_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  let result = set(emulator, 1 << 0, hl_value);
  emulator.memory.write_byte(hl, result);
}

// 0xC7
fn set_0_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  emulator.registers.a = set(emulator, 1 << 0, a);
}

// 0xC8
fn set_1_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  emulator.registers.b = set(emulator, 1 << 1, b);
}

// 0xC9
fn set_1_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  emulator.registers.c = set(emulator, 1 << 1, c);
}

// 0xCA
fn set_1_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  emulator.registers.d = set(emulator, 1 << 1, d);
}

// 0xCB
fn set_1_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  emulator.registers.e = set(emulator, 1 << 1, e);
}

// 0xCC
fn set_1_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  emulator.registers.h = set(emulator, 1 << 1, h);
}

// 0xCD
fn set_1_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  emulator.registers.l = set(emulator, 1 << 1, l);
}

// 0xCE
fn set_1_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  let result = set(emulator, 1 << 1, hl_value);
  emulator.memory.write_byte(hl, result);
}

// 0xCF
fn set_1_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  emulator.registers.a = set(emulator, 1 << 1, a);
}

// 0xD0
fn set_2_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  emulator.registers.b = set(emulator, 1 << 2, b);
}

// 0xD1
fn set_2_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  emulator.registers.c = set(emulator, 1 << 2, c);
}

// 0xD2
fn set_2_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  emulator.registers.d = set(emulator, 1 << 2, d);
}

// 0xD3
fn set_2_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  emulator.registers.e = set(emulator, 1 << 2, e);
}

// 0xD4
fn set_2_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  emulator.registers.h = set(emulator, 1 << 2, h);
}

// 0xD5
fn set_2_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  emulator.registers.l = set(emulator, 1 << 2, l);
}

// 0xD6
fn set_2_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  let result = set(emulator, 1 << 2, hl_value);
  emulator.memory.write_byte(hl, result);
}

// 0xD7
fn set_2_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  emulator.registers.a = set(emulator, 1 << 2, a);
}

// 0xD8
fn set_3_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  emulator.registers.b = set(emulator, 1 << 3, b);
}

// 0xD9
fn set_3_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  emulator.registers.c = set(emulator, 1 << 3, c);
}

// 0xDA
fn set_3_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  emulator.registers.d = set(emulator, 1 << 3, d);
}

// 0xDB
fn set_3_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  emulator.registers.e = set(emulator, 1 << 3, e);
}

// 0xDC
fn set_3_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  emulator.registers.h = set(emulator, 1 << 3, h);
}

// 0xDD
fn set_3_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  emulator.registers.l = set(emulator, 1 << 3, l);
}

// 0xDE
fn set_3_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  let result = set(emulator, 1 << 3, hl_value);
  emulator.memory.write_byte(hl, result);
}

// 0xDF
fn set_3_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  emulator.registers.a = set(emulator, 1 << 3, a);
}

// 0xE0
fn set_4_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  emulator.registers.b = set(emulator, 1 << 4, b);
}

// 0xE1
fn set_4_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  emulator.registers.c = set(emulator, 1 << 4, c);
}

// 0xE2
fn set_4_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  emulator.registers.d = set(emulator, 1 << 4, d);
}

// 0xE3
fn set_4_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  emulator.registers.e = set(emulator, 1 << 4, e);
}

// 0xE4
fn set_4_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  emulator.registers.h = set(emulator, 1 << 4, h);
}

// 0xE5
fn set_4_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  emulator.registers.l = set(emulator, 1 << 4, l);
}

// 0xE6
fn set_4_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  let result = set(emulator, 1 << 4, hl_value);
  emulator.memory.write_byte(hl, result);
}

// 0xE7
fn set_4_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  emulator.registers.a = set(emulator, 1 << 4, a);
}

// 0xE8
fn set_5_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  emulator.registers.b = set(emulator, 1 << 5, b);
}

// 0xE9
fn set_5_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  emulator.registers.c = set(emulator, 1 << 5, c);
}

// 0xEA
fn set_5_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  emulator.registers.d = set(emulator, 1 << 5, d);
}

// 0xEB
fn set_5_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  emulator.registers.e = set(emulator, 1 << 5, e);
}

// 0xEC
fn set_5_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  emulator.registers.h = set(emulator, 1 << 5, h);
}

// 0xED
fn set_5_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  emulator.registers.l = set(emulator, 1 << 5, l);
}

// 0xEE
fn set_5_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  let result = set(emulator, 1 << 5, hl_value);
  emulator.memory.write_byte(hl, result);
}

// 0xEF
fn set_5_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  emulator.registers.a = set(emulator, 1 << 5, a);
}

// 0xF0
fn set_6_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  emulator.registers.b = set(emulator, 1 << 6, b);
}

// 0xF1
fn set_6_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  emulator.registers.c = set(emulator, 1 << 6, c);
}

// 0xF2
fn set_6_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  emulator.registers.d = set(emulator, 1 << 6, d);
}

// 0xF3
fn set_6_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  emulator.registers.e = set(emulator, 1 << 6, e);
}

// 0xF4
fn set_6_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  emulator.registers.h = set(emulator, 1 << 6, h);
}

// 0xF5
fn set_6_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  emulator.registers.l = set(emulator, 1 << 6, l);
}

// 0xF6
fn set_6_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  let result = set(emulator, 1 << 6, hl_value);
  emulator.memory.write_byte(hl, result);
}

// 0xF7
fn set_6_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  emulator.registers.a = set(emulator, 1 << 6, a);
}

// 0xF8
fn set_7_b(emulator: &mut Emulator) {
  let b = emulator.registers.b;
  emulator.registers.b = set(emulator, 1 << 7, b);
}

// 0xF9
fn set_7_c(emulator: &mut Emulator) {
  let c = emulator.registers.c;
  emulator.registers.c = set(emulator, 1 << 7, c);
}

// 0xFA
fn set_7_d(emulator: &mut Emulator) {
  let d = emulator.registers.d;
  emulator.registers.d = set(emulator, 1 << 7, d);
}

// 0xFB
fn set_7_e(emulator: &mut Emulator) {
  let e = emulator.registers.e;
  emulator.registers.e = set(emulator, 1 << 7, e);
}

// 0xFC
fn set_7_h(emulator: &mut Emulator) {
  let h = emulator.registers.h;
  emulator.registers.h = set(emulator, 1 << 7, h);
}

// 0xFD
fn set_7_l(emulator: &mut Emulator) {
  let l = emulator.registers.l;
  emulator.registers.l = set(emulator, 1 << 7, l);
}

// 0xFE
fn set_7_hlp(emulator: &mut Emulator) {
  let hl = emulator.registers.get_hl();
  let hl_value = emulator.memory.read_byte(hl);
  let result = set(emulator, 1 << 7, hl_value);
  emulator.memory.write_byte(hl, result);
}

// 0xFF
fn set_7_a(emulator: &mut Emulator) {
  let a = emulator.registers.a;
  emulator.registers.a = set(emulator, 1 << 7, a);
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
