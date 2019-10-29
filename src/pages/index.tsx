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

    <Typography paragraph>The basic idea is:</Typography>
    <ul>
      <li>
        <Typography>
          Implement the solution in Rust with unit tests like last year.
        </Typography>
      </li>
      <li>
        <Typography>Build the rust code into web assembly.</Typography>
      </li>
      <li>
        <Typography>
          Run the solutions through the browser, using this site.
        </Typography>
      </li>
    </ul>
  </Layout>
)

export default IndexPage
