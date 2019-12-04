extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

mod day01;
mod day02;
mod day03;
mod day04;

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
