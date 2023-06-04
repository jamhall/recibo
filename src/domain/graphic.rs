use std::fmt;

#[cfg(feature = "graphics")]
use image::{DynamicImage, GenericImageView, Rgba};

#[cfg(feature = "graphics")]
use crate::error::{PrinterError, Result};

#[derive(Debug, Clone)]
pub enum GraphicSize {
  Normal,
  DoubleWidth,
  DoubleHeight,
  DoubleWidthAndHeight,
}

impl fmt::Display for GraphicSize {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      GraphicSize::Normal => write!(f, "Normal"),
      GraphicSize::DoubleWidth => write!(f, "DoubleWidth"),
      GraphicSize::DoubleHeight => write!(f, "DoubleHeight"),
      GraphicSize::DoubleWidthAndHeight => write!(f, "DoubleWidthAndHeight"),
    }
  }
}

impl From<&GraphicSize> for u8 {
  fn from(size: &GraphicSize) -> Self {
    match size {
      GraphicSize::Normal => 0x00,
      GraphicSize::DoubleWidth => 0x01,
      GraphicSize::DoubleHeight => 0x02,
      GraphicSize::DoubleWidthAndHeight => 0x03,
    }
  }
}

#[cfg(feature = "graphics")]
pub struct Graphic {
  path: String,
  img: DynamicImage,
  density: u8,
  max_width: u32,
  size: GraphicSize,
}

#[cfg(feature = "graphics")]
impl Graphic {
  pub fn new(path: String, density: u8, max_width: u32, size: GraphicSize) -> Result<Self> {
    let img = image::open(&path)?;
    let img = if img.width() > max_width {
      let resized = img.resize(max_width, max_width, image::imageops::Nearest);
      resized.grayscale()
    } else {
      img.grayscale()
    };
    Ok(Self {
      path,
      img,
      density,
      max_width,
      size,
    })
  }

  pub fn width(&self) -> u16 {
    self.img.width() as u16
  }

  pub fn height(&self) -> u16 {
    self.img.height() as u16
  }

  #[allow(clippy::cast_sign_loss)]
  pub fn width_bytes(&self) -> u16 {
    (f32::from(self.width()) / 8.0).ceil() as u16
  }

  #[allow(clippy::cast_sign_loss)]
  pub fn height_bytes(&self) -> u16 {
    (f32::from(self.height()) / 8.0).ceil() as u16
  }

  pub fn img(&self) -> &DynamicImage {
    &self.img
  }

  pub fn dimensions(&self) -> (u16, u16) {
    (self.width(), self.height())
  }

  pub fn pixel(&self, x: u32, y: u32) -> Rgba<u8> {
    self.img.get_pixel(x, y)
  }

  pub fn density(&self) -> u8 {
    self.density
  }

  pub fn path(&self) -> &str {
    &self.path
  }

  pub fn size(&self) -> &GraphicSize {
    &self.size
  }

  pub fn max_width(&self) -> u32 {
    self.max_width
  }

  pub fn builder() -> GraphicBuilder {
    GraphicBuilder::default()
  }
}

#[cfg(feature = "graphics")]
pub struct GraphicBuilder {
  path: Option<String>,
  density: u8,
  max_width: u32,
  size: GraphicSize,
}

#[cfg(feature = "graphics")]
impl Default for GraphicBuilder {
  fn default() -> Self {
    Self {
      path: None,
      density: 8,
      max_width: 512,
      size: GraphicSize::Normal,
    }
  }
}

#[cfg(feature = "graphics")]
impl GraphicBuilder {
  pub fn path<T: AsRef<str>>(&mut self, path: T) -> &mut Self {
    let path = path.as_ref().to_string();
    self.path = Some(path);
    self
  }

  pub fn density(mut self, density: u8) -> Self {
    self.density = density;
    self
  }

  pub fn size(&mut self, size: GraphicSize) -> &mut Self {
    self.size = size;
    self
  }

  pub fn max_width(&mut self, max_width: u32) -> &mut Self {
    self.max_width = max_width;
    self
  }

  pub fn build(self) -> Result<Graphic> {
    let path = self.path.ok_or(PrinterError::input("No path provided"))?;
    let graphic = Graphic::new(path, self.density, self.max_width, self.size)?;
    Ok(graphic)
  }
}
