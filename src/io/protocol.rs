use std::io::Write;

use barcoders::sym::codabar::Codabar;
use barcoders::sym::code39::Code39;
use barcoders::sym::ean13::{EAN13, UPCA};
use barcoders::sym::ean8::EAN8;
use barcoders::sym::tf::TF;
use byteorder::{LittleEndian, WriteBytesExt};

#[cfg(feature = "graphics")]
use crate::domain::Graphic;
use crate::domain::{
  Alignment, Barcode, BarcodeFont, BarcodeSystem, Font, Hardware, Qr, UnderlineMode,
};
use crate::error::{PrinterError, Result};
use crate::io::{constants, Encoder};

#[derive(Default)]
pub struct Protocol {
  encoder: Encoder,
}

impl Protocol {
  pub fn new(encoder: Encoder) -> Self {
    Self { encoder }
  }

  pub fn hardware(&mut self, hardware: Hardware) -> &[u8] {
    match hardware {
      Hardware::Init => constants::HARDWARE_INIT,
      Hardware::Select => constants::HARDWARE_SELECT,
      Hardware::Reset => constants::HARDWARE_RESET,
    }
  }

  pub fn font(&mut self, font: Font) -> &[u8] {
    match font {
      Font::A => constants::FONT_A,
      Font::B => constants::FONT_B,
      Font::C => constants::FONT_C,
    }
  }

  pub fn align(&mut self, alignment: Alignment) -> &[u8] {
    match alignment {
      Alignment::Left => constants::TEXT_JUSTIFY_LEFT,
      Alignment::Center => constants::TEXT_JUSTIFY_CENTER,
      Alignment::Right => constants::TEXT_JUSTIFY_RIGHT,
    }
  }

  pub fn doublestrike(&mut self, enabled: bool) -> &[u8] {
    if enabled {
      constants::TEXT_DOUBLESTRIKE_ON
    } else {
      constants::TEXT_DOUBLESTRIKE_OFF
    }
  }

