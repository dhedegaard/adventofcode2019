#[path = "../intcode/mod.rs"]
mod intcode;

use intcode::*;

pub fn raw_input() -> String {
  include_str!("input.txt").to_string()
}

pub fn parse_input(input: &str) -> Vec<i64> {
  input
    .split(",")
    .map(|e| e.parse::<i64>().unwrap())
    .collect()
}

fn build_grid(input: &[i64]) -> Vec<Vec<char>> {
  let mut program = Intcode::new(input, &[]);
  let mut result: Vec<Vec<char>> = vec![];
  let mut row: Vec<char> = vec![];
  while let IntcodeState::Output(o) = program.execute() {
    let c = o as u8 as char;
    match c {
      '.' | '#' | '^' => row.push(c),
      '\n' => {
        result.push(row);
        row = vec![];
      }
      _ => panic!("Unknown char: {}", c),
    }
  }
  if !row.is_empty() {
    result.push(row);
  }
  result
}

pub fn part1(input: &[i64]) -> usize {
  let grid = build_grid(input);
  let mut result = 0;
  for (y, row) in grid.iter().enumerate() {
    for (x, &c) in row.iter().enumerate() {
      if c == '#'
        && y > 0
        && x > 0
        && y < grid.len() - 2
        && x < row.len() - 2
        && grid[y - 1][x] == '#'
        && grid[y + 1][x] == '#'
        && grid[y][x + 1] == '#'
        && grid[y][x - 1] == '#'
      {
        result += x * y;
      }
    }
  }
  result
}

fn parse_function(input: &str) -> Vec<i64> {
  input.chars().map(|c| c as u8 as i64).collect()
}

fn functions_to_input(input: &[&str]) -> Vec<i64> {
  input.iter().map(|s| parse_function(s)).flatten().collect()
}

pub fn part2(input: &[i64]) -> i64 {
  let mut program = intcode::Intcode::new(
    input,
    &functions_to_input(&[
      // Main:
      "A,B,C,A,C,A,C\n",            // A:
      "L,6,R,12,L,4,L,6,R,6,L,6\n", // B:
      "R,12,R,6,L,6\n",             // C:
      "R,12,L,6,L,10,L,10,R,6\n",
      "Y\n",
    ]),
  );
  println!("input: {:?}", program.input);
  program.insts[0] = 2;
  let mut result = "".to_string();
  while let intcode::IntcodeState::Output(o) = program.execute() {
    result.push(o as u8 as char);
  }
  println!("{}", result);
  10
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_result() {
    assert_eq!(part1(&parse_input(&raw_input())), 6052);
  }

  // #[test]
  fn part2_result() {
    assert_eq!(part2(&parse_input(&raw_input())), 12);
  }
}
