import { useStaticQuery, graphql } from "gatsby"

const problemsQuery = graphql`
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
`

/**
 * Gets the problems with mod.rs-files currently in the rust code.
 *
 * Ordered asc.
 */
export default (): string[] => {
  const data = useStaticQuery(problemsQuery)
  return data.allFile.edges.map((e: any) => {
    const { dir } = e.node as { dir: string }
    const parts = dir.split("/")
    return parts[parts.length - 1]
  })
}
