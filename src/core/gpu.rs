use {
  core::colour::Colour,
};

const GPU_BG_ENABLE: u8 = 1 << 0;
const GPU_SPRITE_ENABLE: u8 = 1 << 1;
const GPU_SPRITE_V_DOUBLE: u8 = 1 << 2;
const GPU_TILEMAP: u8 = 1 << 3;
const GPU_TILESET: u8 = 1 << 4;
const GPU_WINDOW_ENABLE: u8 = 1 << 5;
const GPU_WINDOW_TILEMAP: u8 = 1 << 6;
const GPU_DISPLAY_ENABLE: u8 = 1 << 7;

const PALETTE: [Colour; 4] = [
  Colour { r: 255, g: 255, b: 255 },
  Colour { r: 192, g: 192, b: 192 },
  Colour { r: 96, g: 96, b: 96 },
  Colour { r: 0, g: 0, b: 0 },
];

pub struct Gpu {
  pub background_palette: [Colour; 4],
  pub control: u8,
  pub scan_line: u8,
  pub scroll_x: u8,
  pub scroll_y: u8,
  pub sprite_palette: [Colour; 4 * 2],
  pub tick: u64,
  pub tiles: [u8; 8 * 8 * 384],
}

impl Gpu {
  pub fn new() -> Gpu {
    Gpu {
      background_palette: PALETTE,
      control: 0,
      scan_line: 0,
      scroll_x: 0,
      scroll_y: 0,
      sprite_palette: [
        PALETTE[0],
        PALETTE[1],
        PALETTE[2],
        PALETTE[3],
        PALETTE[0],
        PALETTE[1],
        PALETTE[2],
        PALETTE[3],
      ],
      tick: 0,
      tiles: [0; 384 * 8 * 8],
    }
  }
}
