import React from "react"
import Layout from "../components/Layout"
import { Typography } from "@material-ui/core"
import { GatsbyPageProps } from "../@types/aoc2019"

const NotFoundPage: React.FC<GatsbyPageProps> = props => (
  <Layout path={props.path}>
    <Typography component="h1" variant="h6">
      Page not found
    </Typography>
    <Typography>The page you're looking for was not found.</Typography>
  </Layout>
)

export default NotFoundPage
