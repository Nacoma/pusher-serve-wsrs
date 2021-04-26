import {
  configureStore,
  combineReducers,
  ThunkAction,
  AnyAction,
} from '@reduxjs/toolkit';
import {appsReducer} from './entities/app';

const reducer = combineReducers({
  app: appsReducer,
});

const store = configureStore({
  reducer,
});

type RootState = ReturnType<typeof store.getState>;
type AppDispatch = typeof store.dispatch;
type Action<R = void> = ThunkAction<R, RootState, unknown, AnyAction>;
type AsyncAction<R = void> = ThunkAction<
  Promise<R>,
  RootState,
  unknown,
  AnyAction
>;

export type {
  RootState,
  AppDispatch,
  Action,
  AsyncAction,
};

export default store;

