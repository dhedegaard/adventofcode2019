use wasm_bindgen::prelude::*;

use std::collections::HashSet;

mod day01 {
  use super::*;

  #[wasm_bindgen(js_name = "day01__raw_input")]
  pub fn raw_input() -> String {
    include_str!("input.txt").to_string()
  }

  #[wasm_bindgen(js_name = "day01__parse_input")]
  pub fn parse_input(input: &str) -> Vec<i32> {
    input
      .split_whitespace()
      .map(|e| e.replace(",", "").replace("+", "").parse::<_>().unwrap())
      .collect()
  }

  #[wasm_bindgen(js_name = "day01__part1")]
  pub fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
  }

  #[wasm_bindgen(js_name = "day01__part2")]
  pub fn part2(input: &[i32]) -> i32 {
    let mut result = 0;
    let mut seen_numbers = HashSet::with_capacity(133_000);
    for elem in input.iter().cycle() {
      if !seen_numbers.insert(result) {
        return result;
      }
      result += elem;
    }
    unreachable!()
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
      assert_eq!(parse_input("+1, +1, +1"), vec![1, 1, 1]);
      assert_eq!(parse_input("+1, +1, -2"), vec![1, 1, -2]);
      assert_eq!(parse_input("-1, -2, -3"), vec![-1, -2, -3]);
    }

    #[test]
    fn examples_part1() {
      assert_eq!(part1(&parse_input("+1, -2, +3, +1")), 3);
      assert_eq!(part1(&parse_input("+1, +1, +1")), 3);
      assert_eq!(part1(&parse_input("+1, +1, -2")), 0);
      assert_eq!(part1(&parse_input("-1, -2, -3")), -6);
    }

    #[test]
    fn result_part1() {
      assert_eq!(part1(&parse_input(&raw_input())), 516);
    }

    #[test]
    fn examples_part2() {
      assert_eq!(part2(&parse_input("+1, -2, +3, +1")), 2);
      assert_eq!(part2(&parse_input("+1, -1")), 0);
      assert_eq!(part2(&parse_input("+3, +3, +4, -2, -4")), 10);
      assert_eq!(part2(&parse_input("-6, +3, +8, +5, -6")), 5);
      assert_eq!(part2(&parse_input("+7, +7, -2, -7, -4")), 14);
    }

    #[test]
    fn result_part2() {
      assert_eq!(part2(&parse_input(&raw_input())), 71892);
    }
  }
}
