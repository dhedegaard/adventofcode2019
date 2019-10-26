import React from "react"

import Layout from "../components/Layout"
import { Typography, Link } from "@material-ui/core"
import { GatsbyPageProps } from "../@types/aoc2019"

const IndexPage: React.FC<GatsbyPageProps> = props => (
  <Layout path={props.path}>
    <Typography component="h1" variant="h6">
      Advent of Code 2019
    </Typography>

    <Typography paragraph>
      This years Advent of Code, implemented in the browser, using typescript.
    </Typography>

    <Typography paragraph>
      The source code for this site can be found here:{" "}
      <Link
        href="https://github.com/dhedegaard/adventofcode2019"
        target="_blank"
        rel="noopener nofollow"
      >
        https://github.com/dhedegaard/adventofcode2019
      </Link>
    </Typography>

    <Typography paragraph>
      The basic idea is:
      <ul>
        <li>Implement the solution in Rust with unit tests like last year.</li>
        <li>Build the rust code into web assembly.</li>
        <li>Run the solutions through the browser, using this site.</li>
      </ul>
    </Typography>
  </Layout>
)

export default IndexPage
