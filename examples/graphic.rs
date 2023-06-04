use recibo::{Alignment, ConsoleDriver, GraphicSize, Printer};

fn main() -> recibo::Result<()> {
  let driver = ConsoleDriver::open();
  let mut printer = Printer::open(driver)?;

  printer
    .init()?
    .align(Alignment::Center)?
    .text_size(4, 4)?
    .text("Hello World")?
    .feed(2)?
    .graphic(move |builder| {
      builder
        .path("resources/rust-logo.png")
        .size(GraphicSize::Normal)
    })?
    .feed(4)?
    .cut()?
    .flush()?;

  Ok(())
}
