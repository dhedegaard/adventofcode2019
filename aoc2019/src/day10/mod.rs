use std::collections::HashSet;
use std::iter::FromIterator;

pub fn raw_input() -> String {
  include_str!("input.txt").to_string()
}

pub fn parse_input(input: &str) -> HashSet<(isize, isize)> {
  input
    .split_whitespace()
    .enumerate()
    .flat_map(|(y, line)| {
      line
        .chars()
        .enumerate()
        .filter(|&(_, c)| c == '#')
        .map(move |(x, _)| (x as isize, y as isize))
    })
    .collect()
}

fn gcd(a: isize, b: isize) -> isize {
  let mut _a = a.abs();
  let mut _b = b.abs();
  while _b != 0 {
    let t = _b;
    _b = _a % _b;
    _a = t;
  }
  _a
}

fn get_slopes(width: isize, height: isize) -> Vec<(isize, isize)> {
  let mut result = vec![];
  for y in -height..height + 1 {
    for x in -width..width + 1 {
      if gcd(x, y) == 1 {
        result.push((x, y));
      }
    }
  }
  result
}

fn traverse_slope(
  asteroids: &HashSet<(isize, isize)>,
  (x, y): (isize, isize),
  (dx, dy): (isize, isize),
  width: isize,
  height: isize,
) -> Option<(isize, isize)> {
  let (mut new_x, mut new_y) = (x, y);
  loop {
    new_x += dx;
    new_y += dy;
    if new_x >= height || new_x < 0 || new_y >= width || new_y < 0 {
      return None;
    }
    if asteroids.contains(&(new_x, new_y)) {
      return Some((new_x, new_y));
    }
  }
  None
}

pub fn part1(input: &HashSet<(isize, isize)>) -> usize {
  let width = 2 + input.iter().map(|&(x, _)| x).max().unwrap() as isize;
  let height = 2 + input.iter().map(|&(_, y)| y).max().unwrap() as isize;
  let slopes = get_slopes(width, height);
  let mut max = 0;
  for asteroid in input {
    let count = slopes
      .iter()
      .map(|&(dx, dy)| traverse_slope(input, *asteroid, (dx, dy), width, height))
      .filter(|e| e.is_some())
      .count();
    if count > max {
      max = count;
    }
  }
  max
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse() {
    let result: Vec<(isize, isize)> = vec![
      (1, 0),
      (4, 0),
      (0, 2),
      (1, 2),
      (2, 2),
      (3, 2),
      (4, 2),
      (4, 3),
      (4, 4),
      (3, 4),
    ];
    assert_eq!(
      parse_input(include_str!("test1.txt")),
      HashSet::from_iter(result.iter().cloned())
    );
  }

  #[test]
  fn test_gcd() {
    assert_eq!(gcd(1, 2), 1);
    assert_eq!(gcd(3, 12), 3);
    assert_eq!(gcd(54, 24), 6);
  }

  #[test]
  fn test_part1_example1() {
    assert_eq!(part1(&parse_input(include_str!("test1.txt"))), 8);
  }

  #[test]
  fn test_part1_example2() {
    assert_eq!(part1(&parse_input(include_str!("test2.txt"))), 33);
  }

  #[test]
  fn test_part1_example3() {
    assert_eq!(part1(&parse_input(include_str!("test3.txt"))), 35);
  }

  #[test]
  fn test_part1_example4() {
    assert_eq!(part1(&parse_input(include_str!("test4.txt"))), 41);
  }

  #[test]
  fn test_part1_example5() {
    assert_eq!(part1(&parse_input(include_str!("test5.txt"))), 210);
  }

  #[test]
  fn test_part1_result() {
    assert_eq!(part1(&parse_input(&raw_input())), 214);
  }
}
