use std::fmt;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Debug, Clone)]
pub enum UnderlineMode {
  #[cfg_attr(feature = "serde", serde(rename = "none"))]
  None,
  #[cfg_attr(feature = "serde", serde(rename = "single"))]
  Single,
  #[cfg_attr(feature = "serde", serde(rename = "double"))]
  Double,
}

impl fmt::Display for UnderlineMode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      UnderlineMode::None => write!(f, "none"),
      UnderlineMode::Single => write!(f, "single"),
      UnderlineMode::Double => write!(f, "double"),
    }
  }
}

#[cfg(test)]
mod tests {

  #[test]
  #[cfg(feature = "serde")]
  fn test_deserialize_from_json() {
    let alignment: super::UnderlineMode = serde_json::from_str("\"single\"").unwrap();

    assert_eq!(alignment, super::UnderlineMode::Single);
  }
}
