import React from "react"
import PropTypes from "prop-types"
import { Helmet } from "react-helmet"
import {
  CssBaseline,
  Grid,
  AppBar,
  Toolbar,
  Typography,
  Paper,
} from "@material-ui/core"
import NavMenu from "./NavMenu"
import styled from "styled-components"

const ContentPaper = styled(Paper)`
  padding: 20px;
  margin-top: 20px;
`

const ContentGrid = styled(Grid)`
  flex-grow: 1;
`

const OuterGrid = styled(Grid)`
  width: 100% !important;
`

/**
 * We load an ES6 module here, since that is how you load WASM without a bundler:
 * <https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html>
 */
const moduleScript = `
import init, * as aoc2019 from '/aoc2019/aoc2019.js';
init().then(() => {
  window.aoc2019 = aoc2019;
  window.postMessage('aoc2019');
});
`.replace(/(\n|\s\s)/g, "")

const ModuleScript = React.memo(
  () => (
    <script type="module" async>
      {moduleScript}
    </script>
  ),
  () => true
)

type Props = {
  path: string
  children: React.ReactNode
}

const Layout: React.FC<Props> = props => {
  return (
    <>
      <ModuleScript />
      <Helmet>
        <meta
          name="viewport"
          content="minimum-scale=1, initial-scale=1, width=device-width, shrink-to-fit=no"
        />
        <title>Advent of Code 2019</title>
        <link rel="icon" type="image/png" href="/favicon.png" />
      </Helmet>
      <CssBaseline />
      <header>
        <AppBar position="static">
          <Toolbar>
            <Typography variant="h6">Advent of Code 2019</Typography>
          </Toolbar>
        </AppBar>
      </header>
      <main>
        <OuterGrid container spacing={3}>
          <Grid item md={2} sm={4} xs={6}>
            <Paper square>
              <NavMenu path={props.path} />
            </Paper>
          </Grid>
          <ContentGrid item md={10} sm={8} xs={6}>
            <ContentPaper>{props.children}</ContentPaper>
          </ContentGrid>
        </OuterGrid>
      </main>
    </>
  )
}

Layout.propTypes = {
  children: PropTypes.node.isRequired,
  path: PropTypes.string.isRequired,
}

export default Layout
