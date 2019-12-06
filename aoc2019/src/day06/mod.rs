use std::collections::HashMap;

pub fn raw_input() -> String {
  include_str!("input.txt").to_string()
}

fn parse_input(input: &str) -> HashMap<String, String> {
  let mut result = HashMap::with_capacity(input.chars().filter(|e| e.is_whitespace()).count());
  for line in input.split_whitespace() {
    let mut elems = line.split(')');
    let parent = elems.next().unwrap();
    let elem = elems.next().unwrap();
    result.insert(elem.to_string(), parent.to_string());
  }
  result
}

fn count_orbits(elem: &str, orbits: &HashMap<String, String>) -> usize {
  let mut count = 0;
  let mut cur = elem;
  while orbits.contains_key(cur) {
    cur = orbits.get(cur).unwrap();
    count += 1;
  }
  count
}

pub fn part1(input: &str) -> usize {
  let orbits = parse_input(input);
  orbits.keys().map(|key| count_orbits(key, &orbits)).sum()
}

fn parse_input2(input: &str) -> HashMap<String, Vec<String>> {
  let mut result: HashMap<String, Vec<String>> =
    HashMap::with_capacity(input.chars().filter(|e| e.is_whitespace()).count());
  for line in input.split_whitespace() {
    let mut elems = line.split(')');
    let parent = elems.next().unwrap();
    let elem = elems.next().unwrap();
    if let Some(e) = result.get_mut(parent) {
      e.push(elem.to_string())
    } else {
      result.insert(parent.to_string(), vec![elem.to_string()]);
    }
    if let Some(e) = result.get_mut(elem) {
      e.push(parent.to_string())
    } else {
      result.insert(elem.to_string(), vec![parent.to_string()]);
    }
  }
  result
}

fn count_transfers(from: &str, to: &str, orbits: &HashMap<String, Vec<String>>) -> usize {
  let mut queue = vec![from.to_string()];
  // planet -> depth.
  let mut seen: HashMap<String, usize> = HashMap::new();
  seen.insert(from.to_string(), 0);
  while !queue.is_empty() {
    let elem = queue.remove(0);
    if elem == to {
      return *seen.get(&elem).unwrap() - 2;
    }
    for edge in orbits.get(&elem).unwrap() {
      if !seen.contains_key(edge) {
        seen.insert(edge.to_string(), seen.get(&elem).unwrap() + 1);
        queue.push(edge.to_string());
      }
    }
  }
  unreachable!();
}

pub fn part2(input: &str) -> usize {
  let orbits = parse_input2(input);
  count_transfers("YOU", "SAN", &orbits)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example1() {
    let orbits = parse_input(&include_str!("test.txt"));

    assert_eq!(count_orbits("COM", &orbits), 0);
    assert_eq!(count_orbits("D", &orbits), 3);
    assert_eq!(count_orbits("L", &orbits), 7);
  }

  #[test]
  fn test_part1() {
    assert_eq!(part1(&raw_input()), 147807);
  }

  #[test]
  fn test_example2() {
    let orbits = parse_input2(&include_str!("test2.txt"));

    assert_eq!(count_transfers("YOU", "SAN", &orbits), 4);
  }

  #[test]
  fn test_part2() {
    assert_eq!(part2(&raw_input()), 229);
  }
}
