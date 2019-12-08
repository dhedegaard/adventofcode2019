#[path = "../intcode/mod.rs"]
mod intcode;

fn run_intcode(insts: &[i32], input: &[i32]) -> Vec<i32> {
  let mut program = intcode::Intcode::new(insts, input);
  program.run()
}

pub fn raw_input() -> String {
  include_str!("input.txt").to_string()
}

pub fn parse_input(input: &str) -> Vec<i32> {
  input
    .split(',')
    .map(|e| e.replace(",", "").replace("+", "").parse::<_>().unwrap())
    .collect()
}

pub fn part1(input: &str) -> i32 {
  *run_intcode(&parse_input(input), &[1]).last().unwrap()
}

pub fn part2(input: &str) -> i32 {
  *run_intcode(&parse_input(input), &[5]).last().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_io_example() {
    // Input -> output.
    assert_eq!(run_intcode(&[3, 0, 4, 0, 99], &[1337]), &[1337]);
  }

  #[test]
  fn result_part1() {
    assert_eq!(
      *run_intcode(&parse_input(&raw_input()), &[1])
        .last()
        .unwrap(),
      15314507
    );
  }

  #[test]
  fn result_part2() {
    assert_eq!(
      *run_intcode(&parse_input(&raw_input()), &[5])
        .last()
        .unwrap(),
      652726
    );
  }

  #[test]
  fn test_equals_examples1() {
    assert_eq!(
      run_intcode(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &[8]),
      vec![1]
    )
  }

  #[test]
  fn test_equals_examples2() {
    assert_eq!(
      run_intcode(&[3, 3, 1108, -1, 8, 3, 4, 3, 99], &[4]),
      vec![0]
    )
  }

  #[test]
  fn test_less_than_examples1() {
    assert_eq!(
      run_intcode(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8, -1, 8], &[4],),
      vec![1]
    )
  }

  #[test]
  fn test_less_than_examples2() {
    assert_eq!(
      run_intcode(&[3, 3, 1107, -1, 8, 3, 4, 3, 99], &[9]),
      vec![0]
    )
  }

  #[test]
  fn test_jump_position_example1() {
    assert_eq!(
      run_intcode(
        &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
        &[1],
      ),
      vec![1]
    );
  }

  #[test]
  fn test_jump_position_example2() {
    assert_eq!(
      run_intcode(&[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], &[0]),
      vec![0]
    );
  }
}
