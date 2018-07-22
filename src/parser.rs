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

pub fn parse(program: &[char]) -> Vec<Instruction> {
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

    instructions.push(instruction);
  }

  let mut instruction_index = 0;
  while instruction_index < instructions.len() {
    instructions[instruction_index] = match instructions[instruction_index] {
      JumpIfZero(_) => {
        JumpIfZero(find_end_of_loop(instruction_index, &instructions))
      }
      JumpIfNonZero(_) => {
        JumpIfNonZero(find_beggining_of_loop(instruction_index, &instructions))
      }
      _ => {
        instruction_index += 1;
        continue;
      }
    };

    instruction_index += 1;
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

fn find_end_of_loop(beginning_index: usize, program: &[Instruction]) -> usize {
  let mut char_index = beginning_index;
  let mut brackets = 1;
  while brackets > 0 {
    char_index += 1;
    match program[char_index] {
      JumpIfZero(_) => brackets += 1,
      JumpIfNonZero(_) => brackets -= 1,
      _ => {}
    }
  }
  char_index
}

fn find_beggining_of_loop(end_index: usize, program: &[Instruction]) -> usize {
  let mut char_index = end_index;
  let mut brackets = 1;
  while brackets > 0 {
    char_index -= 1;
    match program[char_index] {
      JumpIfZero(_) => brackets -= 1,
      JumpIfNonZero(_) => brackets += 1,
      _ => {}
    }
  }
  char_index
}