  pub fn linespacing(&mut self, height: u8) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    buffer.write_all(constants::TEXT_LINESPACING)?;
    buffer.write_u8(height)?;
    Ok(buffer)
  }

  pub fn flip(&mut self, enabled: bool) -> &[u8] {
    if enabled {
      constants::TEXT_FLIP_ON
    } else {
      constants::TEXT_FLIP_OFF
    }
  }

  pub fn reset_linespacing(&mut self) -> &[u8] {
    constants::TEXT_RESET_LINESPACING
  }

  pub fn bold(&mut self, enabled: bool) -> &[u8] {
    if enabled {
      constants::TEXT_BOLD_MODE_ON
    } else {
      constants::TEXT_BOLD_MODE_OFF
    }
  }

  pub fn underline(&mut self, mode: UnderlineMode) -> &[u8] {
    match mode {
      UnderlineMode::None => constants::TEXT_UNDERLINE_MODE_OFF,
      UnderlineMode::Single => constants::TEXT_UNDERLINE_MODE_ON,
      UnderlineMode::Double => constants::TEXT_UNDERLINE_MODE_2_ON,
    }
  }

  pub fn text_size(&mut self, width_multiplier: u8, height_multiplier: u8) -> Result<Vec<u8>> {
    let validate = |multiplier: u8, name: &str| -> Result<()> {
      if (1..=8).contains(&multiplier) {
        Ok(())
      } else {
        Err(PrinterError::input(format!("Invalid {name} multiplier")))
      }
    };

    validate(width_multiplier, "width")?;
    validate(height_multiplier, "height")?;

    let mut buffer = Vec::new();
    buffer.write_all(constants::TEXT_SIZE_SELECT)?;
    buffer.write_u8((2 << 3) * (width_multiplier - 1) + (height_multiplier - 1))?;

    Ok(buffer)
  }

  pub fn reset(&mut self) -> &[u8] {
    constants::HARDWARE_RESET
  }

  pub fn init(&mut self) -> &[u8] {
    constants::HARDWARE_INIT
  }

  pub fn feed(&mut self, n: u8) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    buffer.write_all(constants::PAPER_FEED_FORWARD)?;
    buffer.write_u8(n)?;
    Ok(buffer)
  }

  pub fn reverse_feed(&mut self, n: u8) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    buffer.write_all(constants::PAPER_FEED_REVERSE)?;
    buffer.write_u8(n)?;
    Ok(buffer)
  }

  pub fn reverse_colours(&self, enabled: bool) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    if enabled {
      buffer.write_all(constants::TEXT_REVERSE_COLOURS_ON)?;
    } else {
      buffer.write_all(constants::TEXT_REVERSE_COLOURS_OFF)?;
    }
    Ok(buffer)
  }

  pub fn left(&self, dots: u16) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    buffer.write_all(constants::TEXT_MARGIN_LEFT)?;
    buffer.write_u16::<LittleEndian>(dots)?;
    Ok(buffer)
  }

  pub fn width(&self, dots: u16) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    buffer.write_all(constants::TEXT_PRINTABLE_AREA)?;
    buffer.write_u16::<LittleEndian>(dots)?;
    Ok(buffer)
  }

  pub fn cut(&mut self, partial: bool) -> Result<Vec<u8>> {
    let cut = if partial {
      constants::PAPER_CUT_PARTIAL
    } else {
      constants::PAPER_CUT_FULL
    };

    Ok(cut.to_vec())
  }

  pub fn print(&mut self, text: &str) -> Result<Vec<u8>> {
    let encoded = self.encoder.encode(text)?;
    Ok(encoded)
  }

  pub fn println(&mut self, text: &str) -> Result<Vec<u8>> {
    let feed = self.feed(1)?;
    let print = self.print(text)?;
    let mut buffer = Vec::new();
    buffer.write_all(feed.as_slice())?;
    buffer.write_all(print.as_slice())?;

    Ok(buffer)
  }

  pub fn text(&mut self, text: &str) -> Result<Vec<u8>> {
    self.println(text)
  }

  pub fn qr(&mut self, qr: Qr) -> Result<Vec<u8>> {
    let level = qr.correction_level().into();
    let model = qr.model().into();
    let size = qr.size();
    let text = qr.text();

    let mut buffer = Vec::new();

    // select the QR code model
    buffer.write_all(constants::QR_SELECT_MODEL)?;
    buffer.write_u8(model)?;

    // set the size of the QR code
    buffer.write_all(constants::QR_LEVEL)?;
    buffer.write_u8(size)?;

    // set the error correction level
    buffer.write_all(constants::QR_CORRECTION_ERROR_LEVEL)?;
    buffer.write_u8(level)?;

    // store the data
    buffer.write_all(constants::QR_DATA_STORE_PREFIX)?;
    buffer.write_u16::<LittleEndian>((text.len() + 3) as u16)?;
    buffer.write_all(constants::QR_DATA_STORE_SUFFIX)?;
    buffer.write_all(text.as_bytes())?;

    // print the QR code
    buffer.write_all(constants::QR_PRINT)?;

    Ok(buffer)
  }

  pub fn barcode(&mut self, barcode: Barcode) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();

    let (system, encoded) = match barcode.system() {
      BarcodeSystem::UpcA => {
        let encoder =
          UPCA::new(barcode.text()).map_err(|_| PrinterError::input("Invalid UPCA barcode"))?;
        (constants::BARCODE_TYPE_UPC_A, encoder.encode())
      }
      BarcodeSystem::UpcE => {
        let encoder =
          UPCA::new(barcode.text()).map_err(|_| PrinterError::input("Invalid UPCA barcode"))?;
        (constants::BARCODE_TYPE_UPC_E, encoder.encode())
      }
      BarcodeSystem::Ean13 => {
        let encoder =
          EAN13::new(barcode.text()).map_err(|_| PrinterError::input("Invalid EAN13 barcode"))?;
        (constants::BARCODE_TYPE_JAN13_EAN13, encoder.encode())
      }
      BarcodeSystem::Ean8 => {
        let encoder =
          EAN8::new(barcode.text()).map_err(|_| PrinterError::input("Invalid EAN8 barcode"))?;
        (constants::BARCODE_TYPE_JAN8_EAN8, encoder.encode())
      }
      BarcodeSystem::Code39 => {
        let encoder =
          Code39::new(barcode.text()).map_err(|_| PrinterError::input("Invalid Code39 barcode"))?;
        (constants::BARCODE_TYPE_CODE39, encoder.encode())
      }
      BarcodeSystem::Itf => {
        let encoder = TF::interleaved(barcode.text())
          .map_err(|_| PrinterError::input("Invalid ITF barcode"))?;
        (constants::BARCODE_TYPE_ITF, encoder.encode())
      }
      BarcodeSystem::Codabar => {
        let encoder = Codabar::new(barcode.text())
          .map_err(|_| PrinterError::input("Invalid Codabar barcode"))?;
        (constants::BARCODE_TYPE_CODABAR, encoder.encode())
      }
    };

    let font = match barcode.font() {
      BarcodeFont::A => constants::BARCODE_FONT_A,
      BarcodeFont::B => constants::BARCODE_FONT_B,
    };

    let text_position = barcode.text_position().into();

    // set the barcode system
    buffer.write_all(system)?;

    // set the height of the barcode
    buffer.write_all(constants::BARCODE_HEIGHT)?;
    buffer.write_u8(0x68)?;

    // set the width of the barcode
    buffer.write_all(constants::BARCODE_WIDTH)?;
    buffer.write_u8(0x02)?;

    // Set the barcode font
    buffer.write_all(font)?;

    // set the text position
    buffer.write_all(constants::BARCODE_TEXT_POSITION)?;
    buffer.write_u8(text_position)?;

    // set the barcode data
    buffer.write_all(encoded.as_slice())?;

    // terminate the barcode data with a null character
    buffer.push(constants::NIL);

    Ok(buffer)
  }

  #[cfg(feature = "graphics")]
  pub fn graphic(&mut self, graphic: Graphic) -> Result<Vec<u8>> {
    let mut buffer: Vec<u8> = Vec::new();

    // write command header
    buffer.write_all(constants::IMAGE_HEADER)?;

    buffer.write_u8(graphic.size().into())?;

    // set the width and height
    buffer.write_u16::<LittleEndian>(graphic.width_bytes())?;
    buffer.write_u16::<LittleEndian>(graphic.height())?;

    let density = graphic.density();
    let (width, height) = graphic.dimensions();

    // Iterating over the height and width of the graphic
    for y in 0..height {
      for x in (0..width).step_by(density as usize) {
        let mut byte = 0u8;

        // Processing 8 bits per byte
        for bit in 0..8 {
          let x_offset = x + bit;

          // Breaking the loop if x_offset exceeds the width
          if x_offset >= width {
            break;
          }

          let is_black = graphic.pixel(u32::from(x_offset), u32::from(y)).0[0] <= 128;

          // Shift byte to the left, adding the pixel value at the end
          byte = (byte << 1) | u8::from(is_black);
        }

        // Write the constructed byte to the buffer
        buffer.write_u8(byte)?;
      }
    }

    Ok(buffer)
  }
}

