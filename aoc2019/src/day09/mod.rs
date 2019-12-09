#[path = "../intcode/mod.rs"]
mod intcode;

fn run_intcode(insts: &[i64]) -> Vec<i64> {
  intcode::Intcode::new(insts, &[]).run()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example1() {
    assert_eq!(
      run_intcode(&[109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]),
      vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
    );
  }

  #[test]
  fn test_example2() {
    assert_eq!(
      run_intcode(&[1102, 34915192, 34915192, 7, 4, 7, 99, 0]),
      vec![1219070632396864],
    )
  }

  #[test]
  fn test_example3() {
    assert_eq!(
      run_intcode(&[104, 1125899906842624, 99]),
      vec![1125899906842624]
    );
  }
}
