pub fn raw_input() -> String {
  include_str!("input.txt").to_string()
}

pub fn parse_input(input: &str) -> Vec<i32> {
  input
    .split_whitespace()
    .map(|e| e.replace(",", "").replace("+", "").parse::<_>().unwrap())
    .collect()
}

pub fn part1(input: &[i32]) -> i32 {
  input.iter().map(|e| *e / 3 - 2).sum()
}

pub fn part2(input: &[i32]) -> i32 {
  input
    .iter()
    .map(|e| {
      let mut total = 0;
      let mut fuel = e / 3 - 2;
      while fuel > 0 {
        total += fuel;
        fuel = fuel / 3 - 2;
      }
      total
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn examples_part1() {
    assert_eq!(part1(&[12]), 2);
    assert_eq!(part1(&[14]), 2);
    assert_eq!(part1(&[1969]), 654);
    assert_eq!(part1(&[100756]), 33583);
  }

  #[test]
  fn result_part1() {
    assert_eq!(part1(&parse_input(&raw_input())), 3412094);
  }

  #[test]
  fn examples_part2() {
    assert_eq!(part2(&[14]), 2);
    assert_eq!(part2(&[1969]), 966);
    assert_eq!(part2(&[100756]), 50346);
  }

  #[test]
  fn result_part2() {
    assert_eq!(part2(&parse_input(&raw_input())), 5115267);
  }
}
