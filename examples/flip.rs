use recibo::{Alignment, ConsoleDriver, Printer};

fn main() -> recibo::Result<()> {
  let driver = ConsoleDriver::open();
  let mut printer = Printer::open(driver)?;

  printer
    .init()?
    .align(Alignment::Center)?
    .flip(true)?
    .text("Hello upside down text")?
    .feed(2)?
    .cut()?
    .flush()?;

  Ok(())
}
