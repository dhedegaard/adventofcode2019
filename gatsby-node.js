const path = require("path")

exports.createPages = async ({ actions: { createPage }, graphql }) => {
  const { data } = await graphql(`
    {
      allFile(
        filter: {
          sourceInstanceName: { eq: "problems" }
          name: { eq: "mod" }
          extension: {}
          dir: { glob: "**/day*" }
        }
        sort: { fields: dir, order: ASC }
      ) {
        edges {
          node {
            dir
          }
        }
      }
    }
  `)
  const problems = data.allFile.edges.map(e => {
    const { dir } = e.node
    const parts = dir.split("/")
    return parts[parts.length - 1]
  })
  const problemsTemplate = path.resolve("src", "templates", "problem.tsx")
  problems.forEach(day => {
    createPage({
      path: `/${day}/`,
      component: problemsTemplate,
      context: {
        day,
      },
    })
  })
}
