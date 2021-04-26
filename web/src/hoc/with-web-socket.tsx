import React, {ComponentClass} from 'react';
import {AppData} from '../store';
import {Subtract} from 'utility-types';

export enum ReadyState {
  CONNECTING = 0,
  OPEN = 1,
  CLOSING = 2,
  CLOSED = 3,
}

export interface InjectedWebSocketProps {
  connect(app: AppData, id: string): void
  disconnect(): void
  state: ReadyState
  connected: boolean
  messages: string[],
  stateLabel: string
  send(data: Record<any, any>): void
}

interface WebSocketState {
  state: ReadyState
  error: string | undefined
  messages: string[],
  connected: boolean
}

export const withWebSocket = <P extends InjectedWebSocketProps>(
  Component: React.ComponentType<P>,
) => {
  return class WithWebSocket extends React.Component<
    Subtract<P, InjectedWebSocketProps>,
    WebSocketState
  > {
    private ws: WebSocket | undefined;
    private interval: any;

    constructor(props: Subtract<P, InjectedWebSocketProps>) {
      super(props);

      this.state = {
        connected: false,
        state: ReadyState.CLOSED,
        error: undefined,
        messages: [],
      };
    }

    connect(app: AppData) {
      this.ws = new WebSocket(URL + app.key);

      this.setState({
        state: ReadyState.CLOSED,
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
          state: this.ws?.readyState || ReadyState.CLOSED,
        });
      }, 1000);
    }

    send(data: Record<string, any>): void {
      this.ws?.send(JSON.stringify(data));
    }

    disconnect() {
      this.ws?.close();
      this.ws = undefined;
    }

    componentWillUnmount() {
      this.disconnect();
    }

    readyStateLabel(): string {
      switch (this.state.state) {
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
        <Component {...this.props as P}/>
      );
    }
  };
};
