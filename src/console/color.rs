use sdl2::pixels::Color as SdlColor;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

impl Color {
  #[allow(non_snake_case)]
  pub const fn RGB(r: u8, g: u8, b: u8) -> Self {
    Color::RGBA(r, g, b, u8::MAX)
  }

  #[allow(non_snake_case)]
  pub const fn RGBA(r: u8, g: u8, b: u8, a: u8) -> Self {
    Color { r, g, b, a }
  }

  pub const TRANSPARENT: Color = Color::RGBA(0, 0, 0, 0);
  pub const BLACK: Color = Color::RGB(0, 0, 0);
  pub const MAROON: Color = Color::RGB(128, 0, 0);
  pub const GREEN: Color = Color::RGB(0, 128, 0);
  pub const OLIVE: Color = Color::RGB(128, 128, 0);
  pub const NAVY: Color = Color::RGB(0, 0, 128);
  pub const PURPLE: Color = Color::RGB(128, 0, 128);
  pub const TEAL: Color = Color::RGB(0, 128, 128);
  pub const SILVER: Color = Color::RGB(192, 192, 192);
  pub const GRAY: Color = Color::RGB(128, 128, 128);
  pub const RED: Color = Color::RGB(255, 0, 0);
  pub const LIME: Color = Color::RGB(0, 255, 0);
  pub const YELLOW: Color = Color::RGB(255, 255, 0);
  pub const BLUE: Color = Color::RGB(0, 0, 255);
  pub const FUCHSIA: Color = Color::RGB(255, 0, 255);
  pub const AQUA: Color = Color::RGB(0, 255, 255);
  pub const WHITE: Color = Color::RGB(255, 255, 255);
}

impl Default for Color {
  fn default() -> Self {
    Color::TRANSPARENT
  }
}

impl From<Color> for SdlColor {
  fn from(color: Color) -> SdlColor {
    SdlColor::RGBA(color.r, color.g, color.b, color.a)
  }
}

impl From<SdlColor> for Color {
  fn from(color: SdlColor) -> Color {
    Color::RGBA(color.r, color.g, color.b, color.a)
  }
}
