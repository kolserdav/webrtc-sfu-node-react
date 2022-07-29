/******************************************************************************************
 * Repository: https://github.com/kolserdav/werift-sfu-react.git
 * File name: Icon.tsx
 * Author: Sergey Kolmiller
 * Email: <uyem.ru@gmail.com>
 * License: MIT
 * License text: See in LICENSE file
 * Copyright: kolserdav, All rights reserved (c)
 * Create Date: Fri Jul 29 2022 21:35:51 GMT+0700 (Krasnoyarsk Standard Time)
 ******************************************************************************************/
import React from 'react';

export interface IconProps {
  children: string;
  color?: string;
}

function Icon({ color, children }: IconProps) {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      version="1.1"
      width="24px"
      height="24px"
      viewBox="0 0 24 24"
    >
      <path fill={`${color}`} d={children} />
    </svg>
  );
}

Icon.defaultProps = {
  color: '#000',
};

export default Icon;
