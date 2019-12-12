pub fn raw_input() -> String {
  include_str!("input.txt").to_string()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Moon {
  pos: (i64, i64, i64),
  vel: (i64, i64, i64),
}

impl Moon {
  fn new(x: i64, y: i64, z: i64) -> Self {
    Moon {
      pos: (x, y, z),
      vel: (0, 0, 0),
    }
  }
}

pub fn parse_input(input: &str) -> Vec<Moon> {
  let regex = regex::Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
  input
    .split('\n')
    .map(|line| {
      let cap = regex.captures(line).unwrap();
      Moon::new(
        cap.get(1).unwrap().as_str().parse().unwrap(),
        cap.get(2).unwrap().as_str().parse().unwrap(),
        cap.get(3).unwrap().as_str().parse().unwrap(),
      )
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse() {
    assert_eq!(
      parse_input(include_str!("test.txt")),
      vec![
        Moon::new(-1, 0, 2),
        Moon::new(2, -10, -7),
        Moon::new(4, -8, 8),
        Moon::new(3, 5, -1)
      ]
    );
  }
}
