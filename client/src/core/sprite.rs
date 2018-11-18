use {
  std::mem,
};

pub struct Sprite {
  pub y: u8,
  pub x: u8,
  pub tile: u8,
  pub options: u8,
}

impl Sprite {
  pub fn from_array(array: &[u8], index: u8) -> Sprite {
    let offset = (index * 4) as usize;
    Sprite {
      y: array[offset],
      x: array[offset + 1],
      tile: array[offset + 2],
      options: array[offset + 3],
    }
  }

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
