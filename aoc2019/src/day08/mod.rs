#[derive(Debug, Clone)]
pub struct Layer {
  width: usize,
  height: usize,
  data: Vec<i32>,
}

impl Layer {
  pub fn count_digit(&self, digit: i32) -> usize {
    self.data.iter().filter(|&e| *e == digit).count()
  }
}

pub type Layers = Vec<Layer>;

pub fn part1(input: &Layers) -> usize {
  let layer = input
    .iter()
    .max_by(|&a, &b| b.count_digit(0).cmp(&a.count_digit(0)))
    .unwrap();
  layer.count_digit(1) * layer.count_digit(2)
}

pub fn raw_input() -> String {
  include_str!("input.txt").to_string()
}

fn parse_input(input: &str, width: usize, height: usize) -> Layers {
  let mut ints = input
    .chars()
    .map(|c| c.to_digit(10).unwrap() as i32)
    .collect::<Vec<_>>();
  let mut layers: Layers = vec![];
  while ints.len() > width * height {
    layers.push(Layer {
      width: width,
      height: height,
      data: ints.iter().take(width * height).cloned().collect(),
    });
    ints = ints.iter().skip(width * height).cloned().collect();
  }
  layers
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example1() {
    assert_eq!(part1(&parse_input("123456789012", 3, 2)), 1);
  }

  #[test]
  fn test_part1() {
    assert_eq!(part1(&parse_input(&raw_input(), 25, 6)), 1690);
  }
}
