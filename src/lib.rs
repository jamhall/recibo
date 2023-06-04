#![deny(clippy::all, clippy::pedantic)]
#![allow(
  dead_code,
  clippy::module_name_repetitions,
  clippy::unused_self,
  clippy::needless_pass_by_value,
  clippy::cast_possible_truncation,
  clippy::must_use_candidate,
  clippy::missing_errors_doc,
  clippy::return_self_not_must_use
)]

extern crate core;

pub use domain::*;
pub use error::*;
pub use io::*;
pub use printer::*;

mod domain;
mod error;
mod io;
mod printer;
