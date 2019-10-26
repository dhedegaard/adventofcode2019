import { useStaticQuery, graphql } from "gatsby"

/** A query for the site metadata from the gatsby config. */
export default (): {
  siteUrl: string
  description: string
  title: string
  author: string
  githubUrl: string
} => {
  const data = useStaticQuery(graphql`
    {
      site {
        siteMetadata {
          siteUrl
          description
          title
          author
          githubUrl
        }
      }
    }
  `)
  return data.site.siteMetadata
}
