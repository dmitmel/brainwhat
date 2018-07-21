use std::io;
use std::io::{Read, Write};

const NEXT_CELL: char = '>';
const PREV_CELL: char = '<';
const INCREMENT: char = '+';
const DECREMENT: char = '-';
const PRINT_CHAR: char = '.';
const READ_CHAR: char = ',';
const BEGIN_LOOP: char = '[';
const END_LOOP: char = ']';

pub struct Interpreter {
  memory: Vec<u8>,
  pointer: usize,
}

impl Interpreter {
  pub fn new(memory_size: usize) -> Self {
    Interpreter {
      memory: vec![0; memory_size],
      pointer: 0,
    }
  }

  pub fn run(&mut self, program: &[char]) {
    let mut char_index = 0;

    while char_index < program.len() {
      match program[char_index] {
        NEXT_CELL => self.next_cell(),
        PREV_CELL => self.prev_cell(),

        INCREMENT => self.increment(),
        DECREMENT => self.decrement(),

        PRINT_CHAR => self.print_char().unwrap(),
        READ_CHAR => self.read_char().unwrap(),

        BEGIN_LOOP => if self.read_memory() == 0 {
          char_index = find_end_of_loop(char_index, &program)
        },

        END_LOOP => if self.read_memory() != 0 {
          char_index = find_beggining_of_loop(char_index, &program)
        },

        _ => {}
      }

      char_index += 1;
    }
  }

  fn next_cell(&mut self) {
    if self.pointer >= self.memory.len() - 1 {
      self.pointer = 0;
    } else {
      self.pointer += 1;
    }
  }

  fn prev_cell(&mut self) {
    if self.pointer == 0 {
      self.pointer = self.memory.len() - 1;
    } else {
      self.pointer -= 1;
    }
  }

  fn increment(&mut self) {
    let value = self.read_memory().wrapping_add(1);
    self.store_memory(value);
  }

  fn decrement(&mut self) {
    let value = self.read_memory().wrapping_sub(1);
    self.store_memory(value);
  }

  fn print_char(&self) -> io::Result<()> {
    let chr = self.read_memory();
    let mut stdout = io::stdout();
    stdout.write_all(&[chr])?;
    stdout.flush()?;
    Ok(())
  }

  fn read_char(&mut self) -> io::Result<()> {
    let mut buffer = [0; 1];
    let bytes = io::stdin().read(&mut buffer)?;
    self.store_memory(if bytes > 0 { buffer[0] } else { 0 });
    Ok(())
  }

  fn read_memory(&self) -> u8 {
    self.memory[self.pointer]
  }

  fn store_memory(&mut self, value: u8) {
    self.memory[self.pointer] = value
  }
}

fn find_end_of_loop(beginning_index: usize, program: &[char]) -> usize {
  let mut char_index = beginning_index;
  let mut brackets = 1;
  while brackets > 0 {
    char_index += 1;
    match program[char_index] {
      BEGIN_LOOP => brackets += 1,
      END_LOOP => brackets -= 1,
      _ => {}
    }
  }
  char_index
}

fn find_beggining_of_loop(end_index: usize, program: &[char]) -> usize {
  let mut char_index = end_index;
  let mut brackets = 1;
  while brackets > 0 {
    char_index -= 1;
    match program[char_index] {
      BEGIN_LOOP => brackets -= 1,
      END_LOOP => brackets += 1,
      _ => {}
    }
  }
  char_index
}
