extern crate failure;
#[macro_use]
extern crate failure_derive;

use std::env;
use std::fs::File;
use std::io;
use std::io::Read;

mod error;
use error::Result;

mod instruction;

mod interpreter;
use interpreter::Interpreter;

mod parser;
use parser::parse;

const MEMORY_SIZE: usize = 65_536;

fn main() {
  if let Err(error) = run() {
    println!("{}", error);
  }
}

fn run() -> Result<()> {
  let mut input: Box<dyn Read> = match env::args_os().nth(1) {
    Some(path) => Box::new(File::open(path)?),
    None => Box::new(io::stdin()),
  };

  let mut code = String::new();
  input.read_to_string(&mut code)?;
  let code_chars = code.chars().collect::<Vec<_>>();

  let parsed_program = parse(&code_chars)?;
  let mut interpreter = Interpreter::new(MEMORY_SIZE);
  interpreter.run(&parsed_program)
}
