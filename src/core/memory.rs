use {
  core::{
    emulator::Emulator,
    gpu::PALETTE,
    interrupt::Interrupt,
    registers::Registers,
    keys::Keys,
  },
  rand,
  std::ptr,
};

pub const CART_SIZE: usize = 0x8000;
pub const SRAM_SIZE: usize = 0x2000;
pub const IO_SIZE: usize = 0x100;
pub const VRAM_SIZE: usize = 0x2000;
pub const OAM_SIZE: usize = 0x100;
pub const WRAM_SIZE: usize = 0x2000;
pub const HRAM_SIZE: usize = 0x80;

pub struct Memory {
  pub emulator: *mut Emulator,
  // TODO: Refactor these to use actual devices once starting to get stuff displayed
  pub cart: [u8; CART_SIZE],
  pub sram: [u8; SRAM_SIZE],
  pub io: [u8; IO_SIZE],
  pub vram: [u8; VRAM_SIZE],
  pub oam: [u8; OAM_SIZE],
  pub wram: [u8; WRAM_SIZE],
  pub hram: [u8; HRAM_SIZE],
  pub keys: Keys,
}

impl Memory {
  pub fn new() -> Memory {
    Memory {
      emulator: ptr::null_mut(),
      cart: [0; CART_SIZE],
      sram: [0; SRAM_SIZE],
      io: [
        0x0F, 0x00, 0x7C, 0xFF, 0x00, 0x00, 0x00, 0xF8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01,
        0x80, 0xBF, 0xF3, 0xFF, 0xBF, 0xFF, 0x3F, 0x00, 0xFF, 0xBF, 0x7F, 0xFF, 0x9F, 0xFF, 0xBF, 0xFF,
        0xFF, 0x00, 0x00, 0xBF, 0x77, 0xF3, 0xF1, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF,
        0x91, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFC, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x7E, 0xFF, 0xFE,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x3E, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xC0, 0xFF, 0xC1, 0x00, 0xFE, 0xFF, 0xFF, 0xFF,
        0xF8, 0xFF, 0x00, 0x00, 0x00, 0x8F, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
        0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
        0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        0x45, 0xEC, 0x52, 0xFA, 0x08, 0xB7, 0x07, 0x5D, 0x01, 0xFD, 0xC0, 0xFF, 0x08, 0xFC, 0x00, 0xE5,
        0x0B, 0xF8, 0xC2, 0xCE, 0xF4, 0xF9, 0x0F, 0x7F, 0x45, 0x6D, 0x3D, 0xFE, 0x46, 0x97, 0x33, 0x5E,
        0x08, 0xEF, 0xF1, 0xFF, 0x86, 0x83, 0x24, 0x74, 0x12, 0xFC, 0x00, 0x9F, 0xB4, 0xB7, 0x06, 0xD5,
        0xD0, 0x7A, 0x00, 0x9E, 0x04, 0x5F, 0x41, 0x2F, 0x1D, 0x77, 0x36, 0x75, 0x81, 0xAA, 0x70, 0x3A,
        0x98, 0xD1, 0x71, 0x02, 0x4D, 0x01, 0xC1, 0xFF, 0x0D, 0x00, 0xD3, 0x05, 0xF9, 0x00, 0x0B, 0x00,
      ],
      vram: [0; VRAM_SIZE],
      oam: [0; OAM_SIZE],
      wram: [0; WRAM_SIZE],
      hram: [0; HRAM_SIZE],
      keys: Keys::new(),
    }
  }

  fn copy(&mut self, destination: u16, source: u16, length: u16) {
    for i in 0..length {
      let byte = self.read_byte(source + i);
      self.write_byte(destination + i, byte);
    }
  }

