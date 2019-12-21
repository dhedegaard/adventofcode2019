#[path = "../intcode/mod.rs"]
mod intcode;

pub fn raw_input() -> String {
  include_str!("input.txt").to_string()
}

fn raw_part1() -> String {
  include_str!("part1.txt").to_string()
}

fn raw_part2() -> String {
  include_str!("part2.txt").to_string()
}

pub fn parse_input(input: &str) -> Vec<i64> {
  input
    .split(',')
    .map(|e| e.replace(",", "").replace("+", "").parse::<_>().unwrap())
    .collect()
}

pub fn part1(input: &[i64]) -> i64 {
  let mut program = intcode::Intcode::new(
    input,
    &raw_part1()
      .chars()
      .filter(|c| *c != '\r')
      .map(|c| c as u8 as i64)
      .collect::<Vec<_>>(),
  );
  program.input.push('\n' as u8 as i64);
  let mut output = 0;
  loop {
    match program.execute() {
      intcode::IntcodeState::Halt => break,
      intcode::IntcodeState::Output(e) => output = e,
      _ => {}
    }
  }
  output
}

pub fn part2(input: &[i64]) -> i64 {
  let mut program = intcode::Intcode::new(
    input,
    &raw_part2()
      .chars()
      .filter(|c| *c != '\r')
      .map(|c| c as u8 as i64)
      .collect::<Vec<_>>(),
  );
  program.input.push('\n' as u8 as i64);
  let mut output = 0;
  loop {
    match program.execute() {
      intcode::IntcodeState::Halt => break,
      intcode::IntcodeState::Output(e) => output = e,
      _ => {}
    }
  }
  output
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_result() {
    assert_eq!(part1(&parse_input(&raw_input())), 19348840);
  }

  #[test]
  fn part2_result() {
    assert_eq!(part2(&parse_input(&raw_input())), 1141857182);
  }
}
