extern crate num_integer;
extern crate permutohedron;
extern crate regex;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day14;
mod intcode;

#[wasm_bindgen]
pub fn day01_input() -> String {
  day01::raw_input()
}
#[wasm_bindgen]
pub fn day01_part1(input: &str) -> String {
  day01::part1(&day01::parse_input(input)).to_string()
}
#[wasm_bindgen]
pub fn day01_part2(input: &str) -> String {
  day01::part2(&day01::parse_input(input)).to_string()
}

#[wasm_bindgen]
pub fn day02_input() -> String {
  day02::raw_input()
}
#[wasm_bindgen]
pub fn day02_part1(input: &str) -> String {
  day02::part1(&day02::parse_input(input)).to_string()
}
#[wasm_bindgen]
pub fn day02_part2(input: &str) -> String {
  day02::part2(&day02::parse_input(input)).to_string()
}

#[wasm_bindgen]
pub fn day03_input() -> String {
  day03::raw_input()
}
#[wasm_bindgen]
pub fn day03_part1(input: &str) -> String {
  day03::part1(&day03::parse_input(input)).to_string()
}
#[wasm_bindgen]
pub fn day03_part2(input: &str) -> String {
  day03::part2(&day03::parse_input(input)).to_string()
}

#[wasm_bindgen]
pub fn day04_input() -> String {
  day04::raw_input()
}
#[wasm_bindgen]
pub fn day04_part1(input: &str) -> String {
  day04::part1(input).to_string()
}
#[wasm_bindgen]
pub fn day04_part2(input: &str) -> String {
  day04::part2(input).to_string()
}

#[wasm_bindgen]
pub fn day05_input() -> String {
  day05::raw_input()
}
#[wasm_bindgen]
pub fn day05_part1(input: &str) -> String {
  day05::part1(input).to_string()
}
#[wasm_bindgen]
pub fn day05_part2(input: &str) -> String {
  day05::part2(input).to_string()
}

#[wasm_bindgen]
pub fn day06_input() -> String {
  day06::raw_input()
}
#[wasm_bindgen]
pub fn day06_part1(input: &str) -> String {
  day06::part1(input).to_string()
}
#[wasm_bindgen]
pub fn day06_part2(input: &str) -> String {
  day06::part2(input).to_string()
}

#[wasm_bindgen]
pub fn day07_input() -> String {
  day07::raw_input()
}
#[wasm_bindgen]
pub fn day07_part1(input: &str) -> String {
  day07::part1(&day07::parse_input(input)).to_string()
}
#[wasm_bindgen]
pub fn day07_part2(input: &str) -> String {
  day07::part2(&day07::parse_input(input)).to_string()
}

#[wasm_bindgen]
pub fn day08_input() -> String {
  day08::raw_input()
}
#[wasm_bindgen]
pub fn day08_part1(input: &str) -> String {
  day08::part1(&day08::parse_input(input, 25, 6)).to_string()
}
#[wasm_bindgen]
pub fn day08_part2(input: &str) -> String {
  day08::part2(&day08::parse_input(input, 25, 6)).to_string()
}

#[wasm_bindgen]
pub fn day09_input() -> String {
  day09::raw_input()
}
#[wasm_bindgen]
pub fn day09_part1(input: &str) -> String {
  day09::part1(&day09::parse_input(input)).to_string()
}
#[wasm_bindgen]
pub fn day09_part2(input: &str) -> String {
  day09::part2(&day09::parse_input(input)).to_string()
}

#[wasm_bindgen]
pub fn day10_input() -> String {
  day10::raw_input()
}
#[wasm_bindgen]
pub fn day10_part1(input: &str) -> String {
  day10::part1(&day10::parse_input(input)).to_string()
}
#[wasm_bindgen]
pub fn day10_part2(input: &str) -> String {
  day10::part2(&mut day10::parse_input(input)).to_string()
}

#[wasm_bindgen]
pub fn day11_input() -> String {
  day11::raw_input()
}
#[wasm_bindgen]
pub fn day11_part1(input: &str) -> String {
  day11::part1(&day11::parse_input(input)).to_string()
}
#[wasm_bindgen]
pub fn day11_part2(input: &str) -> String {
  day11::part2(&day11::parse_input(input)).to_string()
}

#[wasm_bindgen]
pub fn day12_input() -> String {
  day12::raw_input()
}
#[wasm_bindgen]
pub fn day12_part1(input: &str) -> String {
  day12::part1(&mut day12::parse_input(input)).to_string()
}
#[wasm_bindgen]
pub fn day12_part2(input: &str) -> String {
  day12::part2(&mut day12::parse_input(input)).to_string()
}
