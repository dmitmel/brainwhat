use error::{Error, Result};
use instruction::{Instruction, Instruction::*};

pub fn parse(code: &[char]) -> Result<Vec<Instruction>> {
  let mut program = Vec::with_capacity(code.len());

  for chr in code {
    program.push(match chr {
      '>' => Right(1),
      '<' => Left(1),
      '+' => Add(1),
      '-' => Subtract(1),
      '.' => Print,
      ',' => Read,
      '[' => JumpIfZero(0),
      ']' => JumpIfNonZero(0),
      _ => continue,
    });
  }

  link_jumps(&mut program)?;

  Ok(program)
}

pub(crate) fn link_jumps(program: &mut [Instruction]) -> Result<()> {
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
