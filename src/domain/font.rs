use std::fmt;

#[derive(Debug, Clone)]
pub enum Font {
  A,
  B,
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
