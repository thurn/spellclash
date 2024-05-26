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

import { ReactNode } from 'react';
import { GameControlView } from '../generated_types';
import { GameButton } from './GameButton';
import { TextInput } from './TextInput';

export function GameControl({
  control,
  className,
}: {
  control: GameControlView;
  className?: string;
}): ReactNode {
  if ('button' in control) {
    return <GameButton button={control.button} className={className} />;
  } else if ('textInput' in control) {
    return <TextInput input={control.textInput} className={className} />;
  }
}
