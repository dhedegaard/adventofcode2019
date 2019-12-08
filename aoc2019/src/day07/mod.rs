#[path = "../intcode/mod.rs"]
mod intcode;

pub fn raw_input() -> String {
  include_str!("input.txt").to_string()
}

pub fn parse_input(input: &str) -> Vec<i32> {
  input
    .split(',')
    .map(|e| e.replace(",", "").replace("+", "").parse::<_>().unwrap())
    .collect()
}

fn run_perm(insts: &[i32], perm: &[i32]) -> i32 {
  println!("perm: {:?}", perm);
  let mut amps: Vec<intcode::Intcode> = vec![];

  let mut output: i32 = 0;
  for i in 0..5 {
    let mut input = vec![];
    input.push(perm[i]);
    input.push(output);
    println!("{}: input: {:?}", i, input);
    let mut amp = intcode::Intcode::new(&insts, &input);
    output = amp.run()[0];
    println!("{}: output: {:?}", i, output);
  }
  println!("output: {}", output);
  output
}

pub fn part1(insts: &[i32]) -> i32 {
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
}
