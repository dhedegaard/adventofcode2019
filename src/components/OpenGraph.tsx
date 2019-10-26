import React from "react"
import useSiteMetadata from "../hooks/useSiteMetadata"
import Helmet from "react-helmet"

type Props = {
  path: string
}

const OpenGraph: React.FC<Props> = props => {
  const siteMetadata = useSiteMetadata()
  const url = new URL(siteMetadata.siteUrl)
  url.pathname = props.path
  const image = new URL(siteMetadata.siteUrl)
  image.pathname = "/favicon.png"
  return (
    <Helmet>
      <meta content={siteMetadata.title} property="og:title" />
      <meta content="website" property="og:type"></meta>
      <meta content={siteMetadata.description} property="og:description" />
      <meta content={image.toString()} property="og:image" />
      <meta content={url.toString()} property="og:url" />
    </Helmet>
  )
}

export default OpenGraph
