use std::io;
use std::io::{Read, Write};

use parser::Instruction;

pub struct Interpreter {
  memory: Vec<u8>,
  pointer: usize,
}

impl Interpreter {
  pub fn new(memory_size: usize) -> Self {
    Interpreter {
      memory: vec![0; memory_size],
      pointer: 0,
    }
  }

  pub fn run(&mut self, program: &[Instruction]) {
    let mut char_index = 0;

    while char_index < program.len() {
      match program[char_index] {
        Instruction::Right(n) => self.move_right(n),
        Instruction::Left(n) => self.move_left(n),

        Instruction::Add(n) => self.add(n),
        Instruction::Subtract(n) => self.subtract(n),

        Instruction::Print => self.print_char().unwrap(),
        Instruction::Read => self.read_char().unwrap(),

        Instruction::JumpIfZero(address) => if self.read_memory() == 0 {
          char_index = address;
        },

        Instruction::JumpIfNonZero(address) => if self.read_memory() != 0 {
          char_index = address;
        },
      }

      char_index += 1;
    }
  }

  fn move_right(&mut self, n: usize) {
    self.pointer += n;
  }

  fn move_left(&mut self, n: usize) {
    self.pointer -= n;
  }

  fn add(&mut self, n: usize) {
    let value = self.read_memory().wrapping_add(n as u8);
    self.store_memory(value);
  }

  fn subtract(&mut self, n: usize) {
    let value = self.read_memory().wrapping_sub(n as u8);
    self.store_memory(value);
  }

  fn print_char(&self) -> io::Result<()> {
    let chr = self.read_memory();
    let mut stdout = io::stdout();
    stdout.write_all(&[chr])?;
    stdout.flush()?;
    Ok(())
  }

  fn read_char(&mut self) -> io::Result<()> {
    let mut buffer = [0; 1];
    let bytes = io::stdin().read(&mut buffer)?;
    self.store_memory(if bytes > 0 { buffer[0] } else { 0 });
    Ok(())
  }

  fn read_memory(&self) -> u8 {
    self.memory[self.pointer]
  }

  fn store_memory(&mut self, value: u8) {
    self.memory[self.pointer] = value
  }
}
