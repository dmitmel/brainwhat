#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum Instruction {
  Move(isize),
  Add(isize),
  Print,
  Read,
  JumpIfZero(usize),
  JumpIfNonZero(usize),
}
