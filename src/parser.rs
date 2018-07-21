pub enum Instruction {
  Right(usize),
  Left(usize),
  Add(usize),
  Subtract(usize),
  Print,
  Read,
  BeginLoop,
  EndLoop,
}

pub fn parse(program: &[char]) -> Vec<Instruction> {
  use self::Instruction::*;

  let mut instructions = Vec::new();

  let mut char_index = 0;
  while char_index < program.len() {
    let instruction = match program[char_index] {
      '>' => Right(count_char(program, char_index, '>')),
      '<' => Left(count_char(program, char_index, '<')),
      '+' => Add(count_char(program, char_index, '+')),
      '-' => Subtract(count_char(program, char_index, '-')),
      '.' => Print,
      ',' => Read,
      '[' => BeginLoop,
      ']' => EndLoop,
      _ => {
        char_index += 1;
        continue;
      }
    };

    char_index += match instruction {
      Right(n) | Left(n) | Add(n) | Subtract(n) => n,
      _ => 1,
    };

    instructions.push(instruction);
  }

  instructions
}

fn count_char(program: &[char], start_index: usize, chr: char) -> usize {
  let mut end_index = start_index + 1;
  while end_index < program.len() && program[end_index] == chr {
    end_index += 1;
  }
  end_index - start_index
}
