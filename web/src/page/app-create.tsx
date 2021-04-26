import React, {FormEvent, useState} from 'react';
import {Button, TextField} from '@material-ui/core';
import {AppDispatch} from '../store';
import {connect, ConnectedProps} from 'react-redux';
import {storeApp} from '../store/entities/app';
import {Redirect} from 'react-router';

type Props = ConnectedProps<typeof connector>;

const AppCreate: React.FC<Props> = ({
  storeApp,
}) => {
  const [name, setName] = useState<string>('');
  const [loading, setLoading] = useState<boolean>(false);
  const [isSuccess, setIsSuccess] = useState<boolean>(false);

  const onSubmit = (e: FormEvent) => {
    e.preventDefault();

    setLoading(true);

    storeApp({name})
        .then(() => {
          setLoading(false);
          setIsSuccess(true);
        });
  };

  if (isSuccess) {
    return <Redirect to={'/apps'}/>;
  }

  return (
    <form onSubmit={onSubmit}>
      <TextField
        required
        label={'Name'}
        value={name}
        onChange={(e) => setName(e.target.value)}
      />

      <Button type={'submit'} disabled={loading}>
        Save
      </Button>
    </form>
  );
};

const mapDispatchToProps = (dispatch: AppDispatch) => {
  return {
    storeApp(data: {name: string}) {
      return dispatch(storeApp(data.name));
    },
  };
};

const connector = connect(null, mapDispatchToProps);

export default connector(AppCreate);
