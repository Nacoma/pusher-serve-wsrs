import React from 'react';
import {AppData} from '../store';
import {NativeSelect} from '@material-ui/core';

interface ComponentProps {
  selectApp(id: number): any
  apps: AppData[]
  app: AppData | undefined
}

type Props = ComponentProps;

const SelectApp: React.FC<Props> = ({
  selectApp,
  apps,
  app,
}) => {
  return (
    <NativeSelect value={app?.id} onChange={(e) => {
      selectApp(Number(e.target.value));
    }}>
      {apps.map((app) => (
        <option value={app.id} key={app.id}>
          {app.name}
        </option>
      ))}
    </NativeSelect>
  );
};

export default SelectApp;
