import {hot} from 'react-hot-loader';
import React from 'react';
import WebSocketClient from './WebSocketClient';
import {v4 as uuid} from 'uuid';
import {
  Button,
  createStyles,
  CssBaseline,
  Theme,
  WithStyles,
  withStyles,
} from '@material-ui/core';

const wsURL = `ws://localhost:6001/app/asdfzsv1234124412`;

interface State {
  clients: string[]
}

type Props = WithStyles<typeof styles>;

const styles = (theme: Theme) => createStyles({
  root: {
    display: 'flex',
    alignItems: 'flex-start',
    minHeight: '100vh',
  },

  client: {
    width: '25%',
    display: 'flex',
    justifyContent: 'center',
    flexDirection: 'column',
    padding: theme.spacing(1),
  },

  action: {
    padding: theme.spacing(1),
  },

  innerClient: {
    backgroundColor: theme.palette.background.paper,
    padding: theme.spacing(1),
  },
});

// eslint-disable-next-line require-jsdoc
class App extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);

    this.state = {
      clients: [
        uuid(),
        uuid(),
      ],
    };

    this.addClient = this.addClient.bind(this);
    this.removeClient = this.removeClient.bind(this);
  }

  addClient(): void {
    this.setState({
      clients: [...this.state.clients, uuid()],
    });
  }

  removeClient(id: string): void {
    this.setState({
      clients: this.state.clients.filter((_id) => _id !== id),
    });
  }

  render() {
    return (
      <>
        <CssBaseline/>

        <div>
          <Button
            onClick={this.addClient}
            color={'primary'}
            variant={'contained'}
          >
            Add Client
          </Button>

          <div className={this.props.classes.root}>
            {this.state.clients.map((id) => (
              <div className={this.props.classes.client} key={id}>
                <div className={this.props.classes.action}>
                  <Button
                    onClick={() => {
                      this.removeClient(id);
                    }}
                    color={'primary'}
                    variant={'contained'}
                  >
                    Remove Client
                  </Button>
                </div>

                <div className={this.props.classes.innerClient}>
                  <WebSocketClient url={wsURL} id={id}/>
                </div>
              </div>
            ))}
          </div>
        </div>
      </>
    );
  };
}

export default hot(module)(withStyles(styles)(App));

// {/* <ul>*/}
// {/*  <li>WS: {wsURL}</li>*/}
// {/*  <li>Ready State: {this.state.readyState}</li>*/}
// {/*  <li>Connected: {this.state.isOpen ? 1 : 0}</li>*/}
// {/*  <li>Error: {this.state.error || 'none'}</li>*/}
// {/* </ul>*/}
//
// {/* <hr/>*/}
//
// {/* <h1>Subscribe</h1>*/}
// {/* <form onSubmit={(e) => {*/}
// {/*  e.preventDefault();*/}
//
// {/*  this.ws?.send(JSON.stringify({*/}
// {/*    event: 'pusher:subscribe',*/}
// {/*    data: {*/}
// {/*      channel: this.state.sub,*/}
// {/*      channel_data: JSON.stringify(channelData),*/}
// {/*    },*/}
// {/*  }));*/}
//
// {/*  this.setState({*/}
// {/*    sub: '',*/}
// {/*  });*/}
// {/* }}>*/}
// {/*  <label htmlFor="channel">Channel</label>*/}
// {/*  <input*/}
// {/*    type="text"*/}
// {/*    id="channel"*/}
// {/*    value={this.state.sub}*/}
// {/*    onChange={(e) => {*/}
// {/*      this.setState({*/}
// {/*        sub: e.target.value,*/}
// {/*      });*/}
// {/*    }}/>*/}
// {/*  <br/>*/}
//
// {/*  <button type={'submit'}>*/}
// {/*    Subscribe*/}
// {/*  </button>*/}
// {/* </form>*/}
//
// {/* <div>*/}
// {/*  {this.state.messages.map((msg, i) => (*/}
// {/*    <p key={i}>*/}
// {/*      {msg}*/}
// {/*    </p>*/}
// {/*  ))}*/}
// {/* </div>*/}
//
// {/* <h1>Unsubscribe</h1>*/}
// {/* <form onSubmit={(e) => {*/}
// {/*  e.preventDefault();*/}
//
// {/*  this.ws?.send(JSON.stringify({*/}
// {/*    event: 'pusher:unsubscribe',*/}
// {/*    data: {*/}
// {/*      channel: this.state.unsub,*/}
// {/*    },*/}
// {/*  }));*/}
//
// {/*  this.setState({*/}
// {/*    unsub: '',*/}
// {/*  });*/}
// {/* }}>*/}
//
//
// {/*  <label htmlFor="channel">Channel</label>*/}
// {/*  <input*/}
// {/*    type="text"*/}
// {/*    id="channel"*/}
// {/*    value={this.state.unsub}*/}
// {/*    onChange={(e) => {*/}
// {/*      this.setState({*/}
// {/*        unsub: e.target.value,*/}
// {/*      });*/}
// {/*    }}/>*/}
// {/*  <br/>*/}
//
// {/*  <button type={'submit'}>*/}
// {/*    Unsubscribe*/}
// {/*  </button>*/}
//
// {/* </form>*/}
