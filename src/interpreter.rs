use std::io;
use std::io::{Read, Write};

use error::Result;
use instruction::{Instruction, Instruction::*};

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

  pub fn run(&mut self, program: &[Instruction]) -> Result<()> {
    let mut char_index = 0;

    while char_index < program.len() {
      match program[char_index] {
        Move(n) => if n > 0 {
          self.pointer += n as usize;
        } else if n < 0 {
          self.pointer -= n.abs() as usize;
        },

        Add(n) => {
          let value = self.read_memory();
          self.store_memory(if n > 0 {
            value.wrapping_add(n as u8)
          } else if n < 0 {
            value.wrapping_sub(n.abs() as u8)
          } else {
            value
          });
        }

        Print => self.print_char()?,
        Read => self.read_char()?,

        JumpIfZero(address) => if self.read_memory() == 0 {
          char_index = address;
        },
        JumpIfNonZero(address) => if self.read_memory() != 0 {
          char_index = address;
        },
      }

      char_index += 1;
    }

    Ok(())
  }

  fn print_char(&self) -> io::Result<()> {
    let chr = self.read_memory();
    io::stdout().write_all(&[chr])?;
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
