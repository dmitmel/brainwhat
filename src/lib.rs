extern crate failure;
#[macro_use]
extern crate failure_derive;

pub mod error;
pub use error::*;

pub mod instruction;
pub use instruction::*;

pub mod interpreter;
pub use interpreter::*;

pub mod parser;
pub use parser::*;
