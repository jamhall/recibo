use log::debug;

use crate::domain::{Alignment, BarcodeBuilder, Font, Hardware, Qr, QrBuilder, UnderlineMode};
#[cfg(feature = "graphics")]
use crate::domain::{Graphic, GraphicBuilder};
use crate::error::Result;
use crate::io::{Driver, Encoder, NoopDriver, Protocol};
use crate::Barcode;

pub struct Printer(Box<dyn Driver>, Protocol);

impl Printer {
  fn new(driver: Box<dyn Driver>, protocol: Protocol) -> Self {
    debug!("Creating printer with driver: {}", driver.name());
    Self(driver, protocol)
  }

  pub fn open(driver: Box<dyn Driver>) -> Result<Self> {
    let protocol = Protocol::default();
    Ok(Self::new(driver, protocol))
  }

  pub fn hardware(&mut self, hardware: Hardware) -> Result<&mut Self> {
    debug!("Hardware command: {}", hardware);
    let bytes = self.1.hardware(hardware);
    self.0.write(bytes).map(|_| self)
  }

  pub fn font(&mut self, font: Font) -> Result<&mut Self> {
    debug!("Font command: {}", font);
    let bytes = self.1.font(font);
    self.0.write(bytes).map(|_| self)
  }

  pub fn align(&mut self, alignment: Alignment) -> Result<&mut Self> {
    debug!("Aligning {}", alignment);
    let bytes = self.1.align(alignment);
    self.0.write(bytes).map(|_| self)
  }

  pub fn doublestrike(&mut self, enabled: bool) -> Result<&mut Self> {
    debug!("doublestrike: {}", enabled);
    let bytes = self.1.doublestrike(enabled);
    self.0.write(bytes).map(|_| self)
  }

  pub fn linespacing(&mut self, height: u8) -> Result<&mut Self> {
    debug!("linespacing: {}", height);
    let bytes = self.1.linespacing(height);
    self.0.write(&bytes).map(|_| self)
  }

  pub fn flip(&mut self, enabled: bool) -> Result<&mut Self> {
    debug!("flip: {}", enabled);
    let bytes = self.1.flip(enabled);
    self.0.write(bytes).map(|_| self)
  }

  pub fn reset_linespacing(&mut self) -> Result<&mut Self> {
    debug!("resetting linespacing");
    let bytes = self.1.reset_linespacing();
    self.0.write(&bytes).map(|_| self)
  }

  pub fn bold(&mut self, enabled: bool) -> Result<&mut Self> {
    debug!("Setting bold to {}", enabled);
    let bytes = self.1.bold(enabled);
    self.0.write(bytes).map(|_| self)
  }

  pub fn underline(&mut self, mode: UnderlineMode) -> Result<&mut Self> {
    debug!("Setting underline to {}", mode);
    let bytes = self.1.underline(&mode);
    self.0.write(bytes).map(|_| self)
  }

  pub fn reset(&mut self) -> Result<&mut Self> {
    debug!("Resetting printer");
    let bytes = self.1.reset();
    self.0.write(bytes).map(|_| self)
  }

  pub fn init(&mut self) -> Result<&mut Self> {
    debug!("Initialising printer");
    let bytes = self.1.init();
    self.0.write(bytes).map(|_| self)
  }

  pub fn feed(&mut self, n: u8) -> Result<&mut Self> {
    debug!("Feeding {} lines", n);
    let bytes = self.1.feed(n)?;
    self.0.write(&bytes).map(|_| self)
  }

  pub fn reverse_feed(&mut self, n: u8) -> Result<&mut Self> {
    debug!("Reverse feeding {} lines", n);
    let bytes = self.1.reverse_feed(n)?;
    self.0.write(&bytes).map(|_| self)
  }

  pub fn cut(&mut self) -> Result<&mut Self> {
    debug!("Cutting paper");
    let bytes = self.1.cut(false)?;
    self.0.write(&bytes).map(|_| self)
  }

  pub fn partial_cut(&mut self) -> Result<&mut Self> {
    debug!("Partially cutting paper");
    let bytes = self.1.cut(true)?;
    self.0.write(&bytes).map(|_| self)
  }

  pub fn print<T: AsRef<str>>(&mut self, text: T) -> Result<&mut Self> {
    let text = text.as_ref();
    debug!("Printing text: {}", text);
    let bytes = self.1.text(text)?;
    self.0.write(&bytes).map(|_| self)
  }

