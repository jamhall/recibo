use recibo::{Alignment, ConsoleDriver, Printer};

fn main() -> recibo::Result<()> {
  let driver = ConsoleDriver::open();
  let mut printer = Printer::open(driver)?;

  printer
    .init()?
    .align(Alignment::Center)?
    .qr(|builder| builder.size(200).text("Hello World"))?
    .text("Hello World")?
    .feed(4)?
    .cut()?
    .flush()?;

  Ok(())
}
