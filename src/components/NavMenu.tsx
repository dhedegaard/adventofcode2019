import React from "react"
import { List, ListItem, ListItemText } from "@material-ui/core"
import { Link } from "gatsby"
import { getDays } from "../mapper-utils"

type Props = {
  path: string
}

const NavMenu: React.FC<Props> = ({ path }) => {
  return (
    <List component="nav">
      <ListItem button to="/" component={Link} divider selected={path === "/"}>
        <ListItemText>Index</ListItemText>
      </ListItem>
      {getDays().map(problem => (
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
  )
}

export default NavMenu
