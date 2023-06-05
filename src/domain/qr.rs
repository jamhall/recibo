use std::fmt;

use crate::io::constants;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum QrModel {
  #[cfg_attr(feature = "serde", serde(rename = "model1"))]
  Model1,
  #[cfg_attr(feature = "serde", serde(rename = "model2"))]
  Model2,
}

impl fmt::Display for QrModel {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      QrModel::Model1 => write!(f, "model1"),
      QrModel::Model2 => write!(f, "model2"),
    }
  }
}

impl From<&QrModel> for u8 {
  fn from(model: &QrModel) -> Self {
    match model {
      QrModel::Model1 => constants::QR_MODEL_1,
      QrModel::Model2 => constants::QR_MODEL_2,
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum QrCorrectionLevel {
  #[cfg_attr(feature = "serde", serde(rename = "low"))]
  Low,
  #[cfg_attr(feature = "serde", serde(rename = "medium"))]
  Medium,
  #[cfg_attr(feature = "serde", serde(rename = "quartile"))]
  Quartile,
  #[cfg_attr(feature = "serde", serde(rename = "high"))]
  High,
}

impl fmt::Display for QrCorrectionLevel {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      QrCorrectionLevel::Medium => write!(f, "l"),
      QrCorrectionLevel::Low => write!(f, "m"),
      QrCorrectionLevel::Quartile => write!(f, "q"),
      QrCorrectionLevel::High => write!(f, "h"),
    }
  }
}

impl From<&QrCorrectionLevel> for u8 {
  fn from(correction_level: &QrCorrectionLevel) -> Self {
    match correction_level {
      QrCorrectionLevel::Low => constants::QR_CORRECTION_ERROR_LEVEL_LOW,
      QrCorrectionLevel::Medium => constants::QR_CORRECTION_ERROR_LEVEL_MEDIUM,
      QrCorrectionLevel::Quartile => constants::QR_CORRECTION_ERROR_LEVEL_QUARTILE,
      QrCorrectionLevel::High => constants::QR_CORRECTION_ERROR_LEVEL_HIGH,
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Debug, Clone)]
pub struct Qr {
  model: QrModel,
  correction_level: QrCorrectionLevel,
  size: u8,
  text: String,
}

impl fmt::Display for Qr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "model: {}, correction_level: {}, size: {}, text: {}",
      self.model, self.correction_level, self.size, self.text
    )
  }
}

impl Qr {
  pub fn new(model: QrModel, correction_level: QrCorrectionLevel, size: u8, text: String) -> Self {
    Self {
      model,
      correction_level,
      size,
      text,
    }
  }

  pub fn model(&self) -> &QrModel {
    &self.model
  }

  pub fn correction_level(&self) -> &QrCorrectionLevel {
    &self.correction_level
  }

  pub fn size(&self) -> u8 {
    self.size
  }

  pub fn text(&self) -> &str {
    &self.text
  }

  pub fn builder() -> QrBuilder {
    QrBuilder::default()
  }
}

impl Default for Qr {
  fn default() -> Self {
    Self::new(QrModel::Model1, QrCorrectionLevel::Medium, 8, String::new())
  }
}

#[derive(Debug, Clone, Default)]
pub struct QrBuilder(Qr);

impl QrBuilder {
  pub fn model(&mut self, model: QrModel) -> &mut Self {
    self.0.model = model;
    self
  }

  pub fn correction_level(&mut self, correction_level: QrCorrectionLevel) -> &mut Self {
    self.0.correction_level = correction_level;
    self
  }

  pub fn size(&mut self, size: u8) -> &mut Self {
    self.0.size = size;
    self
  }

  pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
    self.0.text = text.as_ref().to_string();
    self
  }

  pub fn build(self) -> Qr {
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
        "model": "model1",
        "correction_level": "medium",
        "size": 8,
        "text": "Hello World!"
      }
      "#;

    let qr: super::Qr = serde_json::from_str(json).unwrap();

    assert_eq!(qr.model(), &super::QrModel::Model1);
    assert_eq!(qr.correction_level(), &super::QrCorrectionLevel::Medium);
    assert_eq!(qr.size(), 8);
    assert_eq!(qr.text(), "Hello World!");
  }
}
