use std::fmt;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Debug, Clone)]
pub enum Alignment {
  #[cfg_attr(feature = "serde", serde(rename = "left"))]
  Left,
  #[cfg_attr(feature = "serde", serde(rename = "center"))]
  Center,
  #[cfg_attr(feature = "serde", serde(rename = "right"))]
  Right,
}

impl fmt::Display for Alignment {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Alignment::Left => write!(f, "left"),
      Alignment::Center => write!(f, "center"),
      Alignment::Right => write!(f, "right"),
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  #[cfg(feature = "serde")]
  fn test_deserialize_from_json() {
    let alignment: super::Alignment = serde_json::from_str("\"right\"").unwrap();

    assert_eq!(alignment, super::Alignment::Right);
  }
}
