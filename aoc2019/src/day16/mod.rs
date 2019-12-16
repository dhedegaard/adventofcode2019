pub fn raw_input() -> String {
  include_str!("input.txt").to_string()
}

pub fn parse_input(input: &str) -> Vec<i32> {
  input
    .chars()
    .map(|c| (c as u8 - b'0') as i32)
    .collect::<Vec<_>>()
}

pub fn part1(input: &[i32]) -> String {
  let mut phase = input.to_vec();
  for _ in 0..100 {
    for i in 1..=phase.len() {
      let mut index = i - 1;
      let mut out = [0, 0];
      while index < phase.len() {
        for o in 0..2 {
          for _ in 0..i {
            if index >= phase.len() {
              break;
            }
            out[o] += phase[index];
            index += 1;
          }
          index += i;
        }
      }
      phase[i - 1] = (out[0] - out[1]).abs() % 10;
    }
  }
  digits_to_string(&phase[..8])
}

pub fn part2(input: &[i32]) -> String {
  let offset: usize = digits_to_string(&input[0..7]).parse::<usize>().unwrap();
  let mut phase: Vec<i32> = input
    .iter()
    .cycle()
    .skip(offset % input.len())
    .take(input.len() * 10000 - offset)
    .map(|c| *c)
    .collect::<Vec<_>>();
  phase.reverse();
  for _ in 0..100 {
    let mut acc = 0;
    for i in 0..phase.len() {
      acc += phase[i];
      phase[i] = acc.abs() % 10;
    }
  }
  digits_to_string(&phase.iter().rev().take(8).map(|e| *e).collect::<Vec<_>>())
}

fn digits_to_string(digits: &[i32]) -> String {
  digits
    .iter()
    .map(|&e| (e as u8 + b'0') as char)
    .collect::<String>()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_digits_to_string() {
    assert_eq!(
      digits_to_string(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
      "1234567890"
    );
  }

  #[test]
  fn test_part1_examle1() {
    assert_eq!(
      part1(&parse_input("80871224585914546619083218645595")),
      "24176176"
    );
  }

  #[test]
  fn test_part1_examle2() {
    assert_eq!(
      part1(&parse_input("19617804207202209144916044189917")),
      "73745418"
    );
  }

  #[test]
  fn test_part1_examle3() {
    assert_eq!(
      part1(&parse_input("69317163492948606335995924319873")),
      "52432133"
    );
  }

  #[test]
  fn test_part1() {
    assert_eq!(part1(&parse_input(&raw_input())), "29956495");
  }

  #[test]
  fn test_part1_example1() {
    assert_eq!(
      part2(&parse_input("03036732577212944063491565474664")),
      "84462026"
    );
  }

  #[test]
  fn test_part1_example2() {
    assert_eq!(
      part2(&parse_input("02935109699940807407585447034323")),
      "78725270"
    );
  }

  #[test]
  fn test_part1_example3() {
    assert_eq!(
      part2(&parse_input("03081770884921959731165446850517")),
      "53553731"
    );
  }

  #[test]
  fn test_part2() {
    assert_eq!(part2(&parse_input(&raw_input())), "73556504");
  }
}
