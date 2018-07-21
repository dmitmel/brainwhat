use std::env;
use std::fs::File;
use std::io;
use std::io::Read;

mod interpreter;
use interpreter::Interpreter;

mod parser;
use parser::parse;

const MEMORY_SIZE: usize = 65_536;

fn main() {
  let mut input: Box<dyn Read> = match env::args_os().nth(1) {
    Some(path_arg) => Box::new(File::open(path_arg).unwrap()),
    None => Box::new(io::stdin()),
  };

  let mut program = String::new();
  input.read_to_string(&mut program).unwrap();
  let program_chars = program.chars().collect::<Vec<_>>();

  let parsed_program = parse(&program_chars);
  let mut interpreter = Interpreter::new(MEMORY_SIZE);
  interpreter.run(&parsed_program);
}
