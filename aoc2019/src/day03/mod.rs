use std::collections::hash_set::HashSet;
use std::iter::FromIterator;

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
    char => panic!("Unknown instruction: {}", char),
  };
  let distance = instruction.chars().skip(1).collect::<String>();
  return Instruction {
    direction,
    distance: distance.parse::<i32>().unwrap(),
  };
}

fn wire_to_positions(wire: &[Instruction]) -> Vec<(i32, i32)> {
  let mut cur_pos = (0, 0);
  wire
    .iter()
    .map(|inst| {
      let mut res = vec![];
      for _ in 0..inst.distance {
        match inst.direction {
          Direction::U => {
            cur_pos = (cur_pos.0, cur_pos.1 + 1);
          }
          Direction::D => {
            cur_pos = (cur_pos.0, cur_pos.1 - 1);
          }
          Direction::L => {
            cur_pos = (cur_pos.0 - 1, cur_pos.1);
          }
          Direction::R => {
            cur_pos = (cur_pos.0 + 1, cur_pos.1);
          }
        };
        res.push(cur_pos);
      }
      res
    })
    .flatten()
    .collect()
}

fn find_intersections(wire1: &[(i32, i32)], wire2: &[(i32, i32)]) -> Vec<(i32, i32)> {
  let set1: HashSet<(i32, i32)> = HashSet::from_iter(wire1.into_iter().cloned());
  let set2: HashSet<(i32, i32)> = HashSet::from_iter(wire2.into_iter().cloned());
  set1.intersection(&set2).cloned().collect()
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
  let intersections = find_intersections(&wire1_positions, &wire2_positions);
  intersections
    .iter()
    .map(|e| e.0.abs() + e.1.abs())
    .min()
    .unwrap()
}

pub fn part2(wires: &[Vec<String>]) -> i32 {
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
  let intersections = find_intersections(&wire1_positions, &wire2_positions);
  intersections
    .iter()
    .map(|pos| {
      wire1_positions.iter().position(|x| x == pos).unwrap()
        + wire2_positions.iter().position(|x| x == pos).unwrap()
        + 2 // offset the index of 0
    })
    .min()
    .unwrap() as i32
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

  #[test]
  fn test_part2() {
    assert_eq!(part2(&parse_input(include_str!("test1.txt"))), 30);
    assert_eq!(part2(&parse_input(include_str!("test2.txt"))), 610);
    assert_eq!(part2(&parse_input(include_str!("test3.txt"))), 410);
  }

  #[test]
  fn result_part2() {
    assert_eq!(part2(&parse_input(&raw_input())), 21666);
  }
}
