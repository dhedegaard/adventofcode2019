import React from "react"
import { List, ListItem, ListItemText } from "@material-ui/core"
import { navigate } from "gatsby"
import { getDays } from "../mapper-utils"

const NavMenu: React.FC = () => (
  <List component="nav">
    <ListItem button onClick={() => navigate("/")}>
      <ListItemText>Index</ListItemText>
    </ListItem>
    {getDays().map(problem => (
      <ListItem button key={problem} onClick={() => navigate(`/${problem}`)}>
        <ListItemText>{problem}</ListItemText>
      </ListItem>
    ))}
  </List>
)

export default NavMenu
