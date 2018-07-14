use {
  core::cpu::Cpu,
};

pub struct Instruction {
  pub disassembly: &'static str,
  pub operation_time: u8,
  pub operation: Box<Fn(&mut Cpu)>,
}

impl Instruction {
  pub fn new<F>(
    disassembly: &'static str,
    operation_time: u8,
    operation: F,
  ) -> Instruction where F: Fn(&mut Cpu) + 'static {
    Instruction {
      disassembly,
      operation_time,
      operation: Box::new(operation),
    }
  }
}