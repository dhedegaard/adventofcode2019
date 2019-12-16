use std::collections::HashMap;

pub fn raw_input() -> String {
  include_str!("input.txt").to_string()
}

#[derive(Hash, Eq, PartialEq)]
pub struct Amount {
  amount: i64,
  chemical: String,
}

#[derive(Hash, Eq, PartialEq)]
pub struct Reaction {
  inputs: Vec<Amount>,
  output: Amount,
}

fn ore_cost(fuel: i64, reactions: &HashMap<String, Reaction>) -> i64 {
  let mut counts = HashMap::new();

  for (_, reaction) in reactions {
    for ingredient in &reaction.inputs {
      *counts.entry(ingredient.chemical.to_string()).or_insert(0) += 1;
    }
  }

  let mut ready = vec![Amount {
    amount: fuel,
    chemical: "FUEL".to_string(),
  }];
  let mut not_ready: HashMap<String, _> = HashMap::new();

  while let Some(Amount { amount, chemical }) = ready.pop() {
    let reaction = match reactions.get(&chemical) {
      Some(x) => x,
      None => return amount,
    };
    let times = (amount - 1) / reaction.output.amount + 1;

    for input in &reaction.inputs {
      let amount = input.amount * times + not_ready.get(&input.chemical).copied().unwrap_or(0);
      let count = counts.get_mut(&input.chemical).unwrap();
      *count -= 1;
      if *count == 0 {
        ready.push(Amount {
          amount,
          chemical: input.chemical.to_string(),
        });
      } else {
        not_ready.insert(input.chemical.to_string(), amount);
      }
    }
  }

  unreachable!()
}

fn part1(reactions: &HashMap<String, Reaction>) -> i64 {
  ore_cost(1, reactions)
}

fn optimal_ore_cost(chemical: String, reactions: &HashMap<String, Reaction>) -> f64 {
  let reaction = match reactions.get(&chemical) {
    None => return 1.0,
    Some(x) => x,
  };
  let x = 1.0 / reaction.output.amount as f64;
  reaction
    .inputs
    .iter()
    .map(|Amount { amount, chemical }| {
      *amount as f64 / reaction.output.amount as f64
        * optimal_ore_cost(chemical.to_string(), reactions)
    })
    .sum()
}

fn part2(reactions: &HashMap<String, Reaction>) -> i64 {
  const TRILLION: i64 = 1_000_000_000_000;
  let upper_bound =
    (TRILLION as f64 / optimal_ore_cost("FUEL".to_string(), reactions)).floor() as i64;
  (0..=upper_bound)
    .rev()
    .find(|&n| ore_cost(n, reactions) <= TRILLION)
    .unwrap()
}

pub fn parse_input(input: &str) -> HashMap<String, Reaction> {
  let mut result: HashMap<String, Reaction> = HashMap::new();
  for line in input.lines() {
    let parts = line.split(" => ").collect::<Vec<_>>();
    let input = parts[0];
    let input_parts = input.split(", ").collect::<Vec<_>>();
    let output = parts[1];
    let output_parts = output.split(" ").collect::<Vec<_>>();
    let mut inputs: Vec<Amount> = vec![];
    for part in input_parts {
      let p = part.split(" ").collect::<Vec<_>>();
      inputs.push(Amount {
        amount: p[0].parse::<i64>().unwrap(),
        chemical: p[1].to_string(),
      });
    }
    result.insert(
      output_parts[1].to_string(),
      Reaction {
        inputs,
        output: Amount {
          amount: output_parts[0].parse::<i64>().unwrap(),
          chemical: output_parts[1].to_string(),
        },
      },
    );
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1_example() {
    assert_eq!(part1(&parse_input(include_str!("test1.txt"))), 165);
  }

  #[test]
  fn test_part1() {
    assert_eq!(part1(&parse_input(&raw_input())), 220019);
  }

  #[test]
  fn test_part2() {
    assert_eq!(part2(&parse_input(&raw_input())), 5650230);
  }
}
