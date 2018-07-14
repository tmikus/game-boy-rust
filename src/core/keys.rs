pub struct Keys {
  pub value: u8,
}

impl Keys {
  pub fn new() -> Keys {
    Keys { value: 0 }
  }

  pub fn get_keys_1(&self) -> u8 {
    (self.value & 0xF0) >> 4
  }

  pub fn get_keys_2(&self) -> u8 {
    self.value & 0x0F
  }

  pub fn set_keys_1(&mut self, value: u8) {
    self.value = ((value & 0x0F) << 4) | (self.value & 0x0F);
  }

  pub fn set_keys_2(&mut self, value: u8) {
    self.value = (self.value & 0xF0) | (value & 0x0F);
  }
}
