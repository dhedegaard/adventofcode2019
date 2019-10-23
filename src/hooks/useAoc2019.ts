import React from "react"

/** Async loads the aoc2019 wasm, outputs a given problem. */
const useAoc2019 = (
  problem: string
): null | {
  raw_input?: () => string
  part1?: (arg: string) => string
  part2?: (arg: string) => string
} => {
  const [module, setModule] = React.useState<null | {
    [key: string]: (...args: any[]) => any
  }>(null)

  React.useEffect(() => {
    import("../aoc2019/aoc2019").then(module => setModule(module as any))
  })

  return module == null
    ? module
    : Object.fromEntries(
        Object.entries(module)
          .filter(([name]) => name.startsWith(`${problem}__`))
          .map(([name, func]) => [name.slice(7), func])
      )
}

export default useAoc2019
