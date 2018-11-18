use {
  core::{
    cpu::{
      rl,
      rlc,
      rr,
      rrc,
      srl_flag_update,
    },
    emulator::Emulator,
    instruction::Instruction,
    registers::{ FLAG_CARRY, FLAG_HALF_CARRY, FLAG_NEGATIVE, FLAG_ZERO }
  },
};

pub fn get_instructions() -> [Instruction; 256] {
  return [
    // 0x00
    Instruction::new("RLC B", 8, rlc_b),
    // 0x01
    Instruction::new("RLC C", 8, rlc_c),
    // 0x02
    Instruction::new("RLC D", 8, rlc_d),
    // 0x03
    Instruction::new("RLC E", 8, rlc_e),
    // 0x04
    Instruction::new("RLC H", 8, rlc_h),
    // 0x05
    Instruction::new("RLC L", 8, rlc_l),
    // 0x06
    Instruction::new("RLC (HL)", 16, rlc_hlp),
    // 0x07
    Instruction::new("RLC A", 8, rlc_a),
    
    // 0x08
    Instruction::new("RRC B", 8, rrc_b),
    // 0x09
    Instruction::new("RRC C", 8, rrc_c),
    // 0x0A
    Instruction::new("RRC D", 8, rrc_d),
    // 0x0B
    Instruction::new("RRC E", 8, rrc_e),
    // 0x0C
    Instruction::new("RRC H", 8, rrc_h),
    // 0x0D
    Instruction::new("RRC L", 8, rrc_l),
    // 0x0E
    Instruction::new("RRC (HL)", 16, rrc_hlp),
    // 0x0F
    Instruction::new("RRC A", 8, rrc_a),

    // 0x10
    Instruction::new("RL B", 8, rl_b),
    // 0x11
    Instruction::new("RL C", 8, rl_c),
    // 0x12
    Instruction::new("RL D", 8, rl_d),
    // 0x13
    Instruction::new("RL E", 8, rl_e),
    // 0x14
    Instruction::new("RL H", 8, rl_h),
    // 0x15
    Instruction::new("RL L", 8, rl_l),
    // 0x16
    Instruction::new("RL (HL)", 16, rl_hlp),
    // 0x17
    Instruction::new("RL A", 8, rl_a),
    
    // 0x18
    Instruction::new("RR B", 8, rr_b),
    // 0x19
    Instruction::new("RR C", 8, rr_c),
    // 0x1A
    Instruction::new("RR D", 8, rr_d),
    // 0x1B
    Instruction::new("RR E", 8, rr_e),
    // 0x1C
    Instruction::new("RR H", 8, rr_h),
    // 0x1D
    Instruction::new("RR L", 8, rr_l),
    // 0x1E
    Instruction::new("RR (HL)", 16, rr_hlp),
    // 0x1F
    Instruction::new("RR A", 8, rr_a),

    // 0x20
    Instruction::new("SLA B", 8, sla_b),
    // 0x21
    Instruction::new("SLA C", 8, sla_c),
    // 0x22
    Instruction::new("SLA D", 8, sla_d),
    // 0x23
    Instruction::new("SLA E", 8, sla_e),
    // 0x24
    Instruction::new("SLA H", 8, sla_h),
    // 0x25
    Instruction::new("SLA L", 8, sla_l),
    // 0x26
    Instruction::new("SLA (HL)", 16, sla_hlp),
    // 0x27
    Instruction::new("SLA A", 8, sla_a),

    // 0x28
    Instruction::new("SRA B", 8, sra_b),
    // 0x29
    Instruction::new("SRA C", 8, sra_c),
    // 0x2A
    Instruction::new("SRA D", 8, sra_d),
    // 0x2B
    Instruction::new("SRA E", 8, sra_e),
    // 0x2C
    Instruction::new("SRA H", 8, sra_h),
    // 0x2D
    Instruction::new("SRA L", 8, sra_l),
    // 0x2E
    Instruction::new("SRA (HL)", 16, sra_hlp),
    // 0x2F
    Instruction::new("SRA A", 8, sra_a),

    // 0x30
    Instruction::new("SWAP B", 8, swap_b),
    // 0x31
    Instruction::new("SWAP C", 8, swap_c),
    // 0x32
    Instruction::new("SWAP D", 8, swap_d),
    // 0x33
    Instruction::new("SWAP E", 8, swap_e),
    // 0x34
    Instruction::new("SWAP H", 8, swap_h),
    // 0x35
    Instruction::new("SWAP L", 8, swap_l),
    // 0x36
    Instruction::new("SWAP (HL)", 16, swap_hlp),
    // 0x37
    Instruction::new("SWAP A", 8, swap_a),

    // 0x38
    Instruction::new("SRL B", 8, srl_b),
    // 0x39
    Instruction::new("SRL C", 8, srl_c),
    // 0x3A
    Instruction::new("SRL D", 8, srl_d),
    // 0x3B
    Instruction::new("SRL E", 8, srl_e),
    // 0x3C
    Instruction::new("SRL H", 8, srl_h),
    // 0x3D
    Instruction::new("SRL L", 8, srl_l),
    // 0x3E
    Instruction::new("SRL (HL)", 16, srl_hlp),
    // 0x3F
    Instruction::new("SRL A", 8, srl_a),

    // 0x40
    Instruction::new("BIT 0 B", 8, bit_0_b),
    // 0x41
    Instruction::new("BIT 0 C", 8, bit_0_c),
    // 0x42
    Instruction::new("BIT 0 D", 8, bit_0_d),
    // 0x43
    Instruction::new("BIT 0 E", 8, bit_0_e),
    // 0x44
    Instruction::new("BIT 0 H", 8, bit_0_h),
    // 0x45
    Instruction::new("BIT 0 L", 8, bit_0_l),
    // 0x46
    Instruction::new("BIT 0 (HL)", 12, bit_0_hlp),
    // 0x47
    Instruction::new("BIT 0 A", 8, bit_0_a),

    // 0x48
    Instruction::new("BIT 1 B", 8, bit_1_b),
    // 0x49
    Instruction::new("BIT 1 C", 8, bit_1_c),
    // 0x4A
    Instruction::new("BIT 1 D", 8, bit_1_d),
    // 0x4B
    Instruction::new("BIT 1 E", 8, bit_1_e),
    // 0x4C
    Instruction::new("BIT 1 H", 8, bit_1_h),
    // 0x4D
    Instruction::new("BIT 1 L", 8, bit_1_l),
    // 0x4E
    Instruction::new("BIT 1 (HL)", 12, bit_1_hlp),
    // 0x4F
    Instruction::new("BIT 1 A", 8, bit_1_a),

    // 0x50
    Instruction::new("BIT 2 B", 8, bit_2_b),
    // 0x51
    Instruction::new("BIT 2 C", 8, bit_2_c),
    // 0x52
    Instruction::new("BIT 2 D", 8, bit_2_d),
    // 0x53
    Instruction::new("BIT 2 E", 8, bit_2_e),
    // 0x54
    Instruction::new("BIT 2 H", 8, bit_2_h),
    // 0x55
    Instruction::new("BIT 2 L", 8, bit_2_l),
    // 0x56
    Instruction::new("BIT 2 (HL)", 12, bit_2_hlp),
    // 0x57
    Instruction::new("BIT 2 A", 8, bit_2_a),

    // 0x58
    Instruction::new("BIT 3 B", 8, bit_3_b),
    // 0x59
    Instruction::new("BIT 3 C", 8, bit_3_c),
    // 0x5A
    Instruction::new("BIT 3 D", 8, bit_3_d),
    // 0x5B
    Instruction::new("BIT 3 E", 8, bit_3_e),
    // 0x5C
    Instruction::new("BIT 3 H", 8, bit_3_h),
    // 0x5D
    Instruction::new("BIT 3 L", 8, bit_3_l),
    // 0x5E
    Instruction::new("BIT 3 (HL)", 12, bit_3_hlp),
    // 0x5F
    Instruction::new("BIT 3 A", 8, bit_3_a),

    // 0x60
    Instruction::new("BIT 4 B", 8, bit_4_b),
    // 0x61
    Instruction::new("BIT 4 C", 8, bit_4_c),
    // 0x62
    Instruction::new("BIT 4 D", 8, bit_4_d),
    // 0x63
    Instruction::new("BIT 4 E", 8, bit_4_e),
    // 0x64
    Instruction::new("BIT 4 H", 8, bit_4_h),
    // 0x65
    Instruction::new("BIT 4 L", 8, bit_4_l),
    // 0x66
    Instruction::new("BIT 4 (HL)", 12, bit_4_hlp),
    // 0x67
    Instruction::new("BIT 4 A", 8, bit_4_a),

    // 0x68
    Instruction::new("BIT 5 B", 8, bit_5_b),
    // 0x69
    Instruction::new("BIT 5 C", 8, bit_5_c),
    // 0x6A
    Instruction::new("BIT 5 D", 8, bit_5_d),
    // 0x6B
    Instruction::new("BIT 5 E", 8, bit_5_e),
    // 0x6C
    Instruction::new("BIT 5 H", 8, bit_5_h),
    // 0x6D
    Instruction::new("BIT 5 L", 8, bit_5_l),
    // 0x6E
    Instruction::new("BIT 5 (HL)", 12, bit_5_hlp),
    // 0x6F
    Instruction::new("BIT 5 A", 8, bit_5_a),
    
    // 0x70
    Instruction::new("BIT 6 B", 8, bit_6_b),
    // 0x71
    Instruction::new("BIT 6 C", 8, bit_6_c),
    // 0x72
    Instruction::new("BIT 6 D", 8, bit_6_d),
    // 0x73
    Instruction::new("BIT 6 E", 8, bit_6_e),
    // 0x74
    Instruction::new("BIT 6 H", 8, bit_6_h),
    // 0x75
    Instruction::new("BIT 6 L", 8, bit_6_l),
    // 0x76
    Instruction::new("BIT 6 (HL)", 12, bit_6_hlp),
    // 0x77
    Instruction::new("BIT 6 A", 8, bit_6_a),

    // 0x78
    Instruction::new("BIT 7 B", 8, bit_7_b),
    // 0x79
    Instruction::new("BIT 7 C", 8, bit_7_c),
    // 0x7A
    Instruction::new("BIT 7 D", 8, bit_7_d),
    // 0x7B
    Instruction::new("BIT 7 E", 8, bit_7_e),
    // 0x7C
    Instruction::new("BIT 7 H", 8, bit_7_h),
    // 0x7D
    Instruction::new("BIT 7 L", 8, bit_7_l),
    // 0x7E
    Instruction::new("BIT 7 (HL)", 12, bit_7_hlp),
    // 0x7F
    Instruction::new("BIT 7 A", 8, bit_7_a),
    
    // 0x80
    Instruction::new("RES 0 B", 8, res_0_b),
    // 0x81
    Instruction::new("RES 0 C", 8, res_0_c),
    // 0x82
    Instruction::new("RES 0 D", 8, res_0_d),
    // 0x83
    Instruction::new("RES 0 E", 8, res_0_e),
    // 0x84
    Instruction::new("RES 0 H", 8, res_0_h),
    // 0x85
    Instruction::new("RES 0 L", 8, res_0_l),
    // 0x86
    Instruction::new("RES 0 (HL)", 12, res_0_hlp),
    // 0x87
    Instruction::new("RES 0 A", 8, res_0_a),

    // 0x88
    Instruction::new("RES 1 B", 8, res_1_b),
    // 0x89
    Instruction::new("RES 1 C", 8, res_1_c),
    // 0x8A
    Instruction::new("RES 1 D", 8, res_1_d),
    // 0x8B
    Instruction::new("RES 1 E", 8, res_1_e),
    // 0x8C
    Instruction::new("RES 1 H", 8, res_1_h),
    // 0x8D
    Instruction::new("RES 1 L", 8, res_1_l),
    // 0x8E
    Instruction::new("RES 1 (HL)", 12, res_1_hlp),
    // 0x8F
    Instruction::new("RES 1 A", 8, res_1_a),

    // 0x90
    Instruction::new("RES 2 B", 8, res_2_b),
    // 0x91
    Instruction::new("RES 2 C", 8, res_2_c),
    // 0x92
    Instruction::new("RES 2 D", 8, res_2_d),
    // 0x93
    Instruction::new("RES 2 E", 8, res_2_e),
    // 0x94
    Instruction::new("RES 2 H", 8, res_2_h),
    // 0x95
    Instruction::new("RES 2 L", 8, res_2_l),
    // 0x96
    Instruction::new("RES 2 (HL)", 12, res_2_hlp),
    // 0x97
    Instruction::new("RES 2 A", 8, res_2_a),

    // 0x98
    Instruction::new("RES 3 B", 8, res_3_b),
    // 0x99
    Instruction::new("RES 3 C", 8, res_3_c),
    // 0x9A
    Instruction::new("RES 3 D", 8, res_3_d),
    // 0x9B
    Instruction::new("RES 3 E", 8, res_3_e),
    // 0x9C
    Instruction::new("RES 3 H", 8, res_3_h),
    // 0x9D
    Instruction::new("RES 3 L", 8, res_3_l),
    // 0x9E
    Instruction::new("RES 3 (HL)", 12, res_3_hlp),
    // 0x9F
    Instruction::new("RES 3 A", 8, res_3_a),

    // 0xA0
    Instruction::new("RES 4 B", 8, res_4_b),
    // 0xA1
    Instruction::new("RES 4 C", 8, res_4_c),
    // 0xA2
    Instruction::new("RES 4 D", 8, res_4_d),
    // 0xA3
    Instruction::new("RES 4 E", 8, res_4_e),
    // 0xA4
    Instruction::new("RES 4 H", 8, res_4_h),
    // 0xA5
    Instruction::new("RES 4 L", 8, res_4_l),
    // 0xA6
    Instruction::new("RES 4 (HL)", 12, res_4_hlp),
    // 0xA7
    Instruction::new("RES 4 A", 8, res_4_a),

    // 0xA8
    Instruction::new("RES 5 B", 8, res_5_b),
    // 0xA9
    Instruction::new("RES 5 C", 8, res_5_c),
    // 0xAA
    Instruction::new("RES 5 D", 8, res_5_d),
    // 0xAB
    Instruction::new("RES 5 E", 8, res_5_e),
    // 0xAC
    Instruction::new("RES 5 H", 8, res_5_h),
    // 0xAD
    Instruction::new("RES 5 L", 8, res_5_l),
    // 0xAE
    Instruction::new("RES 5 (HL)", 12, res_5_hlp),
    // 0xAF
    Instruction::new("RES 5 A", 8, res_5_a),

    // 0xB0
    Instruction::new("RES 6 B", 8, res_6_b),
    // 0xB1
    Instruction::new("RES 6 C", 8, res_6_c),
    // 0xB2
    Instruction::new("RES 6 D", 8, res_6_d),
    // 0xB3
    Instruction::new("RES 6 E", 8, res_6_e),
    // 0xB4
    Instruction::new("RES 6 H", 8, res_6_h),
    // 0xB5
    Instruction::new("RES 6 L", 8, res_6_l),
    // 0xB6
    Instruction::new("RES 6 (HL)", 12, res_6_hlp),
    // 0xB7
    Instruction::new("RES 6 A", 8, res_6_a),

    // 0xB8
    Instruction::new("RES 7 B", 8, res_7_b),
    // 0xB9
    Instruction::new("RES 7 C", 8, res_7_c),
    // 0xBA
    Instruction::new("RES 7 D", 8, res_7_d),
    // 0xBB
    Instruction::new("RES 7 E", 8, res_7_e),
    // 0xBC
    Instruction::new("RES 7 H", 8, res_7_h),
    // 0xBD
    Instruction::new("RES 7 L", 8, res_7_l),
    // 0xBE
    Instruction::new("RES 7 (HL)", 12, res_7_hlp),
    // 0xBF
    Instruction::new("RES 7 A", 8, res_7_a),

    // 0xC0
    Instruction::new("SET 0 B", 8, set_0_b),
    // 0xC1
    Instruction::new("SET 0 C", 8, set_0_c),
    // 0xC2
    Instruction::new("SET 0 D", 8, set_0_d),
    // 0xC3
    Instruction::new("SET 0 E", 8, set_0_e),
    // 0xC4
    Instruction::new("SET 0 H", 8, set_0_h),
    // 0xC5
    Instruction::new("SET 0 L", 8, set_0_l),
    // 0xC6
    Instruction::new("SET 0 (HL)", 12, set_0_hlp),
    // 0xC7
    Instruction::new("SET 0 A", 8, set_0_a),

    // 0xC8
    Instruction::new("SET 1 B", 8, set_1_b),
    // 0xC9
    Instruction::new("SET 1 C", 8, set_1_c),
    // 0xCA
    Instruction::new("SET 1 D", 8, set_1_d),
    // 0xCB
    Instruction::new("SET 1 E", 8, set_1_e),
    // 0xCC
    Instruction::new("SET 1 H", 8, set_1_h),
    // 0xCD
    Instruction::new("SET 1 L", 8, set_1_l),
    // 0xCE
    Instruction::new("SET 1 (HL)", 12, set_1_hlp),
    // 0xCF
    Instruction::new("SET 1 A", 8, set_1_a),

    // 0xD0
    Instruction::new("SET 2 B", 8, set_2_b),
    // 0xD1
    Instruction::new("SET 2 C", 8, set_2_c),
    // 0xD2
    Instruction::new("SET 2 D", 8, set_2_d),
    // 0xD3
    Instruction::new("SET 2 E", 8, set_2_e),
    // 0xD4
    Instruction::new("SET 2 H", 8, set_2_h),
    // 0xD5
    Instruction::new("SET 2 L", 8, set_2_l),
    // 0xD6
    Instruction::new("SET 2 (HL)", 12, set_2_hlp),
    // 0xD7
    Instruction::new("SET 2 A", 8, set_2_a),

    // 0xD8
    Instruction::new("SET 3 B", 8, set_3_b),
    // 0xD9
    Instruction::new("SET 3 C", 8, set_3_c),
    // 0xDA
    Instruction::new("SET 3 D", 8, set_3_d),
    // 0xDB
    Instruction::new("SET 3 E", 8, set_3_e),
    // 0xDC
    Instruction::new("SET 3 H", 8, set_3_h),
    // 0xDD
    Instruction::new("SET 3 L", 8, set_3_l),
    // 0xDE
    Instruction::new("SET 3 (HL)", 12, set_3_hlp),
    // 0xDF
    Instruction::new("SET 3 A", 8, set_3_a),

    // 0xE0
    Instruction::new("SET 4 B", 8, set_4_b),
    // 0xE1
    Instruction::new("SET 4 C", 8, set_4_c),
    // 0xE2
    Instruction::new("SET 4 D", 8, set_4_d),
    // 0xE3
    Instruction::new("SET 4 E", 8, set_4_e),
    // 0xE4
    Instruction::new("SET 4 H", 8, set_4_h),
    // 0xE5
    Instruction::new("SET 4 L", 8, set_4_l),
    // 0xE6
    Instruction::new("SET 4 (HL)", 12, set_4_hlp),
    // 0xE7
    Instruction::new("SET 4 A", 8, set_4_a),

    // 0xE8
    Instruction::new("SET 5 B", 8, set_5_b),
    // 0xE9
    Instruction::new("SET 5 C", 8, set_5_c),
    // 0xEA
    Instruction::new("SET 5 D", 8, set_5_d),
    // 0xEB
    Instruction::new("SET 5 E", 8, set_5_e),
    // 0xEC
    Instruction::new("SET 5 H", 8, set_5_h),
    // 0xED
    Instruction::new("SET 5 L", 8, set_5_l),
    // 0xEE
    Instruction::new("SET 5 (HL)", 12, set_5_hlp),
    // 0xEF
    Instruction::new("SET 5 A", 8, set_5_a),

    // 0xF0
    Instruction::new("SET 6 B", 8, set_6_b),
    // 0xF1
    Instruction::new("SET 6 C", 8, set_6_c),
    // 0xF2
    Instruction::new("SET 6 D", 8, set_6_d),
    // 0xF3
    Instruction::new("SET 6 E", 8, set_6_e),
    // 0xF4
    Instruction::new("SET 6 H", 8, set_6_h),
    // 0xF5
    Instruction::new("SET 6 L", 8, set_6_l),
    // 0xF6
    Instruction::new("SET 6 (HL)", 12, set_6_hlp),
    // 0xF7
    Instruction::new("SET 6 A", 8, set_6_a),

    // 0xF8
    Instruction::new("SET 7 B", 8, set_7_b),
    // 0xF9
    Instruction::new("SET 7 C", 8, set_7_c),
    // 0xFA
    Instruction::new("SET 7 D", 8, set_7_d),
    // 0xFB
    Instruction::new("SET 7 E", 8, set_7_e),
    // 0xFC
    Instruction::new("SET 7 H", 8, set_7_h),
    // 0xFD
    Instruction::new("SET 7 L", 8, set_7_l),
    // 0xFE
    Instruction::new("SET 7 (HL)", 12, set_7_hlp),
    // 0xFF
    Instruction::new("SET 7 A", 8, set_7_a),
  ];
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
  emulator.memory.write_byte(hl, result);
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
  emulator.memory.write_byte(hl, result);
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
  emulator.memory.write_byte(hl, result);
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
  emulator.memory.write_byte(hl, result);
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
  emulator.memory.write_byte(hl, result);
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
  emulator.memory.write_byte(hl, result);
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
  emulator.memory.write_byte(hl, result);
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
  emulator.memory.write_byte(hl, result);
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

fn sla(emulator: &mut Emulator, value: u8) -> u8 {
  let carry = value & 0x80 == 0x80;
  let result = value << 1;
  srl_flag_update(emulator, result, carry);
  return result;
}

fn sra(emulator: &mut Emulator, value: u8) -> u8 {
  let carry = value & 0x01 == 0x01;
  let result = (value >> 1) | (value & 0x80);
  srl_flag_update(emulator, result, carry);
  return result;
}

fn srl(emulator: &mut Emulator, value: u8) -> u8 {
  let carry = value & 0x01 == 0x01;
  let result = value >> 1;
  srl_flag_update(emulator, result, carry);
  return result;
}

fn swap(emulator: &mut Emulator, value: u8) -> u8 {
  emulator.registers.set_flags(FLAG_ZERO, value == 0);
  emulator.registers.set_flags(FLAG_NEGATIVE | FLAG_HALF_CARRY | FLAG_CARRY, false);
  (value >> 4) | (value << 4)
}

fn bit(emulator: &mut Emulator, bit: u8, value: u8) {
  emulator.registers.set_flags(FLAG_ZERO, (value & bit) == 0);
  emulator.registers.set_flags(FLAG_NEGATIVE, false);
  emulator.registers.set_flags(FLAG_HALF_CARRY, true);
}

fn set(emulator: &mut Emulator, bit: u8, value: u8) -> u8 {
  value | bit
}
