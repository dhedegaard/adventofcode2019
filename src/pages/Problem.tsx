import React from "react"
import Layout from "../components/layout"
import SEO from "../components/seo"

type Props = {
  pathContext: {
    problem: string
  }
}

const Problem: React.FC<Props> = props => {
  console.log("PROPS:", props)
  return (
    <Layout>
      <SEO title={props.pathContext.problem}></SEO>
      <h1>{props.pathContext.problem}</h1>
    </Layout>
  )
}

export default Problem