  pub fn println<T: AsRef<str>>(&mut self, text: T) -> Result<&mut Self> {
    let text = text.as_ref();
    debug!("Printing line: {}", text);
    let bytes = self.1.println(text)?;
    self.0.write(&bytes).map(|_| self)
  }

  pub fn text<T: AsRef<str>>(&mut self, text: T) -> Result<&mut Self> {
    let text = text.as_ref();
    if !text.is_empty() {
      return self.println(text);
    }
    Ok(self)
  }

  pub fn text_size(&mut self, width_multiplier: u8, height_multiplier: u8) -> Result<&mut Self> {
    debug!(
      "Setting text size to {}x{}",
      width_multiplier, height_multiplier
    );
    let bytes = self.1.text_size(width_multiplier, height_multiplier)?;
    self.0.write(&bytes).map(|_| self)
  }

  pub fn reset_text_size(&mut self) -> Result<&mut Self> {
    debug!("Resetting text size");
    self.text_size(1, 1)
  }

  pub fn mode(&mut self) -> Result<&mut Self> {
    debug!("Setting mode to {}", 1);
    let bytes = self.1.mode()?;
    self.0.write(&bytes).map(|_| self)
  }

  pub fn flush(&mut self) -> Result<()> {
    self.0.flush()
  }

  pub fn barcode<F>(&mut self, function: F) -> Result<&mut Self>
  where
    F: Fn(&mut BarcodeBuilder) -> &mut BarcodeBuilder,
  {
    let mut builder = Barcode::builder();
    function(&mut builder);
    let barcode = builder.build();
    debug!("Printing barcode: {}", barcode);
    let bytes = self.1.barcode(barcode)?;
    self.0.write(&bytes).map(|_| self)
  }

  pub fn qr<F>(&mut self, function: F) -> Result<&mut Self>
  where
    F: Fn(&mut QrBuilder) -> &mut QrBuilder,
  {
    let mut builder = Qr::buidler();
    function(&mut builder);
    let qr = builder.build();
    debug!("Printing qr: {}", qr);
    let bytes = self.1.qr(qr)?;
    self.0.write(&bytes).map(|_| self)
  }

  #[cfg(feature = "graphics")]
  pub fn graphic<F>(&mut self, function: F) -> Result<&mut Self>
  where
    F: Fn(&mut GraphicBuilder) -> &mut GraphicBuilder,
  {
    let mut builder = Graphic::builder();
    function(&mut builder);
    let graphic = builder.build()?;
    debug!("Printing graphic: {}", graphic.path());
    let bytes = self.1.graphic(graphic)?;
    self.0.write(&bytes).map(|_| self)
  }

  pub fn reverse_colours(&mut self, enabled: bool) -> Result<&mut Self> {
    debug!("Reverse colours: {}", enabled);
    let bytes = self.1.reverse_colours(enabled)?;
    self.0.write(&bytes).map(|_| self)
  }

  pub fn left(&mut self, dots: u16) -> Result<&mut Self> {
    debug!("Setting left margin to {} dots", dots);
    let bytes = self.1.left(dots)?;
    self.0.write(&bytes).map(|_| self)
  }

  pub fn width(&mut self, margin: u16) -> Result<&mut Self> {
    debug!("Setting width to {}", margin);
    let bytes = self.1.width(margin)?;
    self.0.write(&bytes).map(|_| self)
  }

  pub fn builder() -> PrinterBuilder {
    PrinterBuilder::new()
  }
}

pub struct PrinterBuilder {
  driver: Box<dyn Driver>,
  encoder: Encoder,
}

impl PrinterBuilder {
  #[allow(clippy::new_without_default)]
  // I did not use a default implementation here because of clippy encountering
  // this issue: https://github.com/rust-lang/rust-clippy/issues/9621
  pub fn new() -> PrinterBuilder {
    Self {
      driver: Box::<NoopDriver>::default(),
      encoder: Encoder::default(),
    }
  }

  pub fn driver(mut self, driver: Box<dyn Driver>) -> Self {
    self.driver = driver;
    self
  }

  pub fn encoder(mut self, encoder: Encoder) -> Self {
    self.encoder = encoder;
    self
  }

  pub fn build(self) -> Printer {
    let driver = self.driver;
    let encoder = self.encoder;
    let protocol = Protocol::new(encoder);

    Printer::new(driver, protocol)
  }
}
