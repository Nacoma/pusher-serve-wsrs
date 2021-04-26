import {hot} from 'react-hot-loader';
import React from 'react';
import {
  AppBar, createStyles,
  CssBaseline,
  IconButton, Theme,
  Toolbar,
  Typography, WithStyles, withStyles,
} from '@material-ui/core';
import MenuIcon from '@material-ui/icons/Menu';

import {Provider} from 'react-redux';
import store from './store';
import {BrowserRouter as Router, Switch, Route} from 'react-router-dom';
import {Redirect} from 'react-router';
import AppRouter from './routers/app-router';
import NavigationSidebar from './components/navigation-sidebar';
import Dashboard from './page/dashboard';

type Props = WithStyles<typeof styles>;
type State = {};

class App extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);
  }

  render() {
    const {classes} = this.props;

    return (
      <>
        <CssBaseline/>

        <AppBar position={'relative'} className={classes.appBar}>
          <Toolbar>
            <IconButton edge={'start'} color={'inherit'}>
              <MenuIcon/>
            </IconButton>
            <Typography variant={'h6'}>
              Pusher WsRs
            </Typography>
          </Toolbar>
        </AppBar>

        <Provider store={store}>
          <Router>
            <div className={classes.page}>
              <NavigationSidebar/>

              <div className={classes.content}>
                <Switch>
                  <Route path={'/'} exact>
                    <Dashboard/>
                  </Route>

                  <Route path={'/apps'}>
                    <AppRouter/>
                  </Route>
                </Switch>
              </div>
            </div>
          </Router>
        </Provider>
      </>
    );
  };
}

const styles = (theme: Theme) => createStyles({
  appBar: {
    zIndex: theme.zIndex.drawer + 1,
  },

  page: {
    display: 'flex',
  },

  content: {
    padding: theme.spacing(1),
    flexGrow: 1,
  },
});

export default hot(module)(withStyles(styles)(App));
