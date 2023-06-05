use std::fmt;
use std::fmt::Formatter;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Debug, Clone)]
pub enum BarcodeSystem {
  /// Universal Product Code (UPC) System A
  #[cfg_attr(feature = "serde", serde(rename = "upca"))]
  UpcA,

  /// Universal Product Code (UPC) System E
  #[cfg_attr(feature = "serde", serde(rename = "upce"))]
  UpcE,

  /// European Article Number (EAN) System 13
  #[cfg_attr(feature = "serde", serde(rename = "ean13"))]
  Ean13,

  /// European Article Number (EAN) System 8
  #[cfg_attr(feature = "serde", serde(rename = "ean8"))]
  Ean8,

  /// Code 39
  #[cfg_attr(feature = "serde", serde(rename = "code39"))]
  Code39,

  /// Interleaved 2 of 5 (ITF)
  #[cfg_attr(feature = "serde", serde(rename = "itf"))]
  Itf,

  #[cfg_attr(feature = "serde", serde(rename = "codebar"))]
  Codabar,
}

impl fmt::Display for BarcodeSystem {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      BarcodeSystem::UpcA => write!(f, "UPC-A"),
      BarcodeSystem::UpcE => write!(f, "UPC-E"),
      BarcodeSystem::Ean13 => write!(f, "EAN13"),
      BarcodeSystem::Ean8 => write!(f, "EAN8"),
      BarcodeSystem::Code39 => write!(f, "CODE39"),
      BarcodeSystem::Itf => write!(f, "ITF"),
      BarcodeSystem::Codabar => write!(f, "CODABAR"),
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Debug, Clone)]
pub enum BarcodeFont {
  #[cfg_attr(feature = "serde", serde(rename = "a"))]
  A,
  #[cfg_attr(feature = "serde", serde(rename = "b"))]
  B,
}

impl fmt::Display for BarcodeFont {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      BarcodeFont::A => write!(f, "A"),
      BarcodeFont::B => write!(f, "B"),
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum BarcodeTextPosition {
  #[cfg_attr(feature = "serde", serde(rename = "none"))]
  None,
  #[cfg_attr(feature = "serde", serde(rename = "above"))]
  Above,
  #[cfg_attr(feature = "serde", serde(rename = "below"))]
  Below,
  #[cfg_attr(feature = "serde", serde(rename = "both"))]
  Both,
}

impl From<&BarcodeTextPosition> for u8 {
  fn from(position: &BarcodeTextPosition) -> Self {
    match position {
      BarcodeTextPosition::None => 0,
      BarcodeTextPosition::Above => 1,
      BarcodeTextPosition::Below => 2,
      BarcodeTextPosition::Both => 3,
    }
  }
}

impl fmt::Display for BarcodeTextPosition {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      BarcodeTextPosition::None => write!(f, "none"),
      BarcodeTextPosition::Above => write!(f, "above"),
      BarcodeTextPosition::Below => write!(f, "below"),
      BarcodeTextPosition::Both => write!(f, "both"),
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Debug, Clone)]
pub struct Barcode {
  text_position: BarcodeTextPosition,
  system: BarcodeSystem,
  font: BarcodeFont,
  width: u8,
  height: u8,
  text: String,
}

impl fmt::Display for Barcode {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "Barcode: system: {}, width: {}, height: {}, text: {}",
      self.system, self.width, self.height, self.text
    )
  }
}

impl Barcode {
  pub fn new(
    system: BarcodeSystem,
    width: u8,
    height: u8,
    text: String,
    text_position: BarcodeTextPosition,
    font: BarcodeFont,
  ) -> Self {
    Self {
      text_position,
      system,
      font,
      width,
      height,
      text,
    }
  }

  pub fn system(&self) -> &BarcodeSystem {
    &self.system
  }

  pub fn width(&self) -> u8 {
    self.width
  }

  pub fn height(&self) -> u8 {
    self.height
  }

  pub fn text(&self) -> &str {
    &self.text
  }

  pub fn text_position(&self) -> &BarcodeTextPosition {
    &self.text_position
  }

  pub fn font(&self) -> &BarcodeFont {
    &self.font
  }

  pub fn builder() -> BarcodeBuilder {
    BarcodeBuilder::default()
  }
}

impl Default for Barcode {
  fn default() -> Self {
    Self::new(
      BarcodeSystem::UpcA,
      3,
      8,
      String::new(),
      BarcodeTextPosition::Below,
      BarcodeFont::A,
    )
  }
}

#[derive(Debug, Clone, Default)]
pub struct BarcodeBuilder(Barcode);

impl BarcodeBuilder {
  pub fn system(&mut self, system: BarcodeSystem) -> &mut Self {
    self.0.system = system;
    self
  }

  pub fn width(mut self, width: u8) -> Self {
    self.0.width = width;
    self
  }

  pub fn height(mut self, height: u8) -> Self {
    self.0.height = height;
    self
  }

  pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
    self.0.text = text.as_ref().to_string();
    self
  }

  pub fn text_position(&mut self, position: BarcodeTextPosition) -> &mut Self {
    self.0.text_position = position;
    self
  }

  pub fn font(&mut self, font: BarcodeFont) -> &mut Self {
    self.0.font = font;
    self
  }

  pub fn build(self) -> Barcode {
    self.0
  }
}

#[cfg(test)]
mod tests {

  #[test]
  #[cfg(feature = "serde")]
  fn test_deserialize_from_json() {
    let json = r#"
    {
      "system": "upca",
      "width": 3,
      "height": 8,
      "text": "123456789012",
      "text_position": "below",
      "font": "a"
    }
    "#;

    let barcode: super::Barcode = serde_json::from_str(json).unwrap();

    assert_eq!(barcode.system(), &super::BarcodeSystem::UpcA);
    assert_eq!(barcode.width(), 3);
    assert_eq!(barcode.height(), 8);
    assert_eq!(barcode.text(), "123456789012");
    assert_eq!(barcode.text_position(), &super::BarcodeTextPosition::Below);
    assert_eq!(barcode.font(), &super::BarcodeFont::A);
  }
}
