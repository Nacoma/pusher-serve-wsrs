import React from 'react';
import {
  Collapse,
  Drawer,
  List,
  ListItemText,
  makeStyles,
  Toolbar,
} from '@material-ui/core';

import ListLinkItem from './navigation-sidebar/list-link-item';
import {matchPath, useLocation, useRouteMatch} from 'react-router';

const useStyles = makeStyles((theme) => ({
  drawer: {
    backgroundColor: theme.palette.background.paper,
    width: 300,
  },

  nested: {
    paddingLeft: theme.spacing(4),
  },
}));

const NavigationSidebar: React.FC = () => {
  const classes = useStyles();

  const location = useLocation();


  const isApp = Boolean(matchPath(location.pathname, {
    path: '/apps',
  }));

  return (
    <Drawer variant={'permanent'} className={classes.drawer} classes={{
      paper: classes.drawer,
    }}>
      <Toolbar/>

      <List component={'nav'}>
        <ListLinkItem to={'/'} exact>
          <ListItemText primary={'Dashboard'}/>
        </ListLinkItem>

        <ListLinkItem to={'/apps'} exact>
          <ListItemText primary={'Apps'}/>
        </ListLinkItem>

        <Collapse in={isApp} timeout={'auto'} unmountOnExit>
          <List disablePadding>
            <ListLinkItem to={'/apps/create'} className={classes.nested}>
              &gt;&nbsp;<ListItemText primary={'New'}/>
            </ListLinkItem>
          </List>
        </Collapse>
      </List>
    </Drawer>
  );
};

export default NavigationSidebar;
