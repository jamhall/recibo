use std::fmt;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Debug, Clone)]
pub enum Font {
  #[cfg_attr(feature = "serde", serde(rename = "a"))]
  A,
  #[cfg_attr(feature = "serde", serde(rename = "b"))]
  B,
  #[cfg_attr(feature = "serde", serde(rename = "c"))]
  C,
}

impl fmt::Display for Font {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Font::A => write!(f, "A"),
      Font::B => write!(f, "B"),
      Font::C => write!(f, "C"),
    }
  }
}

#[cfg(test)]
mod tests {

  #[test]
  #[cfg(feature = "serde")]
  fn test_deserialize_from_json() {
    let font: super::Font = serde_json::from_str("\"a\"").unwrap();

    assert_eq!(font, super::Font::A);
  }
}
