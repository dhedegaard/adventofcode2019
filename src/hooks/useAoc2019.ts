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
    const w = window as any
    // Look for aoc2019 on the window object.
    if (w.aoc2019 != null) {
      setModule(w.aoc2019)
    }
    // If not found, wait for a postMessage with aoc2019.
    const listener = (event: MessageEvent) => {
      if (event.data === "aoc2019") {
        setModule(w.aoc2019)
      }
    }
    window.addEventListener("message", listener, false)
    return () => window.removeEventListener("message", listener, false)
  }, [])

  return module == null
    ? module
    : Object.fromEntries(
        Object.entries(module)
          .filter(([name]) => name.startsWith(`${problem}_`))
          .map(([name, func]) => [name.slice(6), func])
      )
}

export default useAoc2019
