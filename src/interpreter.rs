use std::io;
use std::io::{Read, Write};

use error::{Error, Result};
use instruction::{Instruction, Instruction::*};

#[derive(Debug)]
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
    self.run_with_io(program, &mut io::stdin(), &mut io::stdout())
  }

  pub fn run_with_io<I: Read, O: Write>(
    &mut self,
    program: &[Instruction],
    input: &mut I,
    output: &mut O,
  ) -> Result<()> {
    let mut char_index = 0;

    while char_index < program.len() {
      match program[char_index] {
        Move(n) => {
          if n > 0 {
            let n = n as usize;
            if n <= self.memory.len() && self.pointer < self.memory.len() - n {
              self.pointer += n;
            } else {
              return Err(Error::PointerOverflow);
            }
          } else if n < 0 {
            let n = n.abs() as usize;
            if self.pointer >= n {
              self.pointer -= n;
            } else {
              return Err(Error::PointerUnderflow);
            }
          }
        }

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

        JumpIfZero(address) => {
          if self.read_memory() == 0 {
            char_index = address;
          }
        }
        JumpIfNonZero(address) => {
          if self.read_memory() != 0 {
            char_index = address;
          }
        }

        Print => self.print_char(output)?,
        Read => self.read_char(input)?,

        Clear => self.store_memory(0),
      }

      char_index += 1;
    }

    Ok(())
  }

  fn print_char<O: Write>(&self, output: &mut O) -> io::Result<()> {
    let chr = self.read_memory();
    output.write_all(&[chr])?;
    Ok(())
  }

  fn read_char<I: Read>(&mut self, input: &mut I) -> io::Result<()> {
    let mut buffer = [0; 1];
    let bytes = input.read(&mut buffer)?;
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

#[cfg(test)]
mod tests {
  use super::*;

  const MEMORY_SIZE: usize = 100;

  fn test_run<I: Read, O: Write>(
    program: &[Instruction],
    input: &mut I,
    output: &mut O,
  ) -> Result<()> {
    let mut interpreter = Interpreter::new(MEMORY_SIZE);
    interpreter.run_with_io(program, input, output)
  }

  macro_rules! test {
    ($name:ident, $program:expr, $input:expr, $expected_output:expr) => {
      #[test]
      fn $name() {
        let mut input: &[u8] = &$input;
        let expected_output: &[u8] = &$expected_output;
        let mut actual_output: Vec<u8> = Vec::new();
        test_run(&$program, &mut input, &mut actual_output).unwrap();
        assert_eq!(&actual_output[..], expected_output);
      }
    };
  }

  struct FakeIo;

  impl Read for FakeIo {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
      Err(io::Error::new(io::ErrorKind::Other, "this is fake I/O"))
    }
  }

  impl Write for FakeIo {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
      Err(io::Error::new(io::ErrorKind::Other, "this is fake I/O"))
    }

    fn flush(&mut self) -> io::Result<()> {
      Err(io::Error::new(io::ErrorKind::Other, "this is fake I/O"))
    }
  }

  macro_rules! test_error {
    ($name:ident, $program:expr, $input:expr, $expected_error:pat) => {
      #[test]
      fn $name() {
        let mut input: &[u8] = &$input;
        let mut output = FakeIo;
        let actual_error =
          test_run(&$program, &mut input, &mut output).unwrap_err();
        assert!(match actual_error {
          $expected_error => true,
          _ => false,
        })
      }
    };
  }

  macro_rules! test_io_error {
    ($name:ident, $program:expr, $expected_error:pat) => {
      #[test]
      fn $name() {
        let actual_error =
          test_run(&$program, &mut FakeIo, &mut FakeIo).unwrap_err();
        assert!(match actual_error {
          $expected_error => true,
          _ => false,
        })
      }
    };
  }

  test!(test_print, [Print, Print, Print], [], [0, 0, 0]);

  test!(
    test_add,
    [Print, Add(5), Print, Add(-3), Print],
    [],
    [0, 5, 2]
  );

  test!(
    test_add_overflow,
    [
      Print,
      Add(0),
      Print,
      Add(-1),
      Print,
      Add(1),
      Print,
      Add(256),
      Print,
      Add(-256),
      Print,
      Add(257),
      Print,
      Add(257),
      Print,
      Add(123_456_789),
      Print,
      Add(789_456_321),
      Print,
    ],
    [],
    [0, 0, 255, 0, 0, 0, 1, 2, 23, 216]
  );

  test!(
    test_move,
    [
      Print,
      Move(1),
      Print,
      Move(1),
      Print,
      Move(-2),
      Add(1),
      Move(1),
      Add(2),
      Move(1),
      Add(3),
      Move(-2),
      Print,
      Move(1),
      Print,
      Move(1),
      Print,
    ],
    [],
    [0, 0, 0, 1, 2, 3]
  );

  test_error!(
    test_move_underflow_from_left,
    [Move(-1)],
    [],
    Error::PointerUnderflow
  );
  test_error!(
    test_move_underflow,
    [Move(1), Move(-2)],
    [],
    Error::PointerUnderflow
  );
  test_error!(
    test_move_underflow_from_right,
    [
      Move(MEMORY_SIZE as isize - 1),
      Move(-(MEMORY_SIZE as isize) - 1)
    ],
    [],
    Error::PointerUnderflow
  );

  test_error!(
    test_move_overflow_from_left,
    [Move(MEMORY_SIZE as isize + 1)],
    [],
    Error::PointerOverflow
  );
  test_error!(
    test_move_overflow,
    [Move(MEMORY_SIZE as isize - 2), Move(2)],
    [],
    Error::PointerOverflow
  );
  test_error!(
    test_move_overflow_from_right,
    [Move(MEMORY_SIZE as isize - 1), Move(1)],
    [],
    Error::PointerOverflow
  );

  test!(
    test_read,
    [Print, Add(3), Print, Read, Print],
    [42],
    [0, 3, 42]
  );

  test!(
    test_read_eof,
    [Print, Add(3), Print, Read, Print, Read, Print],
    [42],
    [0, 3, 42, 0]
  );

  test!(
    test_simple_loops,
    [
      Add(3),
      JumpIfZero(7),
      Add(-1),
      Move(1),
      Read,
      Print,
      Move(-1),
      JumpIfNonZero(1)
    ],
    [9, 8, 7, 6, 5, 4, 3, 2, 1],
    [9, 8, 7]
  );

  test!(
    test_nested_loops,
    [
      Read,
      JumpIfZero(7),
      Print,
      JumpIfZero(5),
      Add(-1),
      JumpIfNonZero(3),
      Read,
      JumpIfNonZero(1),
    ],
    [101, 99, 104, 111],
    [101, 99, 104, 111] // "echo"
  );

  #[cfg_attr(rustfmt, rustfmt_skip)]
  test!(
    test_real_program,
    [
      Move(1), Add(1), Add(1), Add(1), Add(1), Add(1), JumpIfZero(25), Add(-1),
      Move(-1), Add(1), Add(1), Add(1), Move(1), Move(1), Add(1), Add(1),
      Add(1), Add(1), Add(1), Add(1), Move(1), Add(1), Add(1), Move(-1),
      Move(-1), JumpIfNonZero(6), Move(-1), JumpIfZero(38), Add(-1), Move(1),
      Add(1), Add(1), Add(1), Add(1), Add(1), Add(1), Add(1), Move(-1),
      JumpIfNonZero(27), Move(1), Add(-1), Print, Add(1), Print, Move(1),
      Add(1), Add(1), Add(1), Print, Move(1), Print,
    ],
    [],
    [104, 105, 33, 10] // "hi!\n"
  );

  test_io_error!(test_read_io_error, [Read], Error::Io(_));
  test_io_error!(test_write_io_error, [Print], Error::Io(_));
}
