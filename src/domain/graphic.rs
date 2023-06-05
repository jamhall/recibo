use std::fmt;

#[cfg(feature = "graphics")]
use image::{DynamicImage, GenericImageView, Rgba};

#[cfg(feature = "serde")]
use serde::{de, Deserializer};

#[cfg(feature = "graphics")]
use crate::error::{PrinterError, Result};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum GraphicSize {
  #[cfg_attr(feature = "serde", serde(rename = "normal"))]
  Normal,
  #[cfg_attr(feature = "serde", serde(rename = "double_width"))]
  DoubleWidth,
  #[cfg_attr(feature = "serde", serde(rename = "double_height"))]
  DoubleHeight,
  #[cfg_attr(feature = "serde", serde(rename = "double_width_and_height"))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub struct Graphic {
  path: String,
  #[cfg_attr(feature = "serde", serde(skip_serializing, skip_deserializing))]
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

#[cfg(feature = "serde")]
struct GraphicVisitor;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for GraphicVisitor {
  type Value = Graphic;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("struct Graphic")
  }

  fn visit_map<M>(self, mut access: M) -> std::result::Result<Graphic, M::Error>
  where
    M: serde::de::MapAccess<'de>,
  {
    let mut path: Option<String> = None;
    let mut density = None;
    let mut max_width = None;
    let mut size = None;

    while let Some(key) = access.next_key()? {
      match key {
        "path" => {
          if path.is_some() {
            return Err(de::Error::duplicate_field("path"));
          }
          path = Some(access.next_value()?);
        }
        "density" => {
          if density.is_some() {
            return Err(de::Error::duplicate_field("density"));
          }
          density = Some(access.next_value()?);
        }
        "max_width" => {
          if max_width.is_some() {
            return Err(de::Error::duplicate_field("max_width"));
          }
          max_width = Some(access.next_value()?);
        }
        "size" => {
          if size.is_some() {
            return Err(de::Error::duplicate_field("size"));
          }
          size = Some(access.next_value()?);
        }
        _ => {
          return Err(de::Error::unknown_field(
            key,
            &["path", "density", "max_width", "size"],
          ));
        }
      }
    }
    let path = path.ok_or_else(|| de::Error::missing_field("path"))?;
    let density = density.ok_or_else(|| de::Error::missing_field("density"))?;
    let max_width = max_width.ok_or_else(|| de::Error::missing_field("max_width"))?;
    let size = size.ok_or_else(|| de::Error::missing_field("size"))?;

    if let Ok(graphic) = Graphic::new(path.clone(), density, max_width, size) {
      Ok(graphic)
    } else {
      Err(de::Error::custom(format!(
        "Could not load graphic at path: {}",
        path
      )))
    }
  }
}

#[cfg(feature = "serde")]
impl<'de> serde::de::Deserialize<'de> for Graphic {
  fn deserialize<D>(deserializer: D) -> std::result::Result<Graphic, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_map(GraphicVisitor)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_graphic_size() {
    let normal = GraphicSize::Normal;
    let double_width = GraphicSize::DoubleWidth;
    let double_height = GraphicSize::DoubleHeight;
    let double_width_and_height = GraphicSize::DoubleWidthAndHeight;

    assert_eq!(u8::from(&normal), 0x00);
    assert_eq!(u8::from(&double_width), 0x01);
    assert_eq!(u8::from(&double_height), 0x02);
    assert_eq!(u8::from(&double_width_and_height), 0x03);
  }

  #[test]
  #[cfg(feature = "serde")]
  fn test_serialize_from_json() -> Result<()> {
    let json = r#"
      {
        "path": "resources/rust-logo-small.png",
        "density": 8,
        "max_width": 512,
        "size": "normal"
      }
    "#;
    let graphic: Graphic = serde_json::from_str(json).unwrap();

    assert_eq!(graphic.path(), "resources/rust-logo-small.png");
    assert_eq!(graphic.density(), 8);
    assert_eq!(graphic.max_width(), 512);
    assert_eq!(graphic.width(), 200);
    assert_eq!(graphic.height(), 200);
    assert_eq!(graphic.size(), &GraphicSize::Normal);
    Ok(())
  }
}
