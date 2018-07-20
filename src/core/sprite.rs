pub struct Sprite {
  pub y: u8,
  pub x: u8,
  pub tile: u8,
  pub options: u8,
}

impl Sprite {
  pub fn get_priority(&self) -> u8 {
    self.options & 0x01
  }

  pub fn get_v_flip(&self) -> u8 {
    self.options & 0x02 >> 1
  }

  pub fn get_h_flip(&self) -> u8 {
    self.options & 0x04 >> 2
  }

  pub fn get_palette(&self) -> u8 {
    self.options & 0x08 >> 3
  }
}
