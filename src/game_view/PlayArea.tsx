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

import { ReactNode } from "react";
import { CardView, DisplayPlayer, GameView, Position } from "../display_types";
import { Zone } from "./Zone";

export function PlayArea({ view }: { view: GameView }): ReactNode {
  const map = cardPositions(view);
  return (
    <div>
      <Zone
        name="Opponent Hand"
        cards={getPosition(map, { Hand: DisplayPlayer.Opponent })}
      />
      <Zone
        name="Opponent Mana"
        cards={getPosition(map, { Hand: DisplayPlayer.Opponent })}
      />
      <Zone
        name="Opponent Permanents"
        cards={getPosition(map, { Hand: DisplayPlayer.Opponent })}
      />
      <Zone
        name="Stack"
        cards={getPosition(map, { Hand: DisplayPlayer.Opponent })}
      />
      <Zone
        name="Viewer Permanents"
        cards={getPosition(map, { Hand: DisplayPlayer.Viewer })}
      />
      <Zone
        name="Viewer Mana"
        cards={getPosition(map, { Hand: DisplayPlayer.Viewer })}
      />
      <Zone
        name="Viewer Hand"
        cards={getPosition(map, { Hand: DisplayPlayer.Viewer })}
      />
    </div>
  );
}

function getPosition(map: Map<Position, CardView[]>, position: Position): CardView[] {
  if (map.has(position)) {
    return map.get(position)!;
  } else {
    return [];
  }
}

function cardPositions(view: GameView): Map<Position, CardView[]> {
  const withKeys = new Map<Position, [number, CardView][]>();
  for (const card of view.cards) {
    const p = card.position;
    if (withKeys.has(p.position)) {
      withKeys.set(p.position, []);
    }
    withKeys.get(p.position)!.push([p.sorting_key, card]);
  }
  const result = new Map<Position, CardView[]>();
  for (const [position, array] of withKeys) {
    array.sort(function (a, b) {
      var x = a[0];
      var y = b[0];
      return x < y ? -1 : x > y ? 1 : 0;
    });
    result.set(position, array.map(([_, card]) => card));
  }
  return result;
}