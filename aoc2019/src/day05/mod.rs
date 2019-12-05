#[path = "../day02/mod.rs"]
mod day02;

pub fn raw_input() -> String {
  include_str!("input.txt").to_string()
}

// pub fn part1(input: &str) -> usize {
//   let (from, to) = parse_input(input);
//   let mut valid_passwords = 0;
//   for val in from.parse::<i32>().unwrap()..to.parse::<i32>().unwrap() {
//     if is_valid_password(&val.to_string()) {
//       valid_passwords += 1;
//     }
//   }
//   valid_passwords
// }

// pub fn part2(input: &str) -> usize {
//   let (from, to) = parse_input(input);
//   let mut valid_passwords = 0;
//   for val in from.parse::<i32>().unwrap()..to.parse::<i32>().unwrap() {
//     if is_valid_password2(&val.to_string()) {
//       valid_passwords += 1;
//     }
//   }
//   valid_passwords
// }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_io_example() {
    let mut outputs = vec![];
    assert_eq!(
      day02::intcode(&[3, 0, 4, 0, 99], &[1337], &mut outputs),
      vec![1337, 0, 4, 0, 99]
    );
    // Input -> output.
    assert_eq!(outputs, &[1337]);
  }

  #[test]
  fn result_part1() {
    let mut outputs = vec![];
    day02::intcode(&day02::parse_input(&raw_input()), &[1], &mut outputs);
    assert_eq!(outputs[outputs.len() - 1], 15314507);
  }
}
