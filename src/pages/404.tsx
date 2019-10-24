import React from "react"
import Layout from "../components/Layout"
import { Typography } from "@material-ui/core"

const NotFoundPage = () => (
  <Layout>
    <Typography component="h1" variant="h6">
      Page not found
    </Typography>
    <Typography>The page you're looking for was not found.</Typography>
  </Layout>
)

export default NotFoundPage
