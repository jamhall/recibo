# recibo - ESC/POS driver for Rust [![Build Status]][actions] [![Latest Version]][crates.io]

[Build Status]: https://img.shields.io/github/actions/workflow/status/jamhall/recibo/ci.yml?branch=main
[actions]: https://github.com/jamhall/recibo/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/recibo.svg
[crates.io]: https://crates.io/crates/recibo

This project provides an implementation for a subset of Epson's [ESC/POS](https://en.wikipedia.org/wiki/ESC/P) protocol used by compatible receipt printers.
It can generate and print receipts that include basic formatting, barcodes, graphics and cutting functions on a compatible
printer.

![Example print out](resources/demo.png?raw=true)
-

## Example usage

```rust
let driver = NetworkDriver::open("127.0.0.1", 9100)?;
let mut printer = Printer::open(driver)?;

printer.init()?
       .align(Alignment::Center)?
       .text_size(4, 4)?
       .text("Hello World")?
       .feed(2)?
       .graphic(move |builder| {
            builder.path("resources/rust-logo.png")
                   .size(GraphicSize::Normal)
        })?
       .feed(4)?
       .cut()?;
```

## Table of Contents

- [Examples](#examples)
- [Installation](#installation)
- [Adapters](#adapters)
- [Supported commands](#supported-commands)
- [Contributing](#contributing)
- [License](#license)

## Examples

Refer to the [examples](examples) directory for further instances of usage.

For the sake of simplicity, the examples will print their output to the console.

To launch an example, use the following command:

```shell
cargo run --example graphic --features "graphics" --quiet
```

If you wish to direct an example's output to a network printer, you can employ the netcat command:

```shell
cargo run --example text | nc 192.168.0.100 9100
```

## Installation

For standard functionalities (e.g. printing text, barcodes, qr codes etc.), no additional dependencies are required:

```
[dependencies]
recibo = "1.0.0"
```

If you would like to raster images, you will need to enable the `image` feature:

```
[dependencies]
recibo = { version = "1.0.0", features = ["graphics"] }
```

You can also enable deserialisation and serialisation using serde if you enable the feature:

```
[dependencies]
recibo = { version = "1.0.0", features = ["serde"] }
```

## Adapters

The library provides two adapters for communicating with the printer:

#### NetworkDriver
This is used for establishing communication with a network printer


```shell
let driver = NetworkDriver::open("192.168.0.100", 9100)?;
let printer = Printer::open(driver)?;
```

#### FileDriver

This is used when interacting with a serial printer or writing to a file

```shell
let driver = FileDriver::open("/tmp/output.bin")?;
let printer = Printer::open(driver)?;
```

#### ConsoleDriver

This is used for writing the output to the console

```shell
let driver = ConsoleDriver::open();
let printer = Printer::open(driver)?;
```

## Supported Commands

> Some of the commands may not be supported by your printer

| Command           | Description                                                        |
|-------------------|--------------------------------------------------------------------|
| init              | Initializes the printer.                                           |
| reset             | Resets the printer to its default settings.                        |
| align             | Aligns the text to the left, right, or center.                     |
| left              | Sets the left margin to n dots.                                    |
| width             | Sets the printable area width.                                     |
| font              | Sets the font to either style 'a', 'b', or 'c'.                    |
| bold              | Sets the emphasis of the text to bold.                             |
| text_size         | Sets the font size of the text.                                    |
| reset_text_size   | Resets the font size of the text.                                  |
| underline         | Underlines the text with a single or double stroke.                |
| doublestrike      | Applies a double-strike effect to the text.                        |
| linespacing       | Adjusts the spacing between lines of text.                         |
| reset_linespacing | Resets the line spacing to the default value.                      |
| flip              | Turns the text upside down.                                        |
| reverse_colours   | Enables white text on a black background.                          |
| qr                | Prints a QR Code.                                                  |
| barcode           | Prints a barcode.                                                  |
| graphic           | Prints a graphic.                                                  |
| feed              | Feeds n lines of paper.                                            |
| reverse_feed      | Reverses the paper feed by n lines.                                |
| cut               | Performs a full cut of the paper.                                  |
| partial_cut       | Performs a partial cut of the paper.                               |
| print             | Prints the specified text.                                         |
| println           | Prints the specified text with a new line ending.                  |
| text              | Same as println, prints the specified text with a new line ending. |

# Contributing

Contributions are welcome! Please open an issue or submit a pull request.

# License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
