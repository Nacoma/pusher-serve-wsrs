import React from 'react';
import {hot} from 'react-hot-loader';
import {createStyles, Theme, withStyles, WithStyles} from '@material-ui/core';

interface ComponentProps {
  messages: string[]
}

type Props = ComponentProps & WithStyles<typeof styles>;

interface State {
  //
}

const styles = (_: Theme) => createStyles({
  message: {
    overflowX: 'scroll',
  },
});

class WebSocketMessages extends React.PureComponent<Props, State> {
  render() {
    return (
      <div>
        {this.props.messages.map((message, key) => (
          <pre key={key} className={this.props.classes.message}>
            {JSON.stringify(
                JSON.parse(message),
                null,
                2,
            )}
          </pre>
        ))}
      </div>
    );
  }
}

export default hot(module)(withStyles(styles)(WebSocketMessages));

