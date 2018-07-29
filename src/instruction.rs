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

#[cfg(test)]
mod tests {
  use super::*;
  use optimizer::optimize;
  use parser::parse;

  #[test]
  fn test_display() {
    // this program is perfect for this test because it contains every
    // instruction, nested loops and comments
    let source_code = "
>>>+[[-]>>[-]++>+>+++++++[<++++>>++<-]++>>+>+>+++++[>++>++++++<<-]+>>>,<++[[>[
->>]<[>>]<<-]<[<]<+>>[>]>[<+>-[[<+>-]>]<[[[-]<]++<-[<+++++++++>[<->-]>>]>>]]<<
]<]<[[<]>[[>]>>[>>]+[<<]<[<]<+>>-]>[>]+[->>]<<<<[[<<]<[<]+<<[+>+<<-[>-->+<<-[>
+<[>>+<<-]]]>[<+>-]<]++>>-->[>]>>[>>]]<<[>>+<[[<]<]>[[<<]<[<]+[-<+>>-[<<+>++>-
[<->[<<+>>-]]]<[>+<-]>]>[>]>]>[>>]>>]<<[>>+>>+>>]<<[->>>>>>>>]<<[>.>>>>>>>]<<[
>->>>>>]<<[>,>>>]<<[>+>]<<[+<<]<]
[input a brainfuck program and its input, separated by an exclamation point.
Daniel B Cristofani (cristofdathevanetdotcom)
http://www.hevanet.com/cristofd/brainfuck/]
";
    let source_code_chars = source_code.chars().collect::<Vec<_>>();

    let expected_code_chars = source_code_chars
      .clone()
      .into_iter()
      .filter(|chr| match chr {
        '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => true,
        _ => false,
      })
      .collect::<Vec<char>>();

    fn program_to_chars(program: &[Instruction]) -> Vec<char> {
      program
        .into_iter()
        .fold(Vec::new(), |mut result, instruction| {
          result.extend(instruction.to_string().chars());
          result
        })
    }

    let parsed_program = parse(&source_code_chars).unwrap();
    assert_eq!(program_to_chars(&parsed_program), expected_code_chars);

    let optimized_program = optimize(&parsed_program).unwrap();
    assert_eq!(program_to_chars(&optimized_program), expected_code_chars);
  }
}
