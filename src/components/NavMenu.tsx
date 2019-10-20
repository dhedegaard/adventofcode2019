import React from "react"
import { List, ListItem, ListItemText } from "@material-ui/core"
import { problems } from "../problems"
import Link from "next/link"

const NavMenu: React.FC = () => (
  <List component="nav">
    <ListItem button>
      <Link href="/">
        <ListItemText>Index</ListItemText>
      </Link>
    </ListItem>
    {problems.map(problem => (
      <ListItem button key={problem}>
        <Link href={`/${problem}`}>
          <ListItemText>{problem}</ListItemText>
        </Link>
      </ListItem>
    ))}
  </List>
)

export default NavMenu
