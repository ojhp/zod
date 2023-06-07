use std::ops::{Index, IndexMut, Range};

use crate::console::{cp437_glyph, Cell, Color};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Buffer {
  width: usize,
  height: usize,
  cells: Box<[Cell]>,
}

impl Buffer {
  pub fn new(width: usize, height: usize) -> Self {
    let cells = vec![Cell::default(); width * height].into_boxed_slice();
    Buffer { width, height, cells }
  }

  pub fn width(&self) -> usize {
    self.width
  }

  pub fn height(&self) -> usize {
    self.height
  }

  pub fn clear(&mut self) {
    self.cells.fill(Cell::default());
  }

  pub fn put<F: Into<Option<Color>>, B: Into<Option<Color>>>(
    &mut self,
    x: usize,
    y: usize,
    ch: char,
    fg: F,
    bg: B,
  ) {
    if x >= self.width || y >= self.height {
      return;
    }

    let tile = &mut self[x][y];

    tile.glyph = cp437_glyph(ch);

    if let Some(fg) = fg.into() {
      tile.foreground = fg;
    }

    if let Some(bg) = bg.into() {
      tile.background = bg;
    }
  }

  pub fn write<T: AsRef<str>, F: Into<Option<Color>>, B: Into<Option<Color>>>(
    &mut self,
    mut x: usize,
    y: usize,
    text: T,
    fg: F,
    bg: B,
  ) {
    let fg = fg.into();
    let bg = bg.into();

    for ch in text.as_ref().chars() {
      self.put(x, y, ch, fg, bg);
      x += 1;
    }
  }

  fn column_range(&self, x: usize) -> Range<usize> {
    let start = x * self.height;
    let end = start + self.height;

    start..end
  }
}

impl Index<usize> for Buffer {
  type Output = [Cell];

  fn index(&self, x: usize) -> &[Cell] {
    let range = self.column_range(x);
    &self.cells[range]
  }
}

impl IndexMut<usize> for Buffer {
  fn index_mut(&mut self, x: usize) -> &mut [Cell] {
    let range = self.column_range(x);
    &mut self.cells[range]
  }
}
