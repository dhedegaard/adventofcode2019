const { getDays } = require("./src/mapper-utils")

module.exports = {
  exportPathMap: async (
    defaultPathMap,
    { dev, dir, outDir, distDir, buildId }
  ) => {
    return {
      "/": { page: "/" },
      ...getDays()
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
