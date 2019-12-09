#[path = "../intcode/mod.rs"]
mod intcode;

pub fn raw_input() -> String {
  include_str!("input.txt").to_string()
}

pub fn parse_input(input: &str) -> Vec<i64> {
  input
    .split(',')
    .map(|e| e.replace(",", "").replace("+", "").parse::<_>().unwrap())
    .collect()
}

fn run_perm(insts: &[i64], perm: &[i64]) -> i64 {
  let mut amps: Vec<intcode::Intcode> = vec![];

  let mut output: i64 = 0;
  for i in 0..5 {
    let mut input = vec![];
    input.push(perm[i]);
    input.push(output);
    let mut amp = intcode::Intcode::new(&insts, &input);
    output = amp.run()[0];
  }
  output
}

pub fn part1(insts: &[i64]) -> i64 {
  let mut options = vec![0, 1, 2, 3, 4];
  let mut permutations = permutohedron::Heap::new(&mut options);
  let mut highest = 0;
  while let Some(perm) = permutations.next_permutation() {
    let output = run_perm(insts, perm);
    if output > highest {
      highest = output;
    }
  }
  highest
}

fn run_perm_part2(insts: &[i64], perm: &[i64]) -> i64 {
  let mut amps: Vec<intcode::Intcode> = vec![];

  let mut output: i64 = 0;
  for i in 0..5 {
    let mut input = vec![];
    input.push(perm[i]);
    if i == 0 {
      input.push(output);
    }
    amps.push(intcode::Intcode::new(&insts, &input));
  }
  let mut running = 0;
  while amps.iter().any(|e| e.state != intcode::IntcodeState::Halt) {
    let mut amp = &mut amps[running];
    if amp.state == intcode::IntcodeState::Halt {
      running = (running + 1) % perm.len();
      continue;
    }
    match amp.execute() {
      intcode::IntcodeState::Output(out) => {
        if running == perm.len() - 1 {
          output = out;
        }
        amps[(running + 1) % perm.len()].input.push(out)
      }
      _ => {}
    }
    running = (running + 1) % perm.len();
  }
  output
}

pub fn part2(insts: &[i64]) -> i64 {
  let mut options = vec![5, 6, 7, 8, 9];
  let mut permutations = permutohedron::Heap::new(&mut options);
  let mut highest = 0;
  while let Some(perm) = permutations.next_permutation() {
    let output = run_perm_part2(insts, perm);
    if output > highest {
      highest = output;
    }
  }
  highest
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_run_parm_example1() {
    assert_eq!(
      run_perm(
        &[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
        &[4, 3, 2, 1, 0]
      ),
      43210
    );
  }

  #[test]
  fn test_run_part_example2() {
    assert_eq!(
      run_perm(
        &[
          3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
          99, 0, 0
        ],
        &[0, 1, 2, 3, 4]
      ),
      54321
    );
  }

  #[test]
  fn test_run_part_example3() {
    assert_eq!(
      run_perm(
        &[
          3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
          33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
        ],
        &[1, 0, 4, 3, 2]
      ),
      65210
    );
  }

  #[test]
  fn test_part1_examples() {
    assert_eq!(
      part1(&[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0]),
      43210
    );
    assert_eq!(
      part1(&[
        3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
        0, 0
      ]),
      54321
    );
    assert_eq!(
      part1(&[
        3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
        31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
      ]),
      65210
    );
  }

  #[test]
  fn test_part1_result() {
    assert_eq!(part1(&parse_input(&raw_input())), 14902);
  }

  #[test]
  fn test_run_perm_part2_example1() {
    assert_eq!(
      run_perm_part2(
        &[
          3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
          1005, 28, 6, 99, 0, 0, 5
        ],
        &[9, 8, 7, 6, 5]
      ),
      139629729
    );
  }

  #[test]
  fn test_run_perm_part2_example2() {
    assert_eq!(
      run_perm_part2(
        &[
          3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
          -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
          53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
        ],
        &[9, 7, 8, 5, 6]
      ),
      18216
    );
  }

  #[test]
  fn test_part2_example1() {
    assert_eq!(
      part2(&[
        3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
        1005, 28, 6, 99, 0, 0, 5
      ],),
      139629729
    );
  }

  #[test]
  fn test_part2_example2() {
    assert_eq!(
      part2(&[
        3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54, -5,
        54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53,
        1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
      ],),
      18216
    );
  }

  #[test]
  fn test_part2_result() {
    assert_eq!(part2(&parse_input(&raw_input())), 6489132);
  }
}
