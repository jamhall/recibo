use std::fmt;

#[derive(Debug, Clone)]
pub enum Alignment {
  Left,
  Center,
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
