import React from "react"

import Layout from "../components/Layout"
import { Typography } from "@material-ui/core"

const IndexPage = () => (
  <Layout>
    <Typography variant="h6">Advent of Code 2019</Typography>
    <Typography>
      This years Advent of Code, implemented in the browser, using typescript.
    </Typography>
  </Layout>
)

export default IndexPage
