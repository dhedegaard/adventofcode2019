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
  // #[test]
  // fn test_is_valid_password() {
  //   assert_eq!(is_valid_password("111111"), true);
  //   assert_eq!(is_valid_password("223450"), false);
  //   assert_eq!(is_valid_password("123789"), false);
  // }

  // #[test]
  // fn result_part1() {
  //   assert_eq!(part1(&raw_input()), 1864);
  // }

  // #[test]
  // fn test_is_valid_password2() {
  //   assert_eq!(is_valid_password2("111111"), false);
  //   assert_eq!(is_valid_password2("112233"), true);
  //   assert_eq!(is_valid_password2("123444"), false);
  //   assert_eq!(is_valid_password2("111122"), true);
  //   assert_eq!(is_valid_password2("123445"), true);
  // }

  // #[test]
  // fn result_part2() {
  //   assert_eq!(part2(&raw_input()), 1258);
  // }
}
