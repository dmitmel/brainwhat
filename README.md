# brainwhat

[![CI Status](https://github.com/dmitmel/brainwhat/workflows/CI/badge.svg)](https://github.com/dmitmel/brainwhat/actions?query=workflow:CI)

> A fast optimizing Brainfuck interpreter written in pure Rust

## What is `brainwhat`?

> `brainwhat` is a fast optimizing [Brainfuck](#what-is-brainfuck) interpreter written in pure [Rust](https://www.rust-lang.org/)

**fast** – Rust is a systems programming language, so it compiles to very fast native binaries.

**optimizing** – Brainfuck code is first parsed to a [list of instructions](https://en.wikipedia.org/wiki/Intermediate_representation), then [optimized](#optimizations) and after that these instructions are passed to the **interpreter**.

**interpreter** – it doesn't compile Brainfuck to another language, instead, it executes program directly.

## What is Brainfuck?

**Brainfuck** is an [esoteric programming language](https://en.wikipedia.org/wiki/Esoteric_programming_language) created by [Urban Müller](https://en.wikipedia.org/wiki/Brainfuck#History). It is not intended for practical use, but to challenge programmers. Despite its extreme minimalism, it is [Turing complete](https://en.wikipedia.org/wiki/Turing_completeness), so **it can solve any computation problem** with enough memory and time.

Brainfuck operates on an array of memory cells, where each cell is initially set to zero. Also there's a pointer, initially set to the first memory cell. Brainfuck has _**eight**_ commands (all other characters are ignored, so they can be used as comments):

| Command         | C equivalent                                                           | Description                                                   |
| --------------- | ---------------------------------------------------------------------- | ------------------------------------------------------------- |
| _Program start_ | `int ptr = 0;`<br>`char arr[30000];`<br>`memset(arr, 0, sizeof(arr));` |                                                               |
| `>`             | `ptr++;`                                                               | Move the pointer to the right                                 |
| `<`             | `ptr--;`                                                               | Move the pointer to the left                                  |
| `+`             | `arr[ptr]++;`                                                          | Increment value in the current cell                           |
| `-`             | `arr[ptr]--;`                                                          | Decrement value in the current cell                           |
| `.`             | `putchar(arr[ptr]);`                                                   | Print value in the current cell as an ASCII character         |
| `,`             | `arr[ptr] = getchar();`                                                | Read a character and put its ASCII value in the current cell  |
| `[`             | `while (arr[ptr]) {`                                                   | Jump past the matching `]` if the current cell is zero        |
| `]`             | `}`                                                                    | Jump back to the matching `[` if the current cell is not zero |

_For more information, check out [Wikipedia](https://en.wikipedia.org/wiki/Brainfuck), [Esolangs.org](https://esolangs.org/wiki/Brainfuck) and
[BrainFuck Programming Tutorial](https://gist.github.com/roachhd/dce54bec8ba55fb17d3a)._

## Project goals

This project should:

1.  have code that is easy to read for beginners (who already know [the concepts of Rust](https://doc.rust-lang.org/book/second-edition/index.html))
2.  not sacrifice readability for performance (thanks to the _magic_ of the Rust compiler)
3.  show that (safe) Rust is a really fast language
4.  be at least comparable to optimizing brainfuck interpreters written in C (e.g. [bff4](http://mazonka.com/brainf/)), but being faster than them would be nice too!

## Installation

```
cargo install --git https://github.com/dmitmel/brainwhat
```

## Usage

As a command-line tool:

```bash
brainwhat path/to/program.b
brainwhat < path/to/program.b
```

As a library (e.g. for [generating Brainfuck programs using AI](http://www.primaryobjects.com/2013/01/27/using-artificial-intelligence-to-write-self-modifying-improving-programs/)):

```rust
extern crate brainwhat;

// this program prints "hi!"
let code = ">+++++[-<+++>>++++++>++<<]<[->+++++++<]>-.+.>+++.>.";
let code_chars = code.chars().collect::<Vec<char>>();
// 4 memory cells is enough for this program
let memory_size = 4;

let parsed_program = brainwhat::parse(&code_chars)?;
let optimized_program = brainwhat::optimize(&parsed_program)?;
let mut interpreter = brainwhat::Interpreter::new(memory_size);
interpreter.run(&optimized_program)?;
```

### Implementation details

- The memory is constrained to 65,536 cells (64 KiB), although size of memory can be changed when using this interpreter as library.
- The interpreter will [panic](https://doc.rust-lang.org/std/macro.panic.html) on under- or overflows when moving the pointer.
- Cells have size of 1 byte.
- Addition and subtraction can over- and underflow.
- If EOF is reached when executing read (`,`) instruction, current cell will be set to zero.

## Benchmarks

**Brainfuck program:** DBFI (Brainfuck self-interpreter) which runs DBFI which runs simple program which prints `hello123`

**Baseline implementations:** [`bff`](https://github.com/apankrat/bff) and [`bff4`](http://mazonka.com/brainf/) (with linear optimizations turned off)

**Hardware:** _13-inch Early 2015 MacBook Pro_ with _Intel Core i7 @ 3.1 GHz_.

```
Benchmark #1: bin/bff bench_prog < bench_input
  Time (mean ± σ):     298.3 ms ±   4.0 ms    [User: 293.3 ms, System: 2.6 ms]
  Range (min … max):   292.0 ms … 303.8 ms

Benchmark #2: bin/bff4 < bench_bff4_input
  Time (mean ± σ):     247.9 ms ±   4.5 ms    [User: 242.7 ms, System: 2.8 ms]
  Range (min … max):   244.3 ms … 259.3 ms

Benchmark #3: bin/brainwhat bench_prog < bench_input
  Time (mean ± σ):     372.4 ms ±   7.1 ms    [User: 367.1 ms, System: 2.9 ms]
  Range (min … max):   364.3 ms … 383.2 ms

Summary
  'bin/bff4 < bench_bff4_input' ran
    1.20x faster than 'bin/bff bench_prog < bench_input'
    1.50x faster than 'bin/brainwhat bench_prog < bench_input'
```

## Optimizations

These are currently implemented optimizations, but I'm [planning](#todo) to add more.

### 1. Instruction stacking

Some instructions (`+`, `-`, `>`, `<`) are stackable, meaning that they can be merged into one instruction:

```
+++++ >>> ------- <<<<<< -> [Add(5), Move(3), Add(-7), Move(-6)]
12345 123 1234567 123456 <-- instruction counts
```

### 2. Loops

All loops are linked when parsing program, so interpreter doesn't have to search matching brackets at runtime (this optimization makes interpreter really fast!!!):

```
   __                            ____________________________
  /  \   <------ loops ------>  /                            \
 |    |                        |                              |
+[ ,. ] -> [Add(1), JumpIfZero(4), Read, Print, JumpIfNonZero(1)]
01 23 4 <-- instruction addresses
```

## TODO

- Documentation
- CLI with different options (e.g. memory/cell size, behavior on overflows etc)
- Different memory tapes (e.g. dynamic, finite etc)
- More optimizations
- Better handling of errors (i.e. print where the error has occurred)
- [Visualizer](https://fatiherikli.github.io/brainfuck-visualizer/) or [debugger](https://www.iamcal.com/misc/bf_debug/)
- More Brainfuck dialects (maybe)

## Contribute

PRs accepted.

_I would really appreciate your help with [TODOs](#todo)!_

## License

[MIT](LICENSE) © [Dmytro Meleshko](https://github.com/dmitmel)
