import {
  createEntityAdapter,
  createSlice,
} from '@reduxjs/toolkit';
import {http} from '../http-utils';
import {AsyncAction} from '../store';

interface AppData {
  id: number
  key: string
  name: string
}

const appsAdapter = createEntityAdapter<AppData>();

type AdditionalState = {
};

// type State = EntityState<AppData> & AdditionalState;

const deleteApp = (id: number): AsyncAction => {
  return async (dispatch) => {
    await http<unknown>(`/api/apps/${id}`, {
      method: 'DELETE',
    });

    dispatch(appRemoved(id));
  };
};

const fetchApps = (): AsyncAction<AppData[]> => {
  return async (dispatch) => {
    const data = await http<AppData[]>('/api/apps');
    dispatch(appsReceived(data));
    return data;
  };
};

const storeApp = (name: string): AsyncAction<AppData> => {
  return async (dispatch) => {
    const data = await http<AppData>('/api/apps', {
      method: 'POST',
      body: JSON.stringify({name}),
      headers: {
        'Content-Type': 'application/json',
      },
    });

    dispatch(appAdded(data));

    return data;
  };
};

const slice = createSlice({
  name: 'app',
  initialState: appsAdapter.getInitialState<AdditionalState>({
  }),
  reducers: {
    appsReceived: appsAdapter.setAll,
    appAdded: appsAdapter.addOne,
    appRemoved: appsAdapter.removeOne,
  },
});

const appsSelector = appsAdapter.getSelectors();

const {
  actions,
  reducer: appsReducer,
} = slice;

const {
  appsReceived,
  appAdded,
  appRemoved,
} = actions;

export {
  fetchApps,
  storeApp,
  deleteApp,
  appsSelector,
  appsReducer,
};

export type {AppData};

