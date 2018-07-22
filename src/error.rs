use std::io::Error as IoError;
use std::result::Result as StdResult;

#[derive(Fail, Debug)]
pub enum Error {
  #[fail(display = "syntax error: {}", _0)]
  Syntax(String),

  #[fail(display = "{}", _0)]
  Io(#[cause] IoError),
}

impl From<IoError> for Error {
  fn from(io_error: IoError) -> Self {
    Error::Io(io_error)
  }
}

pub type Result<T> = StdResult<T, Error>;
