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
import { PlayArea } from './PlayArea';
import { GameInfo } from './GameInfo';

export function Game({ view }: { view: GameView }): ReactNode {
  return (
    <div className="w-screen h-screen flex flex-row">
      <div className="flex flex-col w-5/6">
        <PlayArea view={view} />
      </div>
      <div className="flex flex-col justify-between w-1/6">
        <GameInfo view={view} />
      </div>
    </div>
  );
}
