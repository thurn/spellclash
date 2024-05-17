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
import { GameView } from '../display_types';
import { PlayerInfo } from './PlayerInfo';
import { GameButton } from '../core/GameButton';

export function GameInfo({ view }: { view: GameView }): ReactNode {
  const topButtons = view.top_buttons.map((button, i) => <GameButton button={button} key={i} className="m-2" />);
  const bottomButtons = view.bottom_buttons.map((button, i) => <GameButton button={button} key={i} className="m-2" />);
  return (
    <div className="flex flex-col grow justify-around text-center items-stretch">
      <div className="flex flex-col">{topButtons}</div>
      <PlayerInfo name="Opponent" player={view.opponent} />
      <div className="items-center m-1 text-m">{view.status_description}</div>
      <PlayerInfo name="Viewer" player={view.viewer} />
      <div className="flex flex-col">{bottomButtons}</div>
    </div>
  );
}
