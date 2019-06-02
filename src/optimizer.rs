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

    JumpIfZero(_) => {
        match &program[index + 1 ..] {
            [Add(-1), JumpIfNonZero(_)] => {
                skip_chars += 2;
                Clear
            },
            _ => instruction,
        }
    },

      _ => instruction,
    });
  }

  link_jumps(&mut optimized_program)?;

  Ok(optimized_program)
}

#[cfg(test)]
mod tests {
  use super::*;
  use error::Error;

  macro_rules! test {
    ($name:ident, $program:expr, $expected_program:expr) => {
      #[test]
      fn $name() {
        let expected_program: &[Instruction] = &$expected_program;
        let actual_program = optimize(&$program).unwrap();
        assert_eq!(&actual_program[..], expected_program);
      }
    };
  }

  macro_rules! test_error {
    ($name:ident, $program:expr, $expected_error:pat) => {
      #[test]
      fn $name() {
        let actual_error = optimize(&$program).unwrap_err();
        assert!(match actual_error {
          $expected_error => true,
          _ => false,
        })
      }
    };
  }

  test!(test_empty, [], []);

  test!(
    test_instruction_stacking,
    [
      Add(1),
      Add(1),
      Add(1),
      Add(1),
      Move(1),
      Move(1),
      Move(1),
      Add(-1),
      Add(-1),
      Add(-1),
      Add(-1),
      Add(-1),
      Move(-1),
      Move(-1),
    ],
    [Add(4), Move(3), Add(-5), Move(-2)]
  );

  test!(
    test_similar_instruction_stacking,
    [
      Add(1),
      Add(1),
      Add(1),
      Add(1),
      Add(-1),
      Add(-1),
      Add(-1),
      Add(-1),
      Add(-1),
      Move(1),
      Move(1),
      Move(1),
      Move(-1),
      Move(-1),
    ],
    [Add(-1), Move(1)]
  );

  test!(
      test_clear_pattern,
      [
      Add(1),
      JumpIfZero(0),
      Add(-1),
      JumpIfNonZero(0),
      ],
      [Add(1), Clear]
  );

  test!(
    test_link_loops,
    [
      Add(1),
      Add(1),
      Add(1),
      Add(1),
      JumpIfZero(12),
      Add(-1),
      Move(1),
      Move(1),
      Add(1),
      Add(1),
      Add(1),
      Move(-1),
      Move(-1),
      JumpIfNonZero(4),
    ],
    [
      Add(4),
      JumpIfZero(6),
      Add(-1),
      Move(2),
      Add(3),
      Move(-2),
      JumpIfNonZero(1),
    ]
  );

  test_error!(
    test_loop_syntax_errors,
    [Add(1), JumpIfZero(4), Read, Print],
    Error::Syntax(_)
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
    [
      Move(1), Add(5), JumpIfZero(11), Add(-1), Move(-1), Add(3), Move(2),
      Add(6), Move(1), Add(2), Move(-2), JumpIfNonZero(2), Move(-1),
      JumpIfZero(18), Add(-1), Move(1), Add(7), Move(-1), JumpIfNonZero(13),
      Move(1), Add(-1), Print, Add(1), Print, Move(1), Add(3), Print, Move(1),
      Print,
    ]
  );
}
