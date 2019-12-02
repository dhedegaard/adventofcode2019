pub fn raw_input() -> String {
  include_str!("input.txt").to_string()
}

pub fn parse_input(input: &str) -> Vec<i32> {
  input
    .split(',')
    .map(|e| e.replace(",", "").replace("+", "").parse::<_>().unwrap())
    .collect()
}

pub fn intcode(input: &[i32]) -> Vec<i32> {
  let mut insts = input.to_vec();
  let mut pc = 0;
  loop {
    match insts[pc] {
      1 => {
        // Addition
        let res = insts[pc + 3] as usize;
        insts[res] = insts[insts[pc + 1] as usize] + insts[insts[pc + 2] as usize];
        pc += 4;
      }
      2 => {
        // Multiplication
        let res = insts[pc + 3] as usize;
        insts[res] = insts[insts[pc + 1] as usize] * insts[insts[pc + 2] as usize];
        pc += 4;
      }
      // Halt
      99 => return insts,
      _ => panic!("Unknown instruction: {}", insts[pc]),
    }
  }
}

pub fn part1(input: &[i32]) -> i32 {
  let mut insts = input.to_vec();
  insts[1] = 12;
  insts[2] = 2;
  intcode(&insts)[0]
}

pub fn part2(input: &[i32]) -> i32 {
  for noun in 0..100 {
    for verb in 0..100 {
      let mut insts = input.to_vec();
      insts[1] = noun;
      insts[2] = verb;
      if intcode(&insts)[0] == 19690720 {
        return 100 * noun + verb;
      }
    }
  }
  unreachable!();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn examples_intcode() {
    assert_eq!(intcode(&[1, 0, 0, 0, 99]), &[2, 0, 0, 0, 99]);
    assert_eq!(intcode(&[2, 3, 0, 3, 99]), &[2, 3, 0, 6, 99]);
    assert_eq!(intcode(&[2, 4, 4, 5, 99, 0]), &[2, 4, 4, 5, 99, 9801]);
    assert_eq!(
      intcode(&[1, 1, 1, 4, 99, 5, 6, 0, 99]),
      &[30, 1, 1, 4, 2, 5, 6, 0, 99]
    );
  }

  #[test]
  fn result_part1() {
    assert_eq!(part1(&parse_input(&raw_input())), 3267740);
  }

  #[test]
  fn result_part2() {
    assert_eq!(part2(&parse_input(&raw_input())), 7870);
  }
}
