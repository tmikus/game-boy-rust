pub struct Keys {
  pub value: u8,
}

impl Keys {
  pub fn new() -> Keys {
    Keys { value: 0xFF }
  }

  pub fn get_keys_1(&self) -> u8 {
    (self.value & 0xF0) >> 4
  }

  pub fn get_keys_2(&self) -> u8 {
    self.value & 0x0F
  }

  pub fn reset(&mut self) {
    self.value = 0xFF;
  }

  pub fn set_keys_1(&mut self, value: u8) {
    self.value = ((value & 0x0F) << 4) | (self.value & 0x0F);
  }

  pub fn set_keys_2(&mut self, value: u8) {
    self.value = (self.value & 0xF0) | (value & 0x0F);
  }

  pub fn get_a(&self) -> u8 {
    self.get_keys_1() & 0x01
  }

  pub fn set_a(&mut self, value: bool)  {
    let mut keys = self.get_keys_1();
    if !value {
      keys |= 0x01;
    } else {
      keys &= !0x01;
    }
    self.set_keys_1(keys);
  }

  pub fn get_b(&self) -> u8 {
    self.get_keys_1() & 0x02
  }

  pub fn set_b(&mut self, value: bool) {
    let mut keys = self.get_keys_1();
    if !value {
      keys |= 0x02;
    } else {
      keys &= !0x02;
    }
    self.set_keys_1(keys);
  }

  pub fn get_select(&self) -> u8 {
    self.get_keys_1() & 0x04
  }

  pub fn set_select(&mut self, value: bool) {
    let mut keys = self.get_keys_1();
    if !value {
      keys |= 0x04;
    } else {
      keys &= !0x04;
    }
    self.set_keys_1(keys);
  }

  pub fn get_start(&self) -> u8 {
    self.get_keys_1() & 0x08
  }

  pub fn set_start(&mut self, value: bool) {
    let mut keys = self.get_keys_1();
    if !value {
      keys |= 0x08;
    } else {
      keys &= !0x08;
    }
    self.set_keys_1(keys);
  }

  pub fn get_right(&self) -> u8 {
    self.get_keys_2() & 0x01
  }

  pub fn set_right(&mut self, value: bool) {
    let mut keys = self.get_keys_2();
    if !value {
      keys |= 0x01;
    } else {
      keys &= !0x01;
    }
    self.set_keys_2(keys);
  }

  pub fn get_left(&self) -> u8 {
    self.get_keys_2() & 0x02
  }

  pub fn set_left(&mut self, value: bool) {
    let mut keys = self.get_keys_2();
    if !value {
      keys |= 0x02;
    } else {
      keys &= !0x02;
    }
    self.set_keys_2(keys);
  }

  pub fn get_up(&self) -> u8 {
    self.get_keys_2() & 0x04
  }

  pub fn set_up(&mut self, value: bool) {
    let mut keys = self.get_keys_2();
    if !value {
      keys |= 0x04;
    } else {
      keys &= !0x04;
    }
    self.set_keys_2(keys);
  }

  pub fn get_down(&self) -> u8 {
    self.get_keys_2() & 0x08
  }

  pub fn set_down(&mut self, value: bool) {
    let mut keys = self.get_keys_2();
    if !value {
      keys |= 0x08;
    } else {
      keys &= !0x08;
    }
    self.set_keys_2(keys);
  }
}
