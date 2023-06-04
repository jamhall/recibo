use std::borrow::Cow;
use std::fmt;

#[cfg(feature = "graphics")]
use image::ImageError;

#[derive(Debug)]
pub enum PrinterError {
  Io(String),
  Input(String),
  Network(String),
  Configuration(String),
}

impl std::error::Error for PrinterError {}

impl fmt::Display for PrinterError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      PrinterError::Io(ref err) => write!(f, "IO error: {err}"),
      PrinterError::Network(ref err) => write!(f, "Network error: {err}"),
      PrinterError::Configuration(ref err) => write!(f, "Configuration error: {err}"),
      PrinterError::Input(ref err) => write!(f, "Input error: {err}"),
    }
  }
}

impl PrinterError {
  pub fn configuration<S: Into<String>>(s: S) -> Self {
    PrinterError::Configuration(s.into())
  }
  pub fn input<S: Into<String>>(s: S) -> Self {
    PrinterError::Input(s.into())
  }
}

pub type Result<T> = std::result::Result<T, PrinterError>;

impl From<std::io::Error> for PrinterError {
  fn from(err: std::io::Error) -> PrinterError {
    PrinterError::Io(err.to_string())
  }
}

impl From<Cow<'_, str>> for PrinterError {
  fn from(value: Cow<'_, str>) -> Self {
    PrinterError::Io(value.into_owned())
  }
}

#[cfg(feature = "graphics")]
impl From<ImageError> for PrinterError {
  fn from(err: ImageError) -> PrinterError {
    PrinterError::Io(err.to_string())
  }
}
