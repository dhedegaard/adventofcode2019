#[path = "../intcode/mod.rs"]
mod intcode;

use std::collections::{HashMap, HashSet, VecDeque};

use intcode::*;

#[derive(Debug)]
enum Position {
  Wall,
  Valid,
  OxygenSystem,
}

pub fn raw_input() -> String {
  include_str!("input.txt").to_string()
}

pub fn parse_input(input: &str) -> Intcode {
  Intcode::new(
    &input
      .split(",")
      .map(|e| e.parse::<i64>().unwrap())
      .collect::<Vec<_>>(),
    &[],
  )
}

struct QueueItem {
  program: Intcode,
  depth: usize,
  position: (i64, i64),
}

fn build_grid(program: Intcode) -> (HashSet<(i64, i64)>, usize, (i64, i64)) {
  let mut visited: HashSet<(i64, i64)> = HashSet::new();
  let mut valid_spots: HashSet<(i64, i64)> = HashSet::new();
  let mut queue: VecDeque<QueueItem> = VecDeque::new();
  queue.push_back(QueueItem {
    program,
    depth: 1,
    position: (0, 0),
  });
  let mut result: Option<usize> = None;
  let mut result_pos = (0, 0);
  while !queue.is_empty() {
    let item = queue.pop_front().unwrap();
    for (new_pos, input) in vec![
      ((item.position.0, item.position.1 - 1), 1), // North
      ((item.position.0, item.position.1 + 1), 2), // South
      ((item.position.0 - 1, item.position.1), 3), // West
      ((item.position.0 + 1, item.position.1), 4), // East
    ] {
      if visited.contains(&new_pos) {
        continue;
      }
      visited.insert(new_pos);
      let mut program = Intcode::new(&item.program.insts.clone(), &[input]);
      match program.execute() {
        IntcodeState::Output(e) => match e {
          0 => {
            // Wall, go somewhere else.
            continue;
          }
          1 => {
            valid_spots.insert(new_pos);
            // Valid spot, proceed from here.
          }
          2 => {
            // Oxygen system, jackpot!
            if result.is_none() {
              result = Some(item.depth);
              result_pos = new_pos;
            }
          }
          _ => panic!("weird output: {}", e),
        },
        _ => panic!("WUT!"),
      };
      // Queue the position.
      queue.push_back(QueueItem {
        program: program,
        depth: item.depth + 1,
        position: new_pos,
      });
    }
  }
  (valid_spots, result.unwrap(), result_pos)
}

pub fn part1(program: Intcode) -> usize {
  build_grid(program).1
}

struct PartTwoItem {
  pos: (i64, i64),
  depth: usize,
}

pub fn part2(program: Intcode) -> usize {
  let (valid_spots, _, start_pos) = build_grid(program);
  let mut queue: VecDeque<PartTwoItem> = VecDeque::new();
  queue.push_back(PartTwoItem {
    pos: start_pos,
    depth: 0,
  });
  let mut visited: HashMap<(i64, i64), usize> = HashMap::new();
  visited.insert(start_pos, 0);
  while let Some(item) = queue.pop_front() {
    for (new_pos, input) in vec![
      ((item.pos.0, item.pos.1 - 1), 1), // North
      ((item.pos.0, item.pos.1 + 1), 2), // South
      ((item.pos.0 - 1, item.pos.1), 3), // West
      ((item.pos.0 + 1, item.pos.1), 4), // East
    ] {
      if !valid_spots.contains(&new_pos) || visited.contains_key(&new_pos) {
        continue;
      }
      println!("POS: {:?} - DEPTH: {:?}", new_pos, item.depth + 1);
      queue.push_back(PartTwoItem {
        pos: new_pos,
        depth: item.depth + 1,
      });
      visited.insert(new_pos, item.depth + 1);
    }
  }
  *visited.values().max().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1() {
    assert_eq!(part1(parse_input(&raw_input())), 280);
  }

  #[test]
  fn test_part2() {
    assert_eq!(part2(parse_input(&raw_input())), 400);
  }
}
