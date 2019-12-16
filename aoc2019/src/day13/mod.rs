#[path = "../intcode/mod.rs"]
mod intcode;

use std::cmp::Ordering;

pub fn raw_input() -> String {
  include_str!("input.txt").to_string()
}

#[derive(PartialEq)]
pub enum Tile {
  Empty,
  Wall,
  Block,
  HorizPaddle,
  Ball,
}

pub fn parse_input(input: &str) -> intcode::Intcode {
  let insts = input
    .split(",")
    .map(|e| e.parse::<i64>().unwrap())
    .collect::<Vec<_>>();
  intcode::Intcode::new(&insts, &[])
}

pub fn part1(program: &mut intcode::Intcode) -> usize {
  let mut result = 0;
  while let intcode::IntcodeState::Output(_x) = program.execute() {
    program.execute();
    match program.execute() {
      intcode::IntcodeState::Output(e) => {
        if e == 2 {
          result += 1;
        }
      }
      _ => {}
    }
  }
  result
}

pub fn part2(mut program: intcode::Intcode) -> i64 {
  program.insts[0] = 2;

  let mut ball_x = -1;
  let mut paddle_x = -1;
  let mut output = 0;
  loop {
    let x = match program.execute() {
      intcode::IntcodeState::Halt => return output,
      intcode::IntcodeState::Output(e) => e,
      intcode::IntcodeState::NeedInput => {
        program.input.push(match ball_x.cmp(&paddle_x) {
          Ordering::Less => -1,
          Ordering::Equal => 0,
          Ordering::Greater => 1,
        });
        continue;
      }
      _ => panic!(),
    };

    let y = match program.execute() {
      intcode::IntcodeState::Output(e) => e,
      _ => panic!(),
    };
    let z = match program.execute() {
      intcode::IntcodeState::Output(e) => e,
      _ => panic!(),
    };
    if x == -1 && y == 0 {
      output = z;
    } else {
      match z {
        3 => paddle_x = x,
        4 => ball_x = x,
        _ => {}
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_result() {
    assert_eq!(part1(&mut parse_input(&raw_input())), 376);
  }

  #[test]
  fn part2_result() {
    assert_eq!(part2(parse_input(&raw_input())), 18509);
  }
}
