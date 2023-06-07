use sdl2::image::ImageRWops;
use sdl2::render::{Texture, TextureCreator};
use sdl2::rwops::RWops;
use sdl2::surface::Surface;

use crate::console::SdlError;

pub struct Font {
  surface: Surface<'static>,
}

impl Font {
  pub fn new(data: &[u8]) -> Result<Self, SdlError> {
    let rw = RWops::from_bytes(data)?;
    let surface = rw.load()?;
    Ok(Font { surface })
  }

  pub fn width(&self) -> u32 {
    self.surface.width() / 16
  }

  pub fn height(&self) -> u32 {
    self.surface.height() / 16
  }

  pub fn to_texture<'a, T>(&self, creator: &'a TextureCreator<T>) -> Result<Texture<'a>, SdlError> {
    let texture = self.surface.as_texture(creator)?;
    Ok(texture)
  }
}
