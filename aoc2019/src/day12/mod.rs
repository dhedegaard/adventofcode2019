use num_integer::lcm;
use std::cmp::Ordering;
use std::iter;

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

  fn energy(&self) -> i64 {
    let pot = self.pos.0.abs() + self.pos.1.abs() + self.pos.2.abs();
    let kin = self.vel.0.abs() + self.vel.1.abs() + self.vel.2.abs();
    pot * kin
  }
}

fn calc_delta(a: i64, b: i64) -> i64 {
  match a.cmp(&b) {
    Ordering::Less => 1,
    Ordering::Equal => 0,
    Ordering::Greater => -1,
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

fn part1(input: &mut [Moon]) -> i64 {
  for _ in 0..1000 {
    for i in 0..input.len() {
      for j in (i + 1)..input.len() {
        let dx = calc_delta(input[i].pos.0, input[j].pos.0);
        input[i].vel.0 += dx;
        input[j].vel.0 -= dx;
        let dy = calc_delta(input[i].pos.1, input[j].pos.1);
        input[i].vel.1 += dy;
        input[j].vel.1 -= dy;
        let dz = calc_delta(input[i].pos.2, input[j].pos.2);
        input[i].vel.2 += dz;
        input[j].vel.2 -= dz;
      }
      input[i].pos.0 += input[i].vel.0;
      input[i].pos.1 += input[i].vel.1;
      input[i].pos.2 += input[i].vel.2;
    }
  }
  input.iter().map(|m| m.energy()).sum()
}

fn run_axis(moon: &[i64]) -> i64 {
  let mut moons: Vec<(i64, i64)> = moon.iter().map(|e| (*e, 0 as i64)).collect::<Vec<_>>();
  let init = moons.clone();
  let steps = iter::repeat(())
    .take_while(|_| {
      for i in 0..moon.len() {
        for j in (i + 1)..moon.len() {
          let d = calc_delta(moons[i].0, moons[j].0);
          moons[i].1 += d;
          moons[j].1 -= d;
        }
        moons[i].0 += moons[i].1;
      }
      moons != init
    })
    .count();
  steps as i64 + 1
}

fn part2(input: &mut [Moon]) -> i64 {
  let x = run_axis(&input.iter().map(|e| e.pos.0).collect::<Vec<_>>());
  let y = run_axis(&input.iter().map(|e| e.pos.1).collect::<Vec<_>>());
  let z = run_axis(&input.iter().map(|e| e.pos.2).collect::<Vec<_>>());
  lcm(x, lcm(y, z))
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

  #[test]
  fn test_result1() {
    assert_eq!(part1(&mut parse_input(&raw_input())), 10635);
  }

  #[test]
  fn test_result2() {
    assert_eq!(part2(&mut parse_input(&raw_input())), 583523031727256);
  }
}
