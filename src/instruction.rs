#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
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
