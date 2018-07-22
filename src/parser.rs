use error::{Error, Result};

#[derive(Debug)]
pub enum Instruction {
  Right(usize),
  Left(usize),
  Add(usize),
  Subtract(usize),
  Print,
  Read,
  JumpIfZero(usize),
  JumpIfNonZero(usize),
}

use self::Instruction::*;

pub type Program = Vec<Instruction>;

pub fn parse(code: &[char]) -> Result<Program> {
  let mut program = Vec::new();

  let mut char_index = 0;
  while char_index < code.len() {
    let instruction = match code[char_index] {
      '>' => Right(count_char(code, char_index, '>')),
      '<' => Left(count_char(code, char_index, '<')),
      '+' => Add(count_char(code, char_index, '+')),
      '-' => Subtract(count_char(code, char_index, '-')),
      '.' => Print,
      ',' => Read,
      '[' => JumpIfZero(0),
      ']' => JumpIfNonZero(0),
      _ => {
        char_index += 1;
        continue;
      }
    };

    char_index += match instruction {
      Right(n) | Left(n) | Add(n) | Subtract(n) => n,
      _ => 1,
    };

    program.push(instruction);
  }

  let mut instruction_index = 0;
  while instruction_index < program.len() {
    program[instruction_index] = match program[instruction_index] {
      JumpIfZero(_) => {
        let address = find_end_of_loop(instruction_index, &program)?;
        JumpIfZero(address)
      }

      JumpIfNonZero(_) => {
        let address = find_beggining_of_loop(instruction_index, &program)?;
        JumpIfNonZero(address)
      }

      _ => {
        instruction_index += 1;
        continue;
      }
    };

    instruction_index += 1;
  }

  Ok(program)
}

fn count_char(program: &[char], start_index: usize, chr: char) -> usize {
  let mut end_index = start_index + 1;
  while end_index < program.len() && program[end_index] == chr {
    end_index += 1;
  }
  end_index - start_index
}

fn find_end_of_loop(
  beginning_index: usize,
  program: &Program,
) -> Result<usize> {
  let mut index = beginning_index;
  let mut brackets = 1;
  while brackets > 0 {
    index += 1;

    if index >= program.len() {
      return Err(Error::Syntax("Unclosed bracket".to_owned()));
    }

    match program[index] {
      JumpIfZero(_) => brackets += 1,
      JumpIfNonZero(_) => brackets -= 1,
      _ => {}
    }
  }

  Ok(index)
}

fn find_beggining_of_loop(
  end_index: usize,
  program: &Program,
) -> Result<usize> {
  let mut index = end_index;
  let mut brackets = 1;
  while brackets > 0 {
    if index == 0 {
      return Err(Error::Syntax("Unexpected closing bracket".to_owned()));
    }

    index -= 1;
    match program[index] {
      JumpIfZero(_) => brackets -= 1,
      JumpIfNonZero(_) => brackets += 1,
      _ => {}
    }
  }

  Ok(index)
}
