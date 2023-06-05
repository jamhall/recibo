use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;
use std::net::TcpStream;
use std::path::Path;
use std::rc::Rc;

use log::debug;

use crate::error::Result;

pub trait Driver {
  fn name(&self) -> &str;

  fn write(&self, data: &[u8]) -> Result<()>;

  fn flush(&self) -> Result<()>;
}

pub struct NetworkDriver(Rc<RefCell<TcpStream>>);

#[derive(Default)]
pub struct ConsoleDriver {}

impl Driver for ConsoleDriver {
  fn name(&self) -> &str {
    "console"
  }

  fn write(&self, data: &[u8]) -> Result<()> {
    io::stdout().write_all(data).map_err(Into::into)
  }

  fn flush(&self) -> Result<()> {
    Ok(())
  }
}

impl ConsoleDriver {
  pub fn open() -> Box<Self> {
    Box::new(Self {})
  }
}

impl NetworkDriver {
  pub fn open<A: AsRef<str>>(host: A, port: u16) -> Result<Box<NetworkDriver>> {
    fn inner(host: &str, port: u16) -> Result<Box<NetworkDriver>> {
      debug!("Connecting to address {}:{}", host, port);
      let stream = TcpStream::connect((host, port))?;
      let inner = Rc::new(RefCell::new(stream));
      Ok(Box::new(NetworkDriver(inner)))
    }
    inner(host.as_ref(), port)
  }
}

impl Driver for NetworkDriver {
  fn name(&self) -> &'static str {
    "network"
  }

  fn write(&self, data: &[u8]) -> Result<()> {
    self.0.borrow_mut().write_all(data).map_err(Into::into)
  }

  fn flush(&self) -> Result<()> {
    self.0.borrow_mut().flush().map_err(Into::into)
  }
}

pub struct FileDriver(Rc<RefCell<File>>);

impl FileDriver {
  pub fn new<P: AsRef<Path>>(path: P) -> Result<Box<Self>> {
    let file = OpenOptions::new().read(true).write(true).open(path)?;
    let file = Rc::new(RefCell::new(file));
    Ok(Box::new(FileDriver(file)))
  }
}

impl Driver for FileDriver {
  fn name(&self) -> &'static str {
    "file"
  }

  fn write(&self, data: &[u8]) -> Result<()> {
    self.0.borrow_mut().write_all(data).map_err(Into::into)
  }

  fn flush(&self) -> Result<()> {
    self.0.borrow_mut().flush().map_err(Into::into)
  }
}

#[derive(Default)]
pub struct NoopDriver;

impl Driver for NoopDriver {
  fn name(&self) -> &'static str {
    "noop"
  }

  fn write(&self, _data: &[u8]) -> Result<()> {
    Ok(())
  }

  fn flush(&self) -> Result<()> {
    Ok(())
  }
}
