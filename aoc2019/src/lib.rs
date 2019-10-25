extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

mod day01;

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
