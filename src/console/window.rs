use std::ops::{Deref, DerefMut};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Canvas};
use sdl2::video::Window as SdlWindow;
use sdl2::EventPump;

use crate::console::{Buffer, Font, SdlError, SdlEventQueue};

pub struct Window {
  buffer: Buffer,
  canvas: Canvas<SdlWindow>,
  font: Font,
  events: EventPump,
}

impl Window {
  pub fn new<T: AsRef<str>>(
    title: T,
    width: usize,
    height: usize,
    font: Font,
  ) -> Result<Self, SdlError> {
    let sdl = sdl2::init()?;
    let video = sdl.video()?;

    let window_width = width as u32 * font.width();
    let window_height = height as u32 * font.height();
    let window = video
      .window(title.as_ref(), window_width, window_height)
      .resizable()
      .allow_highdpi()
      .build()?;
    let canvas = window.into_canvas().accelerated().present_vsync().target_texture().build()?;

    let buffer = Buffer::new(width, height);
    let events = sdl.event_pump()?;

    Ok(Window { buffer, canvas, font, events })
  }

  pub fn update(&mut self) -> Result<(), SdlError> {
    let texture_creator = self.canvas.texture_creator();
    let mut font_buffer = self.font.to_texture(&texture_creator)?;
    font_buffer.set_blend_mode(BlendMode::Blend);

    let pixel_format = self.canvas.window().window_pixel_format();
    let buffer_width = self.buffer.width() as u32 * self.font.width();
    let buffer_height = self.buffer.height() as u32 * self.font.height();

    let mut buffer =
      texture_creator.create_texture_target(pixel_format, buffer_width, buffer_height)?;
    buffer.set_blend_mode(BlendMode::None);

    self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
    self.canvas.clear();

    let mut render_error: Result<(), SdlError> = Ok(());
    self.canvas.with_texture_canvas(&mut buffer, |c| {
      c.set_draw_color(Color::RGBA(0, 0, 0, 0));
      c.clear();

      render_error = (|c: &mut Canvas<SdlWindow>| {
        for y in 0..self.buffer.height() {
          for x in 0..self.buffer.width() {
            let cell = self.buffer[x][y];
            let cell_rect = Rect::new(
              x as i32 * self.font.width() as i32,
              y as i32 * self.font.height() as i32,
              self.font.width(),
              self.font.height(),
            );
            let glyph_rect = Rect::new(
              (cell.glyph & 0xf) as i32 * self.font.width() as i32,
              ((cell.glyph >> 4) & 0xf) as i32 * self.font.height() as i32,
              self.font.width(),
              self.font.height(),
            );

            c.set_draw_color(cell.background);
            c.fill_rect(cell_rect)?;

            font_buffer.set_color_mod(cell.foreground.r, cell.foreground.g, cell.foreground.b);
            font_buffer.set_alpha_mod(cell.foreground.a);
            c.copy(&font_buffer, glyph_rect, cell_rect)?;
          }
        }

        Ok(())
      })(c);
    })?;
    render_error?;

    self.canvas.copy(&buffer, None, None)?;
    self.canvas.present();

    Ok(())
  }

  pub fn events(&mut self) -> SdlEventQueue {
    SdlEventQueue::new(&mut self.events)
  }
}

impl Deref for Window {
  type Target = Buffer;

  fn deref(&self) -> &Buffer {
    &self.buffer
  }
}

impl DerefMut for Window {
  fn deref_mut(&mut self) -> &mut Buffer {
    &mut self.buffer
  }
}
