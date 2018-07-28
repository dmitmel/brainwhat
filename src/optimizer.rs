use error::Result;

use instruction::Instruction;
use parser::link_jumps;
use Instruction::*;

pub fn optimize(program: &[Instruction]) -> Result<Vec<Instruction>> {
  let mut optimized_program = Vec::with_capacity(program.len());

  let mut skip_chars = 0;

  for index in 0..program.len() {
    if skip_chars > 0 {
      skip_chars -= 1;
      continue;
    }

    let instruction = program[index];

    optimized_program.push(match instruction {
      Right(_) | Left(_) | Add(_) | Subtract(_) => {
        let n = count_instruction(&program[index..], instruction);
        skip_chars = n - 1;

        match instruction {
          Right(_) => Right(n),
          Left(_) => Left(n),
          Add(_) => Add(n),
          Subtract(_) => Subtract(n),
          _ => unreachable!(),
        }
      }
      _ => instruction,
    });
  }

  link_jumps(&mut optimized_program)?;

  Ok(optimized_program)
}

fn count_instruction(
  program_slice: &[Instruction],
  instruction: Instruction,
) -> usize {
  let mut count = 1;
  while count < program_slice.len() && program_slice[count] == instruction {
    count += 1;
  }
  count
}
