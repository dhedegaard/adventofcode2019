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

const Layout: React.FC = ({ children }) => {
  return (
    <>
      <Helmet>
        <meta
          name="viewport"
          content="minimum-scale=1, initial-scale=1, width=device-width, shrink-to-fit=no"
        />
        <link
          rel="stylesheet"
          href="https://fonts.googleapis.com/css?family=Roboto:300,400,500,700&display=swap"
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
          <Grid item md={2}>
            <Paper>
              <NavMenu />
            </Paper>
          </Grid>
          <ContentGrid item md>
            <ContentPaper>{children}</ContentPaper>
          </ContentGrid>
        </OuterGrid>
      </main>
    </>
  )
}

Layout.propTypes = {
  children: PropTypes.node.isRequired,
}

export default Layout
