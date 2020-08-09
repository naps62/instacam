import React from "react";
import { HashRouter as Router, Switch, Route } from "react-router-dom";
import {
  AppBar,
  Toolbar,
  IconButton,
  Typography,
  Grid,
  Container,
  Paper,
} from "@material-ui/core";
import MenuIcon from "@material-ui/icons/Menu";
import { makeStyles } from "@material-ui/core/styles";

import Settings from "./pages/settings";
import Filters from "./pages/filters";
import Video from "./components/video";
import "./style.css";

const useStyles = makeStyles((theme) => ({
  container: {
    padding: 20,
  },
  root: {
    flexGrow: 1,
  },
  paper: {
    minHeight: 500,
    padding: 20,
  },
  control: {
    padding: theme.spacing(2),
  },
}));

const App = () => {
  const classes = useStyles();

  return (
    <Router>
      <AppBar position="static">
        <Toolbar>
          <IconButton edge="start" color="inherit" aria-label="menu">
            <MenuIcon />
          </IconButton>
          <Typography variant="h6">Instacam</Typography>
        </Toolbar>
      </AppBar>

      <Container className={classes.container}>
        <Grid
          container
          className={classes.root}
          spacing={5}
          direction="row"
          alignItems="stretch"
        >
          <Grid item xs={12} sm={6}>
            <Paper className={classes.paper}>
              <Switch>
                <Route path="/">
                  <Settings />
                </Route>
                <Route path="/filters">
                  <Filters />
                </Route>
              </Switch>
            </Paper>
          </Grid>
          <Grid item xs={12} sm={6}>
            <Paper className={classes.paper}>
              <Video />
            </Paper>
          </Grid>
        </Grid>
      </Container>
    </Router>
  );
};

export default App;
