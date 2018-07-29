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
      Add(_) => {
        let mut total = 0isize;
        for instruction in &program[index..] {
          if let Add(n) = instruction {
            total += n;
            skip_chars += 1;
          } else {
            break;
          }
        }
        skip_chars -= 1;

        Add(total)
      }

      Move(_) => {
        let mut total = 0isize;
        for instruction in &program[index..] {
          if let Move(n) = instruction {
            total += n;
            skip_chars += 1;
          } else {
            break;
          }
        }
        skip_chars -= 1;

        Move(total)
      }

      _ => instruction,
    });
  }

  link_jumps(&mut optimized_program)?;

  Ok(optimized_program)
}
