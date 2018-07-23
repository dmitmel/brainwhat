use error::{Error, Result};
use instruction::{Instruction, Instruction::*};

pub fn parse(code: &[char]) -> Result<Vec<Instruction>> {
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

  link_jumps(&mut program)?;

  Ok(program)
}

fn count_char(code: &[char], start_index: usize, chr: char) -> usize {
  let mut end_index = start_index + 1;
  while end_index < code.len() && code[end_index] == chr {
    end_index += 1;
  }
  end_index - start_index
}

fn link_jumps(program: &mut [Instruction]) -> Result<()> {
  let mut jump_stack = Vec::with_capacity(15);
  for isntruction_index in 0..program.len() {
    match program[isntruction_index] {
      JumpIfZero(_) => {
        jump_stack.push(isntruction_index);
      }
      JumpIfNonZero(_) => {
        let jump_index = jump_stack.pop().ok_or_else(|| {
          Error::Syntax("Unexpected closing bracket".to_owned())
        })?;

        program[isntruction_index] = JumpIfNonZero(jump_index);
        program[jump_index] = JumpIfZero(isntruction_index);
      }
      _ => {}
    }
  }

  if jump_stack.is_empty() {
    Ok(())
  } else {
    Err(Error::Syntax("Unclosed bracket".to_owned()))
  }
}
