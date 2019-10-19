// @ts-check
/**
 * Implement Gatsby's Node APIs in this file.
 *
 * See: https://www.gatsbyjs.org/docs/node-apis/
 */

const path = require("path")
const fs = require("fs")

exports.createPages = async ({ actions: { createPage } }) => {
  for (const problem of fs
    .readdirSync(path.resolve(__dirname, "src/problems"))
    .filter(e => e.startsWith("day") && e.length === 5)) {
    createPage({
      path: `/${problem}`,
      component: path.resolve(__dirname, `./src/pages/Problem.tsx`),
      context: {
        problem,
      },
    })
  }
}
