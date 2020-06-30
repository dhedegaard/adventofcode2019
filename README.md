# Advent of Code 2019

[![Build Status](https://dev.azure.com/dhedegaard/adventofcode2019/_apis/build/status/dhedegaard.adventofcode2019?branchName=master)](https://dev.azure.com/dhedegaard/adventofcode2019/_build/latest?definitionId=10&branchName=master)
![Deploy](https://github.com/dhedegaard/adventofcode2019/workflows/Deploy/badge.svg)
![Build gatsby](https://github.com/dhedegaard/adventofcode2019/workflows/Build%20gatsby/badge.svg)
![Run rust tests](https://github.com/dhedegaard/adventofcode2019/workflows/Run%20rust%20tests/badge.svg)

Solutions in rust, build to web assembly and running in your favorite browser.

## How to get it to run

1. Install rust (using rustup, wasm-pack seem to require it) and node.js.
1. Install wasm-pack following the instructions here:
   <https://rustwasm.github.io/wasm-pack/installer/>
1. Run `$ yarn watch` to develop on the gatsby-part and build using `$ yarn test` and `$ yarn wasm` for the rust part.
