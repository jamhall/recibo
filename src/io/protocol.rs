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

  pub fn linespacing(&mut self, height: u8) -> Vec<u8> {
    vec![constants::ESC, 0x03, height]
  }

  pub fn flip(&mut self, enabled: bool) -> &[u8] {
    if enabled {
      constants::TEXT_FLIP_ON
    } else {
      constants::TEXT_FLIP_OFF
    }
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

  pub fn reset_linespacing(&mut self) -> Vec<u8> {
    vec![constants::ESC, 0x02]
  }

  pub fn bold(&mut self, enabled: bool) -> &[u8] {
    if enabled {
      constants::TEXT_BOLD_MODE_ON
    } else {
      constants::TEXT_BOLD_MODE_OFF
    }
  }

  pub fn underline(&mut self, mode: &UnderlineMode) -> &[u8] {
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

    Ok(vec![
      constants::GS,
      0x21,
      (2 << 3) * (width_multiplier - 1) + (height_multiplier - 1),
    ])
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

  pub fn mode(&self) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    buffer.write_all(constants::HARDWARE_PRINT_MODE)?;
    buffer.write_u8(0x00)?;
    Ok(buffer)
  }

  pub fn cut(&mut self, partial: bool) -> Result<Vec<u8>> {
    let cut = if partial {
      constants::PAPER_CUT_PARTIAL
    } else {
      constants::PAPER_CUT_FULL
    };

    let mut buffer = self.feed(3)?;
    buffer.write_all(cut)?;

    Ok(buffer)
  }

  pub fn print(&mut self, text: &str) -> Result<Vec<u8>> {
    let encoded = self.encoder.encode(text)?;
    Ok(encoded)
  }

  pub fn println(&mut self, text: &str) -> Result<Vec<u8>> {
    let feed = self.feed(1)?;
    let mut buffer = self.print(text)?;
    buffer.write_all(feed.as_slice())?;

    Ok(buffer)
  }

  pub fn text(&mut self, text: &str) -> Result<Vec<u8>> {
    self.println(text)
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

          let is_black = graphic.pixel(x_offset as u32, y as u32).0[0] <= 128;

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
