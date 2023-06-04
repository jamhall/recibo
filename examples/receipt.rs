use recibo::{Alignment, ConsoleDriver, Printer};

fn main() -> recibo::Result<()> {
  let items = vec![
    Item::new("Macbook Pro", 2500.00, false),
    Item::new("Macbook Air", 1500.00, false),
    Item::new("iMac", 3000.00, false),
    Item::new("AirPods", 200.00, false),
    Item::new("iPhone", 1000.00, false),
    Item::new("iPad", 800.00, false),
    Item::new("Apple Watch", 400.00, false),
  ];

  let subtotal = Item::new(
    "Subtotal",
    items.iter().fold(0.0, |acc, item| acc + item.price),
    false,
  );
  let tax = Item::new("Tax", subtotal.price * 0.20, false);
  let total = Item::new("Total", subtotal.price + tax.price, false);

  let driver = ConsoleDriver::open();
  let mut printer = Printer::open(driver)?;

  printer
    .init()?
    .align(Alignment::Center)?
    .bold(true)?
    .text_size(2, 2)?
    .text("Receipt")?
    .align(Alignment::Left)?
    .feed(1)?
    .reset_text_size()?
    .bold(false)?;

  for item in items {
    let item: String = item.into();
    printer.text(item)?;
  }

  printer.bold(true)?;

  let subtotal: String = subtotal.into();
  let tax: String = tax.into();
  let total: String = total.into();
  printer.text(subtotal)?;
  printer.text(tax)?;
  printer.text(total)?;
  Ok(())
}

pub struct Item {
  pub name: String,
  pub price: f32,
  pub symbol: bool,
}

impl Item {
  pub fn new<T: AsRef<str>>(name: T, price: f32, symbol: bool) -> Item {
    let name = name.as_ref().to_string();
    Item {
      name,
      price,
      symbol,
    }
  }
}

impl From<Item> for String {
  fn from(item: Item) -> Self {
    let right_cols = 10;
    let left_cols = 38;

    let left = format!("{: <width$}", item.name, width = left_cols);
    let right = format!("{: >width$.2}", item.price, width = right_cols);

    format!("{}{}\n", left, right)
  }
}
