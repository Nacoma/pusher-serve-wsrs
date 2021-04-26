import React, {useEffect, useState} from 'react';
import {
  AppData,
  AppDispatch,
  appsSelector,
  fetchApps,
  RootState,
} from '../store';
import {connect, ConnectedProps} from 'react-redux';
import {
  createStyles,
  FormControl, InputLabel,
  LinearProgress,
  NativeSelect,
  Theme, withStyles, WithStyles,
} from '@material-ui/core';

type Props = ConnectedProps<typeof connector> & WithStyles<typeof styles>;

interface State {
  app: AppData | undefined
}

class Dashboard extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {
      app: undefined,
    };
  }

  componentDidUpdate() {
    if (!this.state.app && this.props.apps.length > 0) {
      this.setState({
        app: this.props.apps[0],
      });
    }
  }

  componentDidMount() {
    this.props.fetchApps();
  }

  render() {
    const {classes, apps} = this.props;
    const {app} = this.state;

    if (apps.length === 0) {
      return <LinearProgress/>;
    }

    return (
      <div>
        <div className={classes.row}>
          <FormControl>
            <InputLabel>App</InputLabel>
            <NativeSelect value={String(app?.id)} className={classes.select}>
              {apps.map((app) => (
                <option value={String(app.id)} key={app.id}>
                  {app.name}
                </option>
              ))}
            </NativeSelect>
          </FormControl>
        </div>

        <div className={classes.row}>

        </div>
      </div>
    );
  }
}

const styles = (theme: Theme) => createStyles({
  select: {
    width: 300,
  },
  row: {
    marginBottom: theme.spacing(1),
  },
});

const mapStateToProps = (state: RootState) => ({
  apps: appsSelector.selectAll(state.app),
});

const mapDispatchToProps = (dispatch: AppDispatch) => ({
  fetchApps() {
    dispatch(fetchApps());
  },
});

const connector = connect(mapStateToProps, mapDispatchToProps);

export default withStyles(styles)(connector(Dashboard));
