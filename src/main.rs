#![allow(dead_code)]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::console::{Color, Font, Window};

mod console;

fn main() {
  let font = Font::new(include_bytes!("../resources/font.png")).unwrap();
  let mut window = Window::new("Zod", 80, 50, font).unwrap();

  let mut x = 0;
  let mut y = 0;

  loop {
    window.clear();
    window.write(0, 0, format!("Player at ({}, {})", x, y), Color::SILVER, None);
    window.put(x, y, '@', Color::GREEN, None);
    window.update().unwrap();

    for event in window.events() {
      match event {
        Event::KeyDown { keycode: Some(keycode), .. } => match keycode {
          Keycode::Up => y = y.wrapping_sub(1),
          Keycode::Down => y = y.wrapping_add(1),
          Keycode::Left => x = x.wrapping_sub(1),
          Keycode::Right => x = x.wrapping_add(1),
          _ => {}
        },
        Event::Quit { .. } => return,
        _ => {}
      }
    }
  }
}