  pub fn read_byte(&self, address: u16) -> u8 {
    if address <= 0x7FFF {
      return self.cart[address as usize];
    }
    if address >= 0xA000 && address <= 0xBFFF {
      return self.sram[(address - 0xA000) as usize];
    }
    if address >= 0x8000 && address <= 0x9FFF {
      return self.vram[(address - 0x8000) as usize];
    }
    if address >= 0xC000 && address <= 0xDFFF {
      return self.wram[(address - 0xC000) as usize];
    }
    if address >= 0xE000 && address <= 0xFDFF {
      return self.wram[(address - 0xE000) as usize];
    }
    if address >= 0xFE00 && address <= 0xFEFF {
      return self.oam[(address - 0xFE00) as usize];
    }
    // TODO: Check if this random number generator works
    if address == 0xFF04 {
      return rand::random::<u8>();
    }
    if address == 0xFF40 {
      return unsafe { (*self.emulator).gpu.control };
    }
    if address == 0xFF42 {
      return unsafe { (*self.emulator).gpu.scroll_y };
    }
    if address == 0xFF43 {
      return unsafe { (*self.emulator).gpu.scroll_x };
    }
    if address == 0xFF44 {
      return unsafe { (*self.emulator).gpu.scan_line };
    }
    if address == 0xFF00 {
      if (self.io[0x00] & 0x20) == 0 {
        return 0xC0 | self.keys.get_keys_1() | 0x10;
      }
      if (self.io[0x00] & 0x10) == 0 {
        return 0xC0 | self.keys.get_keys_2() | 0x20;
      }
      if (self.io[0x00] & 0x30) == 0 {
        return 0xFF;
      }
      return 0;
    }
    if address == 0xFF0F {
      return unsafe { (*self.emulator).interrupt.flags };
    }
    if address == 0xFFFF {
      return unsafe { (*self.emulator).interrupt.enable };
    }
    if address>= 0xFF80 && address <= 0xFFFE {
      return self.hram[(address - 0xFF80) as usize];
    }
    if address >= 0xFF00 && address <= 0xFF7F {
      return self.io[(address - 0xFF00) as usize];
    }
    return 0;
  }

  pub fn read_short(&self, address: u16) -> u16 {
    self.read_byte(address) as u16 | (self.read_byte(address + 1) as u16) << 8
  }

  pub fn read_short_from_stack(&self) -> u16 {
    let emulator = unsafe { &mut *self.emulator };
    let value = self.read_short(emulator.registers.sp);
    emulator.registers.sp += 2;
    value
  }

  pub fn write_byte(&mut self, address: u16, value: u8) {
    if address >= 0xA000 && address <= 0xBFFF {
      self.sram[(address - 0xA000) as usize] = value;
    }
    else if address >= 0x8000 && address <= 0x9fff {
      self.vram[(address - 0x8000) as usize] = value;
      if address <= 0x97FF {
        let emulator = unsafe { &mut *self.emulator };
        emulator.gpu.update_tile(address);
      }
    }
    else if address >= 0xC000 && address <= 0xDFFF {
      self.wram[(address - 0xC000) as usize] = value;
    }
    else if address >= 0xE000 && address <= 0xFDFF {
      self.wram[(address - 0xE000) as usize] = value;
    }
    else if address >= 0xFE00 && address <= 0xFEFF {
      self.oam[(address - 0xFE00) as usize] = value;
    }
    else if address >= 0xFF80 && address <= 0xFFFE {
      self.hram[(address - 0xFF80) as usize] = value
    }
    else if address == 0xFF40 {
      let emulator = unsafe { &mut *self.emulator };
      emulator.gpu.control = value;
    }
    else if address == 0xFF42 {
      let emulator = unsafe { &mut *self.emulator };
      emulator.gpu.scroll_y = value;
    }
    else if address == 0xFF43 {
      let emulator = unsafe { &mut *self.emulator };
      emulator.gpu.scroll_x = value;
    }
    else if address == 0xFF46 {
      self.copy(0xFE00, (value as u16) << 8, 160);
    }
    else if address == 0xFF47 {
      let emulator = unsafe { &mut *self.emulator };
      for i in 0..4 {
        emulator.gpu.background_palette[i] = PALETTE[((value >> (i * 2)) & 3) as usize];
      }
    }
    else if address == 0xFF48 {
      let emulator = unsafe { &mut *self.emulator };
      for i in 0..4 {
        emulator.gpu.sprite_palette[i] = PALETTE[((value >> (i * 2)) & 3) as usize];
      }
    }
    else if address == 0xFF49 {
      let emulator = unsafe { &mut *self.emulator };
      for i in 0..4 {
        emulator.gpu.sprite_palette[4 + i] = PALETTE[((value >> (i * 2)) & 3) as usize];
      }
    }
    else if address >= 0xFF00 && address <= 0xFF7F {
      self.io[(address - 0xFF00) as usize] = value;
    }
    else if address == 0xFF0F {
      let emulator = unsafe { &mut * self.emulator };
      emulator.interrupt.flags = value;
    }
    else if address == 0xFFFF {
      let emulator = unsafe { &mut * self.emulator };
      emulator.interrupt.enable = value;
    }
  }

  pub fn write_short(&mut self, address: u16, value: u16) {
    self.write_byte(address, (value & 0x00FF) as u8);
    self.write_byte(address + 1, ((value & 0xFF00) >> 8) as u8);
  }

  pub fn write_short_to_stack(&mut self, value: u16) {
    let emulator = unsafe { &mut *self.emulator };
    emulator.registers.sp -= 2;
    self.write_short(emulator.registers.sp, value);
  }
}
