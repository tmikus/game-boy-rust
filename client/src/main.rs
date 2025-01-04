// #![feature(int_to_from_bytes)]
#[macro_use]
extern crate glium;
extern crate num_traits;
#[macro_use]
extern crate num_derive;
extern crate rand;

mod core;

use {
  core::{
    emulator::Emulator,
    keys::Keys,
    rom::load_rom,
  },
  glium::{glutin, Surface},
  std::{thread, time},
};

// Game Boy Cartridge pinout
// GPIO 23 = PIN 13
// GPIO 18 = ANALOG PIN 5
// GPIO 24 = PIN 11
// GPIO 25 = PIN 10
// GPIO 12 = PIN 12
// GPIO 17 = PIN 2
// GPIO 27 = PIN 3
// GPIO 22 = PIN 4
// GPIO 5  = PIN 5
// GPIO 6  = PIN 6
// GPIO 13 = PIN 7
// GPIO 19 = PIN 8
// GPIO 26 = PIN 9

fn main() {
  let mut events_loop = glutin::EventsLoop::new();
  let window = glutin::WindowBuilder::new()
    .with_title("Game Boy Rust")
    .with_dimensions(glutin::dpi::LogicalSize::new(640.0, 640.0))
    .with_resizable(false);
  let context = glutin::ContextBuilder::new();
  let display = glium::Display::new(window, context, &events_loop).unwrap();
  let mut closed = false;
  let mut emulator = Emulator::new();
  emulator.init();
//  let metadata = load_rom(&mut emulator, String::from("C:\\Users\\tmikus\\Projects\\tetris.gb"));
  let metadata = load_rom(&mut emulator, String::from("C:\\Users\\tmikus\\Projects\\dr-mario.gb"));
//   let metadata = load_rom(&mut emulator, String::from("C:\\Users\\tmikus\\Projects\\01-special.gb"));
//  let metadata = load_rom(&mut emulator, String::from("C:\\Users\\tmikus\\Projects\\02-interrupts.gb"));
  metadata.print();
  emulator.reset();
  emulator.gpu.init(display.clone());
  while !closed {
    emulator.run_tick(display.clone());
//    thread::sleep(time::Duration::from_millis(200));
    events_loop.poll_events(|ev| {
      match ev {
        glutin::Event::WindowEvent { event, .. } => match event {
          glutin::WindowEvent::CloseRequested => closed = true,
          glutin::WindowEvent::KeyboardInput { input, .. } => {
            if input.state == glutin::ElementState::Pressed  {
              match input.virtual_keycode {
                Some(glutin::VirtualKeyCode::Escape) => {
                  // TODO: Print registers
                  closed = true;
                },
                Some(glutin::VirtualKeyCode::X) => {
                  emulator.memory.keys.set_a(true);
                  emulator.cpu.stopped = false;
                },
                Some(glutin::VirtualKeyCode::Z) => {
                  emulator.memory.keys.set_b(true);
                  emulator.cpu.stopped = false;
                },
                Some(glutin::VirtualKeyCode::Return) => {
                  emulator.memory.keys.set_start(true);
                  emulator.cpu.stopped = false;
                },
                Some(glutin::VirtualKeyCode::Back) => {
                  emulator.memory.keys.set_select(true);
                  emulator.cpu.stopped = false;
                },
                Some(glutin::VirtualKeyCode::Up) => {
                  emulator.memory.keys.set_up(true);
                  emulator.cpu.stopped = false;
                },
                Some(glutin::VirtualKeyCode::Down) => {
                  emulator.memory.keys.set_down(true);
                  emulator.cpu.stopped = false;
                },
                Some(glutin::VirtualKeyCode::Left) => {
                  emulator.memory.keys.set_left(true);
                  emulator.cpu.stopped = false;
                },
                Some(glutin::VirtualKeyCode::Right) => {
                  emulator.memory.keys.set_right(true);
                  emulator.cpu.stopped = false;
                },
                _ => ()
              }
            } else if input.state == glutin::ElementState::Released {
              match input.virtual_keycode {
                Some(glutin::VirtualKeyCode::X) => {
                  emulator.memory.keys.set_a(false);
                  emulator.cpu.stopped = false;
                },
                Some(glutin::VirtualKeyCode::Z) => {
                  emulator.memory.keys.set_b(false);
                  emulator.cpu.stopped = false;
                },
                Some(glutin::VirtualKeyCode::Return) => {
                  emulator.memory.keys.set_start(false);
                  emulator.cpu.stopped = false;
                },
                Some(glutin::VirtualKeyCode::Return) => {
                  emulator.memory.keys.set_select(false);
                  emulator.cpu.stopped = false;
                },
                Some(glutin::VirtualKeyCode::Up) => {
                  emulator.memory.keys.set_up(false);
                  emulator.cpu.stopped = false;
                },
                Some(glutin::VirtualKeyCode::Down) => {
                  emulator.memory.keys.set_down(false);
                  emulator.cpu.stopped = false;
                },
                Some(glutin::VirtualKeyCode::Left) => {
                  emulator.memory.keys.set_left(false);
                  emulator.cpu.stopped = false;
                },
                Some(glutin::VirtualKeyCode::Right) => {
                  emulator.memory.keys.set_right(false);
                  emulator.cpu.stopped = false;
                },
                _ => ()
              }
            }
          }
          _ => (),
        },
        _ => (),
      }
    });
  }
}
