import React from "react"
import { List, ListItem, ListItemText } from "@material-ui/core"
import Link from "next/link"
import { getDays } from "../mapper-utils"

const NavMenu: React.FC = () => (
  <List component="nav">
    <ListItem button>
      <Link href="/">
        <ListItemText>Index</ListItemText>
      </Link>
    </ListItem>
    {getDays().map(problem => (
      <ListItem button key={problem}>
        <Link href={`/${problem}`}>
          <ListItemText>{problem}</ListItemText>
        </Link>
      </ListItem>
    ))}
  </List>
)

export default NavMenu
