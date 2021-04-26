import React, {useEffect} from 'react';
import {AppData, AppDispatch, deleteApp, fetchApps, RootState} from '../store';
import {connect} from 'react-redux';
import {appsSelector} from '../store/entities/app';
import {
  Button,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableRow,
} from '@material-ui/core';

interface ComponentProps {

}

interface StateProps {
  apps: AppData[],
}

interface DispatchProps {
  fetchApps(): void
  deleteApp(id: number): void
}

type Props = ComponentProps & StateProps & DispatchProps;

const AppIndex: React.FC<Props> = ({
  fetchApps,
  deleteApp,
  apps,
}) => {
  useEffect(() => {
    fetchApps();
  }, []);

  return (
    <Table>
      <TableHead>
        <TableRow>
          <TableCell variant={'head'} component={'th'}>ID</TableCell>
          <TableCell variant={'head'} component={'th'}>Name</TableCell>
          <TableCell variant={'head'} component={'th'}>Key</TableCell>
          <TableCell variant={'head'} component={'th'}/>
        </TableRow>
      </TableHead>
      <TableBody>
        {apps.map((app) => (
          <TableRow key={app.id}>
            <TableCell>{app.id}</TableCell>
            <TableCell>{app.name}</TableCell>
            <TableCell>{app.key}</TableCell>
            <TableCell>
              <Button color={'secondary'} onClick={() => {
                deleteApp(app.id);
              }}>
                Delete
              </Button>
            </TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  );
};

const mapStateToProps = (state: RootState): StateProps => {
  return {
    apps: appsSelector.selectAll(state.app),
  };
};

const mapDispatchToProps = (dispatch: AppDispatch): DispatchProps => {
  return {
    fetchApps(): void {
      dispatch(fetchApps());
    },

    deleteApp(id: number): void {
      dispatch(deleteApp(id));
    },
  };
};

export default connect(mapStateToProps, mapDispatchToProps)(AppIndex);

