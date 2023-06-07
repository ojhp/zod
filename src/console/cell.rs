use crate::console::Color;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cell {
  pub glyph: u8,
  pub foreground: Color,
  pub background: Color,
}

impl Default for Cell {
  fn default() -> Self {
    Cell { glyph: b' ', foreground: Color::WHITE, background: Color::BLACK }
  }
}
