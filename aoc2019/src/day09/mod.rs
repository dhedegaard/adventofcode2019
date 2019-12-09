#[path = "../intcode/mod.rs"]
mod intcode;

pub fn raw_input() -> String {
  include_str!("input.txt").to_string()
}

pub fn parse_input(input: &str) -> Vec<i64> {
  input
    .split(',')
    .map(|e| e.replace(",", "").replace("+", "").parse::<_>().unwrap())
    .collect()
}

fn run_intcode(insts: &[i64], input: &[i64]) -> Vec<i64> {
  intcode::Intcode::new(insts, input).run()
}

pub fn part1(input: &[i64]) -> i64 {
  run_intcode(input, &[1])[0]
}

pub fn part2(input: &[i64]) -> i64 {
  run_intcode(input, &[2])[0]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example1() {
    assert_eq!(
      run_intcode(
        &[109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99],
        &[]
      ),
      vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
    );
  }

  #[test]
  fn test_example2() {
    assert_eq!(
      run_intcode(&[1102, 34915192, 34915192, 7, 4, 7, 99, 0], &[]),
      vec![1219070632396864],
    )
  }

  #[test]
  fn test_example3() {
    assert_eq!(
      run_intcode(&[104, 1125899906842624, 99], &[]),
      vec![1125899906842624]
    );
  }

  #[test]
  fn test_part1() {
    assert_eq!(part1(&parse_input(&raw_input())), 2671328082);
  }

  #[test]
  fn test_part2() {
    assert_eq!(part2(&parse_input(&raw_input())), 2671328082);
  }
}
