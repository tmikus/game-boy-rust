use {
  core::{
    emulator::Emulator,
    memory::CART_SIZE,
  },
  num_traits::FromPrimitive,
  std::{
    fs::{ File, metadata },
    io::Read,
    path::Path,
    str,
  },
};

#[derive(FromPrimitive)]
pub enum RomType {
  RomPlain = 0x00,
  RomMbc1 = 0x01,
  RomMbc1Ram = 0x02,
  RomMbc1RamBattery = 0x03,
  RomMbc2 = 0x05,
  RomMbc2Battery = 0x06,
  RomRam = 0x08,
  RomRamBattery = 0x09,
  RomMmm01 = 0x0B,
  RomMmm01Sram = 0x0C,
  RomMmm01SramBatter = 0x0D,
  RomMbc3TimerBattery = 0x0F,
  RomMbc3TimerRamBattery = 0x10,
  RomMbc3 = 0x11,
  RomMbc3Ram = 0x12,
  RomMbc3RamBattery = 0x13,
  RomMbc5 = 0x19,
  RomMbc5Ram = 0x1A,
  RomMbc5RamBattery = 0x1B,
  RomMbc5Rumble = 0x1C,
  RomMbc5RumbleSram = 0x1D,
  RomMbc5RumbleSramBattery = 0x1E,
  RomPocketCamera = 0x1F,
  RomBandaiTama5 = 0xFD,
  RomHudsonHuc3 = 0xFE,
  RomHudsonHuc1 = 0xFF,
}

pub const ROM_OFFSET_NAME: u16 = 0x134;
pub const ROM_OFFSET_TYPE: u16 = 0x147;
pub const ROM_OFFSET_ROM_SIZE: u16 = 0x148;
pub const ROM_OFFSET_RAM_SIZE: u16 = 0x149;

pub struct RomMetadata {
  pub name: String,
  pub rom_type: RomType,
}

pub fn load_rom(emulator: &mut Emulator, rom_path: String) -> RomMetadata {
  if !Path::new(&rom_path).exists() {
    panic!("ROM file does not exist: {}", rom_path);
  }
  let file_meta = metadata(&rom_path).unwrap();
  if file_meta.len() > CART_SIZE as u64 {
    panic!("Size of the ROM is larger that the supported {}", CART_SIZE);
  }
  let file = File::open(&rom_path);
  if file.is_err() {
    panic!("Could not open ROM file: {}", file.err().unwrap());
  }
  let mut file = file.unwrap();
  // Reset ROM memory
  emulator.memory.cart = [0; CART_SIZE];
  // Load ROM to the memory
  file.read(&mut emulator.memory.cart);
  // Read ROM name
  let mut name_bytes = [0u8; 17];
  for i in 0..17 {
    let letter = emulator.memory.cart[(ROM_OFFSET_NAME + i) as usize];
    if letter == 0x80 || letter == 0xc0 {
      name_bytes[i as usize] = 0;
    } else {
      name_bytes[i as usize] = letter;
    }
  }
  let raw_rom_type = emulator.memory.cart[ROM_OFFSET_TYPE as usize];
  let rom_type = RomType::from_u8(raw_rom_type);
  if rom_type.is_none() {
    panic!("Unknown ROM type: {}", raw_rom_type);
  }
  RomMetadata {
    name: String::from(str::from_utf8(&name_bytes).unwrap()),
    rom_type: rom_type.unwrap(),
  }
}
