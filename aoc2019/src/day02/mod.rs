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

pub fn intcode(insts_slice: &[i32], input_slice: &[i32], outputs: &mut Vec<i32>) -> Vec<i32> {
  let mut insts = insts_slice.to_vec();
  let mut inputs = input_slice.to_vec();
  let mut pc = 0;
  loop {
    let operation = insts[pc];
    let opcode = operation % 100;
    match opcode {
      1 => {
        // Addition
        let a = get_registry(&mut insts, operation, pc, 1, false);
        let b = get_registry(&mut insts, operation, pc, 2, false);
        let res = get_registry(&mut insts, operation, pc, 3, true) as usize;
        insts[res] = a + b;
        pc += 4;
      }
      2 => {
        // Multiplication
        let a = get_registry(&mut insts, operation, pc, 1, false);
        let b = get_registry(&mut insts, operation, pc, 2, false);
        let res = get_registry(&mut insts, operation, pc, 3, true) as usize;
        insts[res] = a * b;
        pc += 4;
      }
      3 => {
        // Input
        let res = get_registry(&mut insts, operation, pc, 1, true) as usize;
        insts[res] = inputs.remove(0);
        pc += 2;
      }
      4 => {
        // Output
        let res = get_registry(&mut insts, operation, pc, 1, false);
        outputs.push(res);
        pc += 2;
      }
      6 => {
        // jump-if-false
        let res = get_registry(&mut insts, operation, pc, 1, false);
        if res == 0 {
          pc = get_registry(&mut insts, operation, pc, 2, false) as usize;
        } else {
          pc += 3;
        }
      }
      7 => {
        // less-than
        let a = get_registry(&mut insts, operation, pc, 1, false);
        let b = get_registry(&mut insts, operation, pc, 2, false);
        let res = get_registry(&mut insts, operation, pc, 3, true) as usize;
        insts[res] = if a < b { 1 } else { 0 };
        pc += 4;
      }
      8 => {
        // equals
        let a = get_registry(&mut insts, operation, pc, 1, false);
        let b = get_registry(&mut insts, operation, pc, 2, false);
        let res = get_registry(&mut insts, operation, pc, 3, true) as usize;
        insts[res] = if a == b { 1 } else { 0 };
        pc += 4;
      }
      // Halt
      99 => return insts,
      _ => panic!("Unknown opcode: {}", opcode),
    }
  }
}

pub fn part1(input: &[i32]) -> i32 {
  let mut insts = input.to_vec();
  insts[1] = 12;
  insts[2] = 2;
  intcode(&insts, &[], &mut vec![])[0]
}

pub fn part2(input: &[i32]) -> i32 {
  for noun in 0..100 {
    for verb in 0..100 {
      let mut insts = input.to_vec();
      insts[1] = noun;
      insts[2] = verb;
      if intcode(&insts, &[], &mut vec![])[0] == 19690720 {
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
    assert_eq!(
      intcode(&[1, 0, 0, 0, 99], &[], &mut vec![]),
      &[2, 0, 0, 0, 99]
    );
    assert_eq!(
      intcode(&[2, 3, 0, 3, 99], &[], &mut vec![]),
      &[2, 3, 0, 6, 99]
    );
    assert_eq!(
      intcode(&[2, 4, 4, 5, 99, 0], &[], &mut vec![]),
      &[2, 4, 4, 5, 99, 9801]
    );
    assert_eq!(
      intcode(&[1, 1, 1, 4, 99, 5, 6, 0, 99], &[], &mut vec![]),
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
