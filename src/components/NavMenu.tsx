import React from "react"
import { List, ListItem, ListItemText } from "@material-ui/core"
import { Link } from "gatsby"
import UnstyledLink from "./UnstyledLink"
import { problems } from "../problems"

const NavMenu: React.FC = () => (
  <List component="nav">
    <ListItem button>
      <ListItemText>
        <UnstyledLink to="/">ASDF</UnstyledLink>
      </ListItemText>
      {problems.map(problem => (
        <ListItemText key={problem}>
          <UnstyledLink to={`/${problem}`}>{problem}</UnstyledLink>
        </ListItemText>
      ))}
    </ListItem>
  </List>
)

export default NavMenu
