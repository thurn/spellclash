// Copyright © spellclash 2024-present
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import { ReactNode, useContext } from 'react';
import { GlobalContext } from '../App';
import { Button, ButtonProps } from '@nextui-org/react';
import { handleAction } from '../server';
import { GameButtonView } from '../generated_types';

export function GameButton({ button, className }: { button: GameButtonView; className?: string }): ReactNode {
  const { response, setState } = useContext(GlobalContext);
  let color: ButtonProps['color'];
  switch (button.kind) {
    case 'primary':
      color = 'primary';
      break;
    case 'default':
      color = 'default';
      break;
  }

  return (
    <Button className={className} color={color} onClick={() => handleAction(setState, response, button.action)}>
      {button.label}
    </Button>
  );
}
