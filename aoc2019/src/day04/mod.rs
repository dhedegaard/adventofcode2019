pub fn raw_input() -> String {
  include_str!("input.txt").to_string()
}

fn is_valid_password(input: &str) -> bool {
  if input.len() != 6 {
    return false;
  }
  let mut last_char: Option<char> = None;
  let mut adjacent_are_equal = false;
  for c in input.chars() {
    if !c.is_numeric() {
      return false;
    }
    match last_char {
      Some(e) => {
        if c < e {
          return false;
        } else if !adjacent_are_equal && c == e {
          adjacent_are_equal = true;
        }
      }
      None => {}
    }
    last_char = Some(c);
  }
  adjacent_are_equal
}

fn parse_input(input: &str) -> (String, String) {
  let mut inputs = input.split('-');
  (
    inputs.next().unwrap().to_string(),
    inputs.next().unwrap().to_string(),
  )
}

pub fn part1(input: &str) -> usize {
  let (from, to) = parse_input(input);
  println!("f: {}, t: {}", from, to);
  let mut valid_passwords = 0;
  for val in from.parse::<i32>().unwrap()..to.parse::<i32>().unwrap() {
    if is_valid_password(&val.to_string()) {
      valid_passwords += 1;
    }
  }
  valid_passwords
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_valid_password() {
    assert_eq!(is_valid_password("111111"), true);
    assert_eq!(is_valid_password("223450"), false);
    assert_eq!(is_valid_password("123789"), false);
  }

  #[test]
  fn result_part1() {
    assert_eq!(part1(&raw_input()), 1864);
  }
}
