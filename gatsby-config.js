module.exports = {
  siteMetadata: {
    title: `Advent of Code 2019`,
    description: `Implementations for Advent of Code 2019`,
    author: `Dennis Hedegaard <dennis@dhedegaard.dk>`,
    siteUrl: `https://aoc2019.dhedegaard.dk/`,
    githubUrl: `https://github.com/dhedegaard/adventofcode2019/`,
  },
  plugins: [
    `gatsby-plugin-typescript`,
    {
      resolve: `gatsby-source-filesystem`,
      options: {
        name: `problems`,
        path: `${__dirname}/aoc2019/src/`,
      },
    },
    `gatsby-plugin-react-helmet`,
    {
      resolve: `gatsby-plugin-material-ui`,
      options: {
        stylesProvider: {
          injectFirst: true,
        },
      },
    },
    `gatsby-plugin-styled-components`,
    {
      resolve: `gatsby-plugin-html-attributes`,
      options: {
        lang: "en",
      },
    },
    `gatsby-plugin-robots-txt`,
    `gatsby-plugin-sitemap`,
    {
      resolve: `gatsby-plugin-prefetch-google-fonts`,
      options: {
        fonts: [
          {
            family: `Roboto`,
            variants: [`300`, `400`, `500`, `700`],
          },
        ],
      },
    },
    {
      resolve: `gatsby-plugin-manifest`,
      options: {
        name: `Advent of Code 2019`,
        short_name: `AoC2019`,
        start_url: `/`,
        background_color: `#fafafa`,
        theme_color: `#546e7a`,
        display: `standalone`,
        icon: `static/favicon.png`,
      },
    },
    {
      resolve: `gatsby-plugin-offline`,
      options: {
        workboxConfig: {
          importWorkboxFrom: `cdn`,
          runtimeCaching: [
            {
              urlPattern: /.*/,
              handler: `NetworkFirst`,
            },
          ],
          skipWaiting: true,
          clientsClaim: true,
        },
      },
    },
  ],
}

exports.onCreateWebpackConfig = ({ actions, stage }) => {
  // If production JavaScript and CSS build
  if (stage === "build-javascript") {
    // Turn off source maps
    actions.setWebpackConfig({
      devtool: false,
    })
  }
}
