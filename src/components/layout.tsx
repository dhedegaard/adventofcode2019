/**
 * Layout component that queries for data
 * with Gatsby's useStaticQuery component
 *
 * See: https://www.gatsbyjs.org/docs/use-static-query/
 */
import MenuIcon from "@material-ui/icons/Menu"

import React from "react"
import PropTypes from "prop-types"
import { useStaticQuery, graphql } from "gatsby"
import { Helmet } from "react-helmet"
import {
  CssBaseline,
  Box,
  Grid,
  AppBar,
  Toolbar,
  Typography,
  IconButton,
  Paper,
} from "@material-ui/core"
import NavMenu from "./NavMenu"

const Layout = ({ children }) => {
  const data = useStaticQuery(graphql`
    query SiteTitleQuery {
      site {
        siteMetadata {
          title
        }
      }
    }
  `)

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
