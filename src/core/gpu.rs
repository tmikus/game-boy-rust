use {
  core::{
    colour::Colour,
    emulator::Emulator,
    interrupt::INTERRUPT_VBLANK,
    sprite::Sprite,
  },
  glium::{
    index::{
      NoIndices,
      PrimitiveType,
    },
    texture::{
      RawImage2d,
      Texture2d,
    },
    uniforms::{
      MagnifySamplerFilter,
      Sampler,
      SamplerBehavior,
    },
    Display,
    Program,
    Surface,
    VertexBuffer,
  },
  std::{
    ptr,
  },
};


#[derive(Clone, Copy)]
struct Vertex {
  position: [f32; 2],
  texture_coordinates: [f32; 2],
}

implement_vertex!(Vertex, position, texture_coordinates);

const GPU_BG_ENABLE: u8 = 1 << 0;
const GPU_SPRITE_ENABLE: u8 = 1 << 1;
const GPU_SPRITE_V_DOUBLE: u8 = 1 << 2;
const GPU_TILEMAP: u8 = 1 << 3;
const GPU_TILESET: u8 = 1 << 4;
const GPU_WINDOW_ENABLE: u8 = 1 << 5;
const GPU_WINDOW_TILEMAP: u8 = 1 << 6;
const GPU_DISPLAY_ENABLE: u8 = 1 << 7;

pub const PALETTE: [Colour; 4] = [
  Colour { r: 255, g: 255, b: 255 },
  Colour { r: 192, g: 192, b: 192 },
  Colour { r: 96, g: 96, b: 96 },
  Colour { r: 0, g: 0, b: 0 },
];

pub enum GpuMode {
  HBlank,
  VBlank,
  OAM,
  VRAM,
}

pub struct Gpu {
  pub background_palette: [Colour; 4],
  pub control: u8,
  pub emulator: *mut Emulator,
  pub frame_buffer: [Colour; 160 * 144],
  pub last_cpu_ticks: u64,
  pub mode: GpuMode,
  pub scan_line: u8,
  pub scroll_x: u8,
  pub scroll_y: u8,
  pub sprite_palette: [Colour; 4 * 2],
  pub ticks: u64,
  pub tiles: [u8; 384 * 8 * 8],
  vertex_buffer: Option<VertexBuffer<Vertex>>,
  indices: Option<NoIndices>,
  program: Option<Program>,
}

impl Gpu {
  pub fn new() -> Gpu {
    Gpu {
      background_palette: PALETTE,
      control: 0,
      emulator: ptr::null_mut(),
      frame_buffer: [Colour::new(255, 255, 255); 160 * 144],
      last_cpu_ticks: 0,
      mode: GpuMode::HBlank,
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
      ticks: 0,
      tiles: [0; 384 * 8 * 8],
      vertex_buffer: None,
      indices: None,
      program: None,
    }
  }

  pub fn reset(&mut self) {
    self.control = 0;
    self.scroll_x = 0;
    self.scroll_y = 0;
    self.scan_line = 0;
    self.ticks = 0;
    self.last_cpu_ticks = 0;
    self.mode = GpuMode::HBlank;
    self.frame_buffer = [Colour::new(255, 255, 255); 160 * 144];
    self.background_palette = PALETTE;
    self.sprite_palette = [
      PALETTE[0],
      PALETTE[1],
      PALETTE[2],
      PALETTE[3],
      PALETTE[0],
      PALETTE[1],
      PALETTE[2],
      PALETTE[3],
    ];
  }

  pub fn run_tick(&mut self) {
    let emulator = unsafe { &mut *self.emulator };
    self.ticks += emulator.cpu.ticks - self.last_cpu_ticks;
    self.last_cpu_ticks = emulator.cpu.ticks;
    match self.mode {
      GpuMode::HBlank => {
        if self.ticks >= 204 {
          self.scan_line += 1;
          if self.scan_line == 143 {
            if emulator.interrupt.enable & INTERRUPT_VBLANK != 0 {
              emulator.interrupt.flags |= INTERRUPT_VBLANK;
            }
            self.mode = GpuMode::VBlank;
          } else {
            self.mode = GpuMode::OAM;
            self.ticks -= 204;
          }
        }
      },
      GpuMode::VBlank => {
        if self.ticks >= 456 {
          self.scan_line += 1;
          if self.scan_line > 153 {
            self.scan_line = 0;
            self.mode = GpuMode::OAM;
          } 
          self.ticks -= 456;
        }
      },
      GpuMode::OAM => {
        if self.ticks >= 80 {
          self.mode = GpuMode::VRAM;
          self.ticks -= 80;
        }
      },
      GpuMode::VRAM => {
        if self.ticks >= 172 {
          self.mode = GpuMode::HBlank;
          self.render_scan_line();
          self.ticks -= 172;
        }
      },
    };
  }

