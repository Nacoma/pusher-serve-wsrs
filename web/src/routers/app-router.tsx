import React from 'react';
import {Switch, Route} from 'react-router';
import AppIndex from '../page/app-index';
import AppCreate from '../page/app-create';

const AppRouter: React.FC = () => (
  <Switch>
    <Route path={'/apps'} exact>
      <AppIndex/>
    </Route>
    <Route path={'/apps/create'} exact>
      <AppCreate/>
    </Route>
  </Switch>
);

export default AppRouter;
