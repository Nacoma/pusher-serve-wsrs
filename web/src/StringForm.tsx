import React, {useState} from 'react';
import {hot} from 'react-hot-loader';
import {Button, TextField} from '@material-ui/core';

interface ComponentProps {
  submit: (channel: string) => void
  label: string
  disabled: boolean
}

type Props = ComponentProps;

const StringForm: React.FC<Props> = ({
  submit,
  disabled,
  label,
}) => {
  const [str, setStr] = useState<string>('');

  return (
    <form onSubmit={(e) => {
      e.preventDefault();

      submit(str);

      setStr('');
    }}>
      <TextField
        value={str}
        size={'small'}
        variant={'outlined'}
        disabled={disabled}
        onChange={(e) => {
          setStr(e.target.value);
        }}
      />

      {' '}

      <Button
        type={'submit'}
        variant={'contained'}
        color={'secondary'}
        disabled={disabled}
      >
        {label}
      </Button>
    </form>
  );
};

export default hot(module)(StringForm);
