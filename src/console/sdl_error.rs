use std::error::Error;
use std::fmt::{self, Display, Formatter};

use sdl2::render::{TargetRenderError, TextureValueError};
use sdl2::video::WindowBuildError;
use sdl2::IntegerOrSdlError;

#[derive(Debug)]
pub struct SdlError {
  message: String,
}

impl Display for SdlError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "SDL error: {}", self.message)
  }
}

impl Error for SdlError {}

impl From<String> for SdlError {
  fn from(message: String) -> Self {
    SdlError { message }
  }
}

impl From<TextureValueError> for SdlError {
  fn from(err: TextureValueError) -> Self {
    if let TextureValueError::SdlError(message) = err {
      SdlError { message }
    } else {
      SdlError { message: err.to_string() }
    }
  }
}

impl From<WindowBuildError> for SdlError {
  fn from(err: WindowBuildError) -> Self {
    if let WindowBuildError::SdlError(message) = err {
      SdlError { message }
    } else {
      SdlError { message: err.to_string() }
    }
  }
}

impl From<IntegerOrSdlError> for SdlError {
  fn from(err: IntegerOrSdlError) -> Self {
    if let IntegerOrSdlError::SdlError(message) = err {
      SdlError { message }
    } else {
      SdlError { message: err.to_string() }
    }
  }
}

impl From<TargetRenderError> for SdlError {
  fn from(err: TargetRenderError) -> Self {
    if let TargetRenderError::SdlError(err) = err {
      #[allow(deprecated)]
      SdlError { message: err.description().to_owned() }
    } else {
      SdlError { message: err.to_string() }
    }
  }
}