#[cfg(test)]
mod tests {
  use crate::BarcodeTextPosition;

  use super::*;

  macro_rules! join {
    ($base:expr, $($byte:expr),*) => {{
        let mut result = $base.to_vec();
        $(result.push($byte);)*
        result
    }};
  }

  #[test]
  fn test_feed() -> Result<()> {
    let mut protocol = Protocol::default();
    assert_eq!(
      protocol.feed(1)?,
      join!(constants::PAPER_FEED_FORWARD, 0x01)
    );
    assert_eq!(
      protocol.feed(5)?,
      join!(constants::PAPER_FEED_FORWARD, 0x05)
    );

    Ok(())
  }

  #[test]
  fn test_reverse_feed() -> Result<()> {
    let mut protocol = Protocol::default();
    assert_eq!(
      protocol.reverse_feed(1)?,
      join!(constants::PAPER_FEED_REVERSE, 0x01)
    );
    assert_eq!(
      protocol.reverse_feed(5)?,
      join!(constants::PAPER_FEED_REVERSE, 0x05)
    );
    Ok(())
  }

  #[test]
  fn test_font() -> Result<()> {
    let mut protocol = Protocol::default();
    assert_eq!(protocol.font(Font::A), constants::FONT_A);
    assert_eq!(protocol.font(Font::B), constants::FONT_B);
    assert_eq!(protocol.font(Font::C), constants::FONT_C);
    Ok(())
  }

  #[test]
  fn test_align() -> Result<()> {
    let mut protocol = Protocol::default();
    assert_eq!(
      protocol.align(Alignment::Left),
      constants::TEXT_JUSTIFY_LEFT
    );
    assert_eq!(
      protocol.align(Alignment::Center),
      constants::TEXT_JUSTIFY_CENTER
    );
    assert_eq!(
      protocol.align(Alignment::Right),
      constants::TEXT_JUSTIFY_RIGHT
    );
    Ok(())
  }

