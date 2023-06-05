use std::fmt;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Debug, Clone)]
pub enum Hardware {
  #[cfg_attr(feature = "serde", serde(rename = "init"))]
  Init,
  #[cfg_attr(feature = "serde", serde(rename = "select"))]
  Select,
  #[cfg_attr(feature = "serde", serde(rename = "reset"))]
  Reset,
}

impl fmt::Display for Hardware {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Hardware::Init => write!(f, "init"),
      Hardware::Select => write!(f, "select"),
      Hardware::Reset => write!(f, "reset"),
    }
  }
}

#[cfg(test)]
mod tests {

  #[test]
  #[cfg(feature = "serde")]
  fn test_deserialize_from_json() {
    let hardware: super::Hardware = serde_json::from_str("\"reset\"").unwrap();

    assert_eq!(hardware, super::Hardware::Reset);
  }
}
