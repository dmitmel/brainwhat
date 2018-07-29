use error::{Error, Result};
use instruction::{Instruction, Instruction::*};

const TYPICAL_LOOP_NESTING: usize = 10;

pub fn parse(code: &[char]) -> Result<Vec<Instruction>> {
  let mut program = Vec::with_capacity(code.len());

  for chr in code {
    program.push(match chr {
      '>' => Move(1),
      '<' => Move(-1),
      '+' => Add(1),
      '-' => Add(-1),
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
  let mut jump_stack = Vec::with_capacity(TYPICAL_LOOP_NESTING);
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

#[cfg(test)]
mod tests {
  use super::*;

  fn test_parse(code: &str) -> Result<Vec<Instruction>> {
    let code_chars = code.chars().collect::<Vec<char>>();
    parse(&code_chars)
  }

  macro_rules! test {
    ($name:ident, $code:expr, $expected_program:expr) => {
      #[test]
      fn $name() {
        let expected_program: &[Instruction] = &$expected_program;
        let actual_program = test_parse($code).unwrap();
        assert_eq!(expected_program, &actual_program[..]);
      }
    };
  }

  macro_rules! test_error {
    ($name:ident, $code:expr, $expected_error:pat) => {
      #[test]
      fn $name() {
        let actual_error = test_parse($code).unwrap_err();
        assert!(match actual_error {
          $expected_error => true,
          _ => false,
        })
      }
    };
  }

  test!(test_empty, "", []);

  test!(test_empty_comments, "hello world", []);

  test!(
    test_basic,
    "+>-<,.",
    [Add(1), Move(1), Add(-1), Move(-1), Read, Print]
  );

  test!(
    test_comments,
    "this + is > a - very < long , comment .",
    [Add(1), Move(1), Add(-1), Move(-1), Read, Print]
  );

  test!(
    test_unicode_comments,
    "це + довгий > коментар - это < длинный , комментарий . ¯\\_(ツ)_/¯",
    [Add(1), Move(1), Add(-1), Move(-1), Read, Print]
  );

  test!(
    test_simple_loops,
    "+[,.]",
    [Add(1), JumpIfZero(4), Read, Print, JumpIfNonZero(1)]
  );

  test!(
    test_nested_loops,
    ",[.[-],]",
    [
      Read,
      JumpIfZero(7),
      Print,
      JumpIfZero(5),
      Add(-1),
      JumpIfNonZero(3),
      Read,
      JumpIfNonZero(1),
    ]
  );

  test_error!(test_unclosed_simple_loops, "+[,.", Error::Syntax(_));
  test_error!(test_unclosed_nested_loops, ",[.[-,]", Error::Syntax(_));

  test_error!(
    test_unexpected_closing_simple_loops,
    "+[,.]]",
    Error::Syntax(_)
  );
  test_error!(
    test_unexpected_closing_nested_loops,
    ",.[-],]",
    Error::Syntax(_)
  );
}
