const path = require("path")
const { getDays } = require("./src/mapper-utils")

exports.createPages = async ({ actions: { createPage } }) => {
  const problemsTemplate = path.resolve(
    __dirname,
    "src",
    "templates",
    "problem.tsx"
  )
  getDays().forEach(day => {
    createPage({
      path: `/${day}`,
      component: problemsTemplate,
      context: {
        day,
      },
    })
  })
}
