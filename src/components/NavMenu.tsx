import React from "react"
import { List, ListItem, ListItemText, Paper } from "@material-ui/core"
import { Link } from "gatsby"
import styled from "styled-components"
import useProblems from "../hooks/useProblems"

const ContainerPaper = styled(Paper)`
  height: 100%;
`

type Props = {
  path: string
}

const NavMenu: React.FC<Props> = ({ path }) => {
  const problems = useProblems()
  return (
    <ContainerPaper square>
      <List component="nav">
        <ListItem
          button
          to="/"
          component={Link}
          divider
          selected={path === "/"}
        >
          <ListItemText>Index</ListItemText>
        </ListItem>
        {problems.map(problem => (
          <ListItem
            button
            key={problem}
            to={`/${problem}/`}
            component={Link}
            selected={path === `/${problem}/`}
          >
            <ListItemText>{problem}</ListItemText>
          </ListItem>
        ))}
      </List>
    </ContainerPaper>
  )
}

export default NavMenu