  #[test]
  fn test_doublestrike() -> Result<()> {
    let mut protocol = Protocol::default();
    assert_eq!(protocol.doublestrike(true), constants::TEXT_DOUBLESTRIKE_ON);
    assert_eq!(
      protocol.doublestrike(false),
      constants::TEXT_DOUBLESTRIKE_OFF
    );
    Ok(())
  }

  #[test]
  fn test_linespacing() -> Result<()> {
    let mut protocol = Protocol::default();
    assert_eq!(
      protocol.linespacing(1)?,
      join!(constants::TEXT_LINESPACING, 0x01)
    );
    assert_eq!(
      protocol.linespacing(5)?,
      join!(constants::TEXT_LINESPACING, 0x05)
    );
    Ok(())
  }

  #[test]
  fn test_flip() -> Result<()> {
    let mut protocol = Protocol::default();
    assert_eq!(protocol.flip(true), constants::TEXT_FLIP_ON);
    assert_eq!(protocol.flip(false), constants::TEXT_FLIP_OFF);
    Ok(())
  }

  #[test]
  fn test_reset_linespacing() -> Result<()> {
    let mut protocol = Protocol::default();
    assert_eq!(
      protocol.reset_linespacing(),
      constants::TEXT_RESET_LINESPACING
    );
    Ok(())
  }

  #[test]
  fn test_bold() -> Result<()> {
    let mut protocol = Protocol::default();
    assert_eq!(protocol.bold(true), constants::TEXT_BOLD_MODE_ON);
    assert_eq!(protocol.bold(false), constants::TEXT_BOLD_MODE_OFF);
    Ok(())
  }

  #[test]
  fn test_underline() -> Result<()> {
    let mut protocol = Protocol::default();
    assert_eq!(
      protocol.underline(UnderlineMode::None),
      constants::TEXT_UNDERLINE_MODE_OFF
    );
    assert_eq!(
      protocol.underline(UnderlineMode::Single),
      constants::TEXT_UNDERLINE_MODE_ON
    );
    assert_eq!(
      protocol.underline(UnderlineMode::Double),
      constants::TEXT_UNDERLINE_MODE_2_ON
    );
    Ok(())
  }

  #[test]
  fn test_text_size() -> Result<()> {
    let mut protocol = Protocol::default();
    assert_eq!(
      protocol.text_size(4, 4)?,
      join!(constants::TEXT_SIZE_SELECT, 0x33)
    );
    assert_eq!(
      protocol.text_size(8, 2)?,
      join!(constants::TEXT_SIZE_SELECT, 0x71)
    );
    assert_eq!(
      protocol.text_size(2, 8)?,
      join!(constants::TEXT_SIZE_SELECT, 0x17)
    );
    assert!(protocol.text_size(9, 16).is_err());
    assert!(protocol.text_size(0, 9).is_err());

    Ok(())
  }

  #[test]
  fn test_reset() -> Result<()> {
    let mut protocol = Protocol::default();
    assert_eq!(protocol.reset(), constants::HARDWARE_RESET);
    Ok(())
  }

  #[test]
  fn test_init() -> Result<()> {
    let mut protocol = Protocol::default();
    assert_eq!(protocol.init(), constants::HARDWARE_INIT);
    Ok(())
  }

  #[test]
  fn reverse_colours() -> Result<()> {
    let protocol = Protocol::default();
    assert_eq!(
      protocol.reverse_colours(true)?,
      constants::TEXT_REVERSE_COLOURS_ON
    );
    assert_eq!(
      protocol.reverse_colours(false)?,
      constants::TEXT_REVERSE_COLOURS_OFF
    );
    Ok(())
  }

  #[test]
  fn test_left() -> Result<()> {
    let protocol = Protocol::default();
    assert_eq!(
      protocol.left(10)?,
      join!(constants::TEXT_MARGIN_LEFT, 0x0A, 0x00)
    );
    assert_eq!(
      protocol.left(1)?,
      join!(constants::TEXT_MARGIN_LEFT, 0x01, 0x00)
    );
    Ok(())
  }

