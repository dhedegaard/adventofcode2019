#[path = "../intcode/mod.rs"]
mod intcode;

use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Copy, Debug, Clone)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
enum Color {
  White,
  Black,
}

pub fn raw_input() -> String {
  include_str!("input.txt").to_string()
}

pub fn parse_input(input: &str) -> Vec<i64> {
  input
    .split(',')
    .map(|e| e.replace(",", "").replace("+", "").parse::<_>().unwrap())
    .collect()
}

fn run_intcode(insts: &[i64]) -> Vec<i64> {
  intcode::Intcode::new(insts, &[]).run()
}

fn determine_new_direction(old_direction: &Direction, inst: i64) -> Direction {
  match old_direction {
    Direction::Up => match inst {
      0 => Direction::Left,
      1 => Direction::Right,
      _ => panic!("Unmapped inst: {}", inst),
    },
    Direction::Right => match inst {
      0 => Direction::Up,
      1 => Direction::Down,
      _ => panic!("Unmapped inst: {}", inst),
    },
    Direction::Down => match inst {
      0 => Direction::Right,
      1 => Direction::Left,
      _ => panic!("Unmapped inst: {}", inst),
    },
    Direction::Left => match inst {
      0 => Direction::Down,
      1 => Direction::Up,
      _ => panic!("Unmapped inst: {}", inst),
    },
  }
}

fn part1(insts: &[i64]) -> usize {
  let mut painting: HashMap<(i64, i64), Color> = HashMap::new();
  let mut pos = (0, 0);
  let mut dir = Direction::Up;
  let mut program = intcode::Intcode::new(insts, &[0]);
  let mut output: VecDeque<i64> = VecDeque::new();
  loop {
    match program.execute() {
      intcode::IntcodeState::Output(e) => output.push_back(e),
      intcode::IntcodeState::Halt => break,
      intcode::IntcodeState::NeedInput => program.input.push(match painting.get(&pos) {
        Some(color) => {
          if *color == Color::White {
            1
          } else {
            0
          }
        }
        None => 0, // Default color is black
      }),
      _ => {}
    };
    while output.len() >= 2 {
      // Determine the color and direction.
      let color = if output.pop_front().unwrap() == 0 {
        Color::Black
      } else {
        Color::White
      };
      dir = determine_new_direction(&dir, output.pop_front().unwrap());
      // Paint the current position.
      painting.insert(pos, color);
      // Move in the new direction.
      pos = match dir {
        Direction::Up => (pos.0, pos.1 - 1),
        Direction::Down => (pos.0, pos.1 + 1),
        Direction::Left => (pos.0 - 1, pos.1),
        Direction::Right => (pos.0 + 1, pos.1),
      };
    }
  }
  painting.len()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1_result() {
    assert_eq!(part1(&parse_input(&raw_input())), 1564);
  }
}
