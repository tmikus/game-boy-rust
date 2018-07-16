use {
  core::emulator::Emulator,
};

pub fn cpu_cb_n(emulator: &mut Emulator) {
  let instruction = emulator.cpu.read_next_byte();
  // TODO: Implement
}