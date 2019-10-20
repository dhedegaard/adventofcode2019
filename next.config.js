const fs = require("fs")
const path = require("path")

module.exports = {
  exportPathMap: async (
    defaultPathMap,
    { dev, dir, outDir, distDir, buildId }
  ) => {
    const problems = fs
      .readdirSync(path.resolve(__dirname, "src", "problems"))
      .filter(e => e.startsWith("day") && e.length === 5)
    return {
      "/": { page: "/" },
      ...problems
        .map(problem => [
          `/${problem}`,
          { page: "/[problem]", query: { problem } },
        ])
        .reduce((obj, [key, value]) => {
          obj[key] = value
          return obj
        }, {}),
    }
  },
}
