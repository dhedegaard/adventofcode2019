use std::collections::hash_set::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
  x: i32,
  y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
  U,
  D,
  L,
  R,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Instruction {
  direction: Direction,
  distance: i32,
}

pub fn raw_input() -> String {
  include_str!("input.txt").to_string()
}

pub fn parse_input(input: &str) -> Vec<Vec<String>> {
  input
    .replace("\r", "")
    .split('\n')
    .map(|line| line.split(",").map(|e| e.to_string()).collect::<Vec<_>>())
    .collect::<Vec<_>>()
}

fn parse_instruction(instruction: &str) -> Instruction {
  let direction = match instruction.chars().next().unwrap() {
    'U' => Direction::U,
    'D' => Direction::D,
    'L' => Direction::L,
    'R' => Direction::R,
    _ => unreachable!(),
  };
  let distance = instruction.chars().skip(1).collect::<String>();
  return Instruction {
    direction,
    distance: distance.parse::<i32>().unwrap(),
  };
}

fn wire_to_positions(wire: &[Instruction]) -> HashSet<Point> {
  let mut result: HashSet<Point> = HashSet::new();
  let mut cur_pos = Point { x: 0, y: 0 };
  for inst in wire {
    for _ in 0..inst.distance {
      match inst.direction {
        Direction::U => {
          cur_pos.y -= 1;
        }
        Direction::D => {
          cur_pos.y += 1;
        }
        Direction::L => {
          cur_pos.x -= 1;
        }
        Direction::R => {
          cur_pos.x += 1;
        }
      };
      result.insert(cur_pos);
    }
  }
  result
}

pub fn part1(wires: &[Vec<String>]) -> i32 {
  let wire1_positions = wire_to_positions(
    &wires[0]
      .iter()
      .map(|e| parse_instruction(e))
      .collect::<Vec<_>>(),
  );
  let wire2_positions = wire_to_positions(
    &wires[1]
      .iter()
      .map(|e| parse_instruction(e))
      .collect::<Vec<_>>(),
  );
  let intersections = wire1_positions.intersection(&wire2_positions);
  intersections.map(|e| e.x.abs() + e.y.abs()).min().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1() {
    assert_eq!(part1(&parse_input(include_str!("test1.txt"))), 6);
    assert_eq!(part1(&parse_input(include_str!("test2.txt"))), 159);
    assert_eq!(part1(&parse_input(include_str!("test3.txt"))), 135);
  }

  #[test]
  fn result_part1() {
    assert_eq!(part1(&parse_input(&raw_input())), 2050);
  }
}
