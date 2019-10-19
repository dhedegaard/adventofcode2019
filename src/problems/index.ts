export const splitLines = (text: string): string[] =>
  text.replace(/\r\n/g, "\n").split("\n")

export const problems = ["day01"]
