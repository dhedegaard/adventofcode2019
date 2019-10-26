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
  createMuiTheme,
} from "@material-ui/core"
import NavMenu from "./NavMenu"
import styled, { createGlobalStyle } from "styled-components"
import { ThemeProvider } from "@material-ui/styles"
import { blueGrey } from "@material-ui/core/colors"
import OpenGraph from "./OpenGraph"
import useSiteMetadata from "../hooks/useSiteMetadata"

const ContentPaper = styled(Paper)`
  padding: 20px;
  margin-top: 20px;
`

const ContentGrid = styled(Grid)`
  flex-grow: 1;
`

const OuterGrid = styled(Grid)`
  width: 100% !important;
  height: 100%;
`

const Logo = styled.img`
  margin-right: 16px;
`

const GlobalTheme = createGlobalStyle`
html, body, body > div, body > div > div, main {
  height: 100%;
}
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
`

const theme = createMuiTheme({
  palette: {
    primary: blueGrey,
  },
})

const App: React.FC = ({ children }) => (
  <>
    <CssBaseline />
    <GlobalTheme />
    <ThemeProvider theme={theme}>{children}</ThemeProvider>
  </>
)

type Props = {
  path: string
  children: React.ReactNode
}

const Layout: React.FC<Props> = props => {
  const { description } = useSiteMetadata()
  return (
    <App>
      <OpenGraph path={props.path} />
      <Helmet>
        <meta
          name="viewport"
          content="minimum-scale=1, initial-scale=1, width=device-width, shrink-to-fit=no"
        />
        <meta name="description" content={description} />
        <title>Advent of Code 2019</title>
        <link rel="icon" type="image/png" href="/favicon.png" />
        <script type="module">{moduleScript}</script>
      </Helmet>
      <header>
        <AppBar position="static">
          <Toolbar>
            <Logo src="/favicon.png" width={32} height={32} alt="logo" />
            <Typography variant="h6">Advent of Code 2019</Typography>
          </Toolbar>
        </AppBar>
      </header>
      <main>
        <OuterGrid container spacing={3}>
          <Grid item md={2} sm={4} xs={6}>
            <NavMenu path={props.path} />
          </Grid>
          <ContentGrid item md={10} sm={8} xs={6}>
            <ContentPaper>{props.children}</ContentPaper>
          </ContentGrid>
        </OuterGrid>
      </main>
    </App>
  )
}

Layout.propTypes = {
  children: PropTypes.node.isRequired,
  path: PropTypes.string.isRequired,
}

export default Layout
