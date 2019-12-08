#[path = "../intcode/mod.rs"]
mod intcode;

pub fn run_intcode(insts: &[i32]) -> Vec<i32> {
  let mut program = intcode::Intcode::new(insts, &[]);
  println!("before: {:?}", program.insts);
  program.run();
  println!("after: {:?}", program.insts);
  program.insts
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

fn get_registry(insts: &mut Vec<i32>, operation: i32, pc: usize, param: usize, addr: bool) -> i32 {
  let val = insts[pc + param];
  let mask = (operation as u32) / (10_u32.pow(param as u32 + 1)) % 10;
  assert!(!addr || mask == 0);
  if !addr && mask == 0 {
    insts[val as usize]
  } else {
    val
  }
}

pub fn part1(input: &[i32]) -> i32 {
  let mut insts = input.to_vec();
  insts[1] = 12;
  insts[2] = 2;
  run_intcode(&insts)[0]
}

pub fn part2(input: &[i32]) -> i32 {
  for noun in 0..100 {
    for verb in 0..100 {
      let mut insts = input.to_vec();
      insts[1] = noun;
      insts[2] = verb;
      if run_intcode(&insts)[0] == 19690720 {
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
    assert_eq!(run_intcode(&[1, 0, 0, 0, 99]), &[2, 0, 0, 0, 99]);
    assert_eq!(run_intcode(&[2, 3, 0, 3, 99]), &[2, 3, 0, 6, 99]);
    assert_eq!(run_intcode(&[2, 4, 4, 5, 99, 0]), &[2, 4, 4, 5, 99, 9801]);
    assert_eq!(
      run_intcode(&[1, 1, 1, 4, 99, 5, 6, 0, 99]),
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
