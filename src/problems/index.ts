export const splitLines = (text: string): string[] =>
  text
    .replace(/\r\n/g, "\n")
    .split("\n")
    .filter(e => e != null)
    .map(e => e.trim())

export const problems = ["day01"]

export type Solution = (input: string) => Promise<string>
