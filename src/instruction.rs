use std::fmt;

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum Instruction {
  Move(isize),
  Add(isize),
  Print,
  Read,
  JumpIfZero(usize),
  JumpIfNonZero(usize),
}

impl fmt::Display for Instruction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use Instruction::*;

    write!(
      f,
      "{}",
      match self {
        Move(n) if *n > 0 => ">".repeat(*n as usize),
        Move(n) if *n < 0 => "<".repeat(n.abs() as usize),

        Add(n) if *n > 0 => "+".repeat(*n as usize),
        Add(n) if *n < 0 => "-".repeat(n.abs() as usize),

        Print => ".".to_owned(),
        Read => ",".to_owned(),

        JumpIfZero(_) => "[".to_owned(),
        JumpIfNonZero(_) => "]".to_owned(),

        _ => String::default(),
      }
    )
  }
}
