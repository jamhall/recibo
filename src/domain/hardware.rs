use std::fmt;

#[derive(Debug, Clone)]
pub enum Hardware {
  Init,
  Select,
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
