import React from 'react';
import {hot} from 'react-hot-loader';
import {v4} from 'uuid';
import {Button} from '@material-ui/core';
import {AppData} from '../../store';
import {InjectedWebSocketProps} from '../../hoc/with-web-socket';

interface ComponentProps {
  app: AppData
}

type Props = ComponentProps & InjectedWebSocketProps;

interface State {
  connected: boolean
  messages: string[]
  error: string | undefined
  readyState: ReadyState
}

class Connection extends React.Component<Props, State> {

  constructor(props: Props) {
    super(props);
  }

  render() {
    return (
      const {
        connect,
        disconnect,
        stateLabel,
        connected,
      } = this.props;

      <div>
        <h5>
          {this.props.id}

          <small>
            {' '}({stateLabel})
          </small>
        </h5>

        <button type={'button'} onClick={() => {
          if (!connected) {
            connect();
          } else {
            disconnect();
          }
        }}>
          {!connected ? 'Connect' : 'Disconnect'}
        </button>

        <br/>
        <br/>

        <Button color={'primary'} variant={'contained'} onClick={() => {
          this.setState({
            messages: [],
          });
        }}>
          Clear Messages
        </Button>
      </div>
    );
  }
}

export default hot(module)(Connection);
