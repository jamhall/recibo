use recibo::{ConsoleDriver, Printer};

fn main() -> recibo::Result<()> {
  let driver = ConsoleDriver::open();
  let mut printer = Printer::open(driver)?;

  printer.init()?;

  printer.text("Change height & width")?;

  for n in 1..8 {
    printer.text_size(n, n)?;
    printer.text(format!("{n}"))?;
  }

  printer.feed(1)?;

  printer.text("Change width only (height=4)")?;

  for n in 1..8 {
    printer.text_size(n, 4)?;
    printer.text(format!("{n}"))?;
  }

  printer.feed(1)?;

  printer.text("Change height only (width=4)")?;

  for n in 1..8 {
    printer.text_size(4, n)?;
    printer.text(format!("{n}"))?;
  }

  printer.feed(1)?;

  printer.text("Very narrow text")?;

  printer.text_size(4, 1)?;
  printer.text("Hello world!")?;

  printer.feed(1)?;

  printer.text("Very wide text")?;

  printer.text_size(4, 1)?;
  printer.text("Hello world!")?;

  printer.text("Largest possible text")?;

  printer.text_size(8, 8)?;
  printer.text("Hello world!")?;

  printer.feed(5)?;

  printer.cut()?;

  printer.flush()?;

  Ok(())
}
