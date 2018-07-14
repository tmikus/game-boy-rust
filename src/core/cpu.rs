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
  pub ticks: u64,
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      emulator: ptr::null_mut(),
      instructions: get_instructions(),
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
