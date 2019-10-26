import React from "react"
import useSiteMetadata from "../hooks/useSiteMetadata"
import Helmet from "react-helmet"

type Props = {
  path: string
}

const siteUrlWithPath = (siteUrl: string, path: string): string => {
  const url = new URL(siteUrl)
  url.pathname = path
  return url.toJSON()
}

const OpenGraph: React.FC<Props> = props => {
  const { title, description, siteUrl } = useSiteMetadata()
  return (
    <Helmet>
      <meta content={title} property="og:title" />
      <meta content="website" property="og:type"></meta>
      <meta content={description} property="og:description" />
      <meta
        content={siteUrlWithPath(siteUrl, "/favicon.png")}
        property="og:image"
      />
      <meta content={siteUrlWithPath(siteUrl, props.path)} property="og:url" />
    </Helmet>
  )
}

export default OpenGraph