  #[test]
  fn test_width() -> Result<()> {
    let protocol = Protocol::default();
    assert_eq!(
      protocol.width(10)?,
      join!(constants::TEXT_PRINTABLE_AREA, 0x0A, 0x00)
    );
    assert_eq!(
      protocol.width(1)?,
      join!(constants::TEXT_PRINTABLE_AREA, 0x01, 0x00)
    );
    Ok(())
  }

  #[test]
  fn test_cut() -> Result<()> {
    let mut protocol = Protocol::default();
    assert_eq!(protocol.cut(true)?, constants::PAPER_CUT_PARTIAL);
    assert_eq!(protocol.cut(false)?, constants::PAPER_CUT_FULL);
    Ok(())
  }

  #[test]
  fn test_print() -> Result<()> {
    let mut protocol = Protocol::default();
    let buffer = protocol.print("Hello World")?;
    assert_eq!(
      buffer,
      vec![72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]
    );
    Ok(())
  }

  #[test]
  fn test_println() -> Result<()> {
    let mut protocol = Protocol::default();
    let buffer = protocol.println("Hello World")?;
    assert_eq!(
      buffer,
      vec![0x1B, 0x64, 0x01, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64]
    );
    Ok(())
  }

  #[test]
  fn test_text() -> Result<()> {
    let mut protocol = Protocol::default();
    let buffer = protocol.text("Hello World")?;
    assert_eq!(
      buffer,
      vec![0x1B, 0x64, 0x01, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64]
    );
    Ok(())
  }

  #[test]
  fn test_graphic() -> Result<()> {
    let mut builder = Graphic::builder();
    builder.path("resources/rust-logo-small.png");

    let graphic = builder.build()?;

    let mut protocol = Protocol::default();
    let buffer = protocol.graphic(graphic)?;
    assert_eq!(&buffer[0..3], constants::IMAGE_HEADER);

    assert_eq!(buffer[3], 0x00);
    assert_eq!(buffer[4], 0x19);
    assert_eq!(buffer[5], 0x00);
    assert_eq!(buffer[6], 0xC8);
    assert_eq!(buffer[7], 0x00);
    assert_eq!(buffer.len(), 5008);

    Ok(())
  }

  #[test]
  fn test_qr() -> Result<()> {
    let mut protocol = Protocol::default();
    let mut builder = Qr::builder();
    builder.text("Hello world");

    let qr = builder.build();

    let buffer = protocol.qr(qr)?;

    assert_eq!(&buffer[0..8], constants::QR_SELECT_MODEL);
    assert_eq!(buffer[8], 0x31);
    assert_eq!(&buffer[9..16], constants::QR_LEVEL);
    assert_eq!(buffer[16], 0x08);
    assert_eq!(&buffer[17..24], constants::QR_CORRECTION_ERROR_LEVEL);
    assert_eq!(buffer[24], constants::QR_CORRECTION_ERROR_LEVEL_MEDIUM);
    assert_eq!(&buffer[25..28], constants::QR_DATA_STORE_PREFIX);
    assert_eq!(buffer[28], 0x0E);
    assert_eq!(buffer[29], 0x00);
    assert_eq!(&buffer[30..33], constants::QR_DATA_STORE_SUFFIX);
    assert_eq!(&buffer[33..44], "Hello world".as_bytes());
    assert_eq!(&buffer[44..52], constants::QR_PRINT);

    Ok(())
  }

  #[test]
  fn test_barcode() -> Result<()> {
    let mut protocol = Protocol::default();
    let mut builder = Barcode::builder();
    builder
      .text("123456123456")
      .system(BarcodeSystem::Ean13)
      .text_position(BarcodeTextPosition::None);
    let barcode = builder.build();

    let buffer = protocol.barcode(barcode)?;

    assert_eq!(&buffer[0..3], constants::BARCODE_TYPE_JAN13_EAN13);
    assert_eq!(&buffer[3..5], constants::BARCODE_HEIGHT);
    assert_eq!(buffer[5], 0x68);
    assert_eq!(buffer[5], 0x68);
    assert_eq!(&buffer[6..8], constants::BARCODE_WIDTH);
    assert_eq!(buffer[8], 0x02);
    assert_eq!(&buffer[9..12], constants::BARCODE_FONT_A);
    assert_eq!(&buffer[12..14], constants::BARCODE_TEXT_POSITION);
    assert_eq!(buffer[14], 0x00);
    assert_eq!(buffer[110], 0x00);

    Ok(())
  }
}
