use std::fmt;

#[derive(Debug, Clone)]
pub enum UnderlineMode {
  None,
  Single,
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
