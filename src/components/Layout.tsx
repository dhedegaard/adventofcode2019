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
      </Helmet>
      <CssBaseline />
      <main>
        <AppBar position="static">
          <Toolbar>
            <Typography variant="h6">Advent of Code 2019</Typography>
          </Toolbar>
        </AppBar>
        <Grid container>
          <Grid item md={2}>
            <Paper>
              <NavMenu />
            </Paper>
          </Grid>
          <Grid item>{children}</Grid>
        </Grid>
      </main>
    </>
  )
}

Layout.propTypes = {
  children: PropTypes.node.isRequired,
}

export default Layout
