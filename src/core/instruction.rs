use {
  core::emulator::Emulator,
};

pub struct Instruction {
  pub disassembly: &'static str,
  pub operation_time: u8,
  pub operation: Box<Fn(&mut Emulator)>,
}

impl Instruction {
  pub fn new<F>(
    disassembly: &'static str,
    operation_time: u8,
    operation: F,
  ) -> Instruction where F: Fn(&mut Emulator) + 'static {
    Instruction {
      disassembly,
      operation_time,
      operation: Box::new(operation),
    }
  }
}