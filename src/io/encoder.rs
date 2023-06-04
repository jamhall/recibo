use encoding::{EncoderTrap, EncodingRef};
use log::debug;

use crate::error::Result;

pub struct Encoder {
  codec: EncodingRef,
  trap: EncoderTrap,
}

impl Default for Encoder {
  fn default() -> Self {
    Encoder {
      codec: encoding::all::UTF_8,
      trap: EncoderTrap::Replace,
    }
  }
}

impl Encoder {
  pub fn new(codec: EncodingRef, trap: EncoderTrap) -> Self {
    debug!("Creating encoder for {}", codec.name());
    Self { codec, trap }
  }

  pub fn encode(&self, data: &str) -> Result<Vec<u8>> {
    debug!("Encoding data");
    self.codec.encode(data, self.trap).map_err(Into::into)
  }
}
