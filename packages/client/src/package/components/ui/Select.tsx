import React, { useContext } from 'react';
import { LocaleSelector } from '../../types/interfaces';
import ThemeContext from '../../Theme.context';
import s from './Select.module.scss';

function Select({
  value,
  children,
  onChange,
}: {
  value: string;
  children: typeof LocaleSelector;
  onChange: (e: React.ChangeEvent<HTMLSelectElement>) => void;
}) {
  const theme = useContext(ThemeContext);
  return (
    <select
      className={s.wrapper}
      onChange={onChange}
      value={value}
      style={{ background: theme.colors.active, color: theme.colors.paper }}
    >
      {children.map((item) => (
        <option key={item.value} disabled={!item.impl} value={item.value}>
          {item.name}
        </option>
      ))}
    </select>
  );
}

export default Select;