  pub fn render_scan_line(&mut self) {
    let emulator = unsafe { &mut *self.emulator };
    let mut map_offset: u16 = if (self.control & GPU_TILEMAP) != 0 {
      0x1C00
    } else {
      0x1800
    };
    map_offset += (((self.scan_line as u16 + self.scroll_y as u16) & 255) >> 3) << 5;
    let mut line_offset = self.scroll_x as u16 >> 3;
    let mut x = self.scroll_x & 7;
//    let y = (Wrapping(self.scan_line) + Wrapping(self.scroll_y)).0 & 7;
    let y = (self.scan_line + self.scroll_y) & 7;
    let mut pixel_offset = (self.scan_line as u16) * 160;
    let mut tile = emulator.memory.vram[(map_offset + line_offset) as usize] as u16;
    let mut scan_line_row = [0u8; 160];
    for i in 0..160 {
      let colour = self.tiles[((tile * 64) + (y as u16 * 8) + x as u16) as usize];
      scan_line_row[i] = colour;
      self.frame_buffer[pixel_offset as usize] = self.background_palette[colour as usize];
      x += 1;
      pixel_offset += 1;
      if x == 8 {
        x = 0;
        line_offset = (line_offset + 1) & 31;
        tile = emulator.memory.vram[(map_offset + line_offset) as usize] as u16;
      }
    }
    for i in 0..40 {
      let sprite = Sprite::from_array(&emulator.memory.oam, i);
      let sx = sprite.x as i32 - 8;
      let sy = sprite.y as i32 - 16;
      if sy <= self.scan_line as i32 && (sy + 8) > self.scan_line as i32 {
        let palette_offset = sprite.get_palette() * 4;
        let mut pixel_offset = (self.scan_line as i32 * 160) + sx;
        let tile_row = if sprite.get_v_flip() != 0 {
          7 - (self.scan_line as i32 - sy)
        } else {
          self.scan_line as i32 - sy
        };
        for x in 0..8 {
          if  sx + x >= 0
            && sx + x < 160
            && ((!sprite.get_priority()) != 0 || (!scan_line_row[(sx + x) as usize]) != 0) {
            let colour = if sprite.get_h_flip() != 0 {
              self.tiles[(sprite.tile as usize * 64) + (tile_row as usize * 8) + (7 - x as usize)]
            } else {
              self.tiles[(sprite.tile as usize * 64) + (tile_row as usize * 8) + x as usize]
            };
            if colour != 0 {
              let sprite_colour = self.sprite_palette[(palette_offset + colour) as usize];
              self.frame_buffer[pixel_offset as usize] = sprite_colour;
            }
            pixel_offset += 1;
          }
        }
      }
    }
  }

  pub fn update_tile(&mut self, address: u16) {
    let emulator = unsafe { &mut *self.emulator };
    let address = address & 0x1FFE;
    let tile = (address >> 4) & 511;
    let y = (address >> 1) & 7;
    let mut bit_index: u8;
    for x in 0..8 {
      bit_index = 1 << (7 - x);
      let first_bit: u8 = if (emulator.memory.vram[address as usize] & bit_index) != 0 {
        1
      } else {
        0
      };
      let second_bit: u8 = if (emulator.memory.vram[(address + 1) as usize] & bit_index) != 0 {
        2
      } else {
        0
      };
      self.tiles[(tile * 64 + y * 8 + x) as usize] = first_bit + second_bit;
    }
  }

  pub fn init(&mut self, display: Display) {
    let shape = vec![
      Vertex { position: [-1.0, -1.0], texture_coordinates: [0.0, 0.0] },
      Vertex { position: [-1.0, 1.0], texture_coordinates: [0.0, 1.0] },
      Vertex { position: [1.0, 1.0], texture_coordinates: [1.0, 1.0] },
      Vertex { position: [-1.0, -1.0], texture_coordinates: [0.0, 0.0] },
      Vertex { position: [1.0, 1.0], texture_coordinates: [1.0, 1.0] },
      Vertex { position: [1.0, -1.0], texture_coordinates: [1.0, 0.0] },
    ];
    self.vertex_buffer = Some(VertexBuffer::new(&display, &shape).unwrap());
    self.indices = Some(NoIndices(PrimitiveType::TrianglesList));
    let vertex_shader_src = r#"
      #version 140

      in vec2 position;
      in vec2 texture_coordinates;
      out vec2 v_texture_coordinates;

      void main() {
        v_texture_coordinates = texture_coordinates;
        gl_Position = vec4(position, 0.0, 1.0);
      }
    "#;
    let fragment_shader_src = r#"
      #version 140

      in vec2 v_texture_coordinates;
      out vec4 color;

      uniform sampler2D tex;

      void main() {
        color = texture(tex, v_texture_coordinates);
      }
    "#;
    self.program = Some(Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap());
  }

  pub fn draw_frame_buffer(&mut self, display: Display) {
    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 1.0);

    let vertex_buffer = self.vertex_buffer.as_ref().unwrap();
    let indices = self.indices.as_ref().unwrap();
    let program = self.program.as_ref().unwrap();
    let mut colours: Vec<u8> = vec![];
    for y in (0..144).rev() {
      let y_offset = y * 160;
      for x in 0..160 {
        let colour = &self.frame_buffer[y_offset + x];
        colours.push(colour.r);
        colours.push(colour.g);
        colours.push(colour.b);
      }
    }
    let image = RawImage2d::from_raw_rgb(colours, (160, 144));
    let texture = Texture2d::new(&display, image).unwrap();
    let uniforms = uniform! {
      tex: Sampler(&texture, SamplerBehavior {
        magnify_filter: MagnifySamplerFilter::Nearest,
        ..Default::default()
      }),
    };

    target.draw(vertex_buffer, indices, program, &uniforms, &Default::default()).unwrap();

    target.finish().unwrap();
  }
}

