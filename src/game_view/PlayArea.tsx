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
import { CardView, GameView, Position } from '../generated_types';
import { LinearCardDisplay } from './LinearCardDisplay';
import { StackCardDisplay } from './StackCardDisplay';

export type PositionKey = string;

export function PlayArea({ view }: { view: GameView }): ReactNode {
  const map = cardPositions(view);
  return (
    <div className="flex flex-row">
      <div className="w-11/12">
        <LinearCardDisplay
          key="oh"
          name="Opponent Hand"
          cards={getPosition(map, keyForPosition({ Hand: 'Opponent' }))}
        />
        <LinearCardDisplay
          key="om"
          name="Opponent Mana"
          cards={getPosition(map, keyForPosition({ Battlefield: ['Opponent', 'Mana'] }))}
        />
        <LinearCardDisplay
          key="op"
          name="Opponent Permanents"
          cards={getPosition(map, keyForPosition({ Battlefield: ['Opponent', 'Permanents'] }))}
        />
        <LinearCardDisplay key="st" name="Stack" cards={getPosition(map, keyForPosition('Stack'))} />
        <LinearCardDisplay
          key="vp"
          name="Viewer Permanents"
          cards={getPosition(map, keyForPosition({ Battlefield: ['Viewer', 'Permanents'] }))}
        />
        <LinearCardDisplay
          key="vm"
          name="Viewer Mana"
          cards={getPosition(map, keyForPosition({ Battlefield: ['Viewer', 'Mana'] }))}
        />
        <LinearCardDisplay key="vh" name="Viewer Hand" cards={getPosition(map, keyForPosition({ Hand: 'Viewer' }))} />
      </div>
      <div className="w-1/12 flex flex-col justify-between">
        <StackCardDisplay key="og" cards={getPosition(map, keyForPosition({ DiscardPile: 'Opponent' }))} />
        <StackCardDisplay key="vg" cards={getPosition(map, keyForPosition({ DiscardPile: 'Viewer' }))} />
      </div>
    </div>
  );
}

function getPosition(map: Map<PositionKey, CardView[]>, position: PositionKey): CardView[] {
  if (map.has(position)) {
    return map.get(position)!;
  } else {
    return [];
  }
}

function cardPositions(view: GameView): Map<PositionKey, CardView[]> {
  const withKeys = new Map<PositionKey, [number, CardView][]>();
  for (const card of view.cards) {
    const p = card.position;
    if (!withKeys.has(keyForPosition(p.position))) {
      withKeys.set(keyForPosition(p.position), []);
    }
    withKeys.get(keyForPosition(p.position))!.push([p.sorting_key, card]);
  }
  const result = new Map<PositionKey, CardView[]>();
  for (const [position, array] of withKeys) {
    array.sort(function (a, b) {
      const x = a[0];
      const y = b[0];
      return x < y ? -1 : x > y ? 1 : 0;
    });
    result.set(
      position,
      array.map(([_, card]) => card),
    );
  }
  return result;
}

function keyForPosition(position: Position): string {
  return JSON.stringify(position);
}
