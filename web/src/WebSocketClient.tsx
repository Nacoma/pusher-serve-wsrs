import React from 'react';
import {hot} from 'react-hot-loader';
import WebSocketMessages from './WebSocketMessages';
import StringForm from './StringForm';
import {Button} from '@material-ui/core';

interface ComponentProps {
  url: string
  id: string
}

type Props = ComponentProps;

interface State {
  connected: boolean
  messages: string[]
  error: string | undefined
  readyState: ReadyState
}

enum ReadyState {
  CONNECTING = 0,
  OPEN = 1,
  CLOSING = 2,
  CLOSED = 3,
}

class WebSocketClient extends React.Component<Props, State> {
  ws: WebSocket | undefined;
  interval: any;

  constructor(props: Props) {
    super(props);

    this.state = {
      connected: false,
      messages: [],
      error: undefined,
      readyState: ReadyState.CLOSED,
    };

    this.send = this.send.bind(this);
    this.connect = this.connect.bind(this);
    this.disconnect = this.disconnect.bind(this);
  }

  connect(): void {
    this.ws = new WebSocket(this.props.url);

    this.setState({
      readyState: -1,
    });

    this.ws.addEventListener('error', (e) => {
      this.setState({
        error: String(e),
      });
    });

    this.ws.addEventListener('message', (e) => {
      this.setState({
        messages: [...this.state.messages, e.data as string],
      });
    });

    this.interval = setInterval(() => {
      this.setState({
        readyState: this.ws?.readyState || ReadyState.CLOSED,
      });
    }, 1000);
  }

  send(data: string) {
    this.ws?.send(data);
  }

  disconnect(): void {
    this.ws?.close();
    this.ws = undefined;
  }

  componentWillUnmount() {
    this.disconnect();
  }

  isClosed(): boolean {
    return this.state.readyState === ReadyState.CLOSED;
  }

  readyStateLabel(): string {
    switch (this.state.readyState) {
      case ReadyState.CLOSED:
        return 'CLOSED';
      case ReadyState.CLOSING:
        return 'CLOSING';
      case ReadyState.CONNECTING:
        return 'CONNECTING';
      default:
        return 'CONNECTED';
    }
  }

  render() {
    return (
      <div>
        <h5>
          {this.props.id}

          <small>
            {' '}({this.readyStateLabel()})
          </small>
        </h5>

        <button type={'button'} onClick={() => {
          if (this.isClosed()) {
            this.connect();
          } else {
            this.disconnect();
          }
        }}>
          {this.isClosed() ? 'Connect' : 'Disconnect'}
        </button>

        <br/>
        <br/>

        <StringForm
          disabled={this.isClosed()}
          label={'Join'}
          submit={(channel) => {
            this.send(JSON.stringify({
              'event': 'pusher:subscribe',
              'data': {
                'channel': channel,
                'channel_data': JSON.stringify({
                  'user_id': this.props.id,
                  'user_info': {
                    'id': this.props.id,
                  },
                }),
              },
            }));
          }}
        />

        <br/>
        <br/>

        <StringForm
          disabled={this.isClosed()}
          label={'Leave'}
          submit={(channel) => {
            this.send(JSON.stringify({
              'event': 'pusher:unsubscribe',
              'data': {
                'channel': channel,
              },
            }));
          }}
        />

        <Button color={'primary'} variant={'contained'} onClick={() => {
          this.setState({
            messages: [],
          });
        }}>
          Clear Messages
        </Button>

        <WebSocketMessages messages={this.state.messages}/>
      </div>
    );
  }
}

export default hot(module)(WebSocketClient);
