extern crate glium;
extern crate glutin;
extern crate rand;

mod core;

use {
  core::{
    emulator::Emulator,
    keys::Keys,
  },
};

fn main() {
  let mut events_loop = glutin::EventsLoop::new();
  let window = glutin::WindowBuilder::new()
    .with_title("Game Boy Rust")
    .with_dimensions(glutin::dpi::LogicalSize::new(640.0, 640.0))
    .with_resizable(false);
  let context = glutin::ContextBuilder::new();
  let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();
  let mut closed = false;
  let mut emulator = Emulator::new();
  while !closed {
    // TODO: Enable the emulator once the ROM is loaded
    // emulator.run_tick();
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
                Some(glutin::VirtualKeyCode::Return) => {
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
