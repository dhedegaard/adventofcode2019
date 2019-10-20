import { splitLines, Solution } from ".."
import sum from "lodash/sum"

const day01: Solution = async (input: string) =>
  sum(
    splitLines(input)
      .map(line => parseInt(line, 10))
      .filter(e => !isNaN(e))
  ).toString()

export default day01
