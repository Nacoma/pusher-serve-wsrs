import React from 'react';

import {ListItem, ListItemProps} from '@material-ui/core';
import {NavLink} from 'react-router-dom';

interface ComponentProps {
  to: string
  exact?: boolean
}

type Props = ComponentProps & ListItemProps;

const ListLinkItem: React.FC<Props> = ({
  children,
  to,
  exact = false,
  ...props
}) => (
  <ListItem
    // @ts-ignore
    button
    component={NavLink}
    activeClassName={'Mui-selected'}
    to={to}
    exact={exact}
    {...props}
  >
    {children}
  </ListItem>
);

export default ListLinkItem;
