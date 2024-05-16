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
        cards={getPosition(map, PositionKey.OpponentHand)}
      />
      <Zone
        name="Opponent Mana"
        cards={getPosition(map, PositionKey.OpponentBattlefieldMana)}
      />
      <Zone
        name="Opponent Permanents"
        cards={getPosition(map, PositionKey.OpponentBattlefield)}
      />
      <Zone
        name="Stack"
        cards={getPosition(map, PositionKey.Stack)}
      />
      <Zone
        name="Viewer Permanents"
        cards={getPosition(map, PositionKey.ViewerBattlefield)}
      />
      <Zone
        name="Viewer Mana"
        cards={getPosition(map, PositionKey.ViewerBattlefieldMana)}
      />
      <Zone
        name="Viewer Hand"
        cards={getPosition(map, PositionKey.ViewerHand)}
      />
    </div>
  );
}

function getPosition(
  map: Map<PositionKey, CardView[]>,
  position: PositionKey
): CardView[] {
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
      var x = a[0];
      var y = b[0];
      return x < y ? -1 : x > y ? 1 : 0;
    });
    result.set(
      position,
      array.map(([_, card]) => card)
    );
  }
  return result;
}

function keyForPosition(position: Position): PositionKey {
  if (position.Stack != null) {
    return PositionKey.Stack;
  }

  if (position.Hand != null) {
    return position.Hand === DisplayPlayer.Viewer
      ? PositionKey.ViewerHand
      : PositionKey.OpponentHand;
  }

  if (position.Deck != null) {
    return position.Deck === DisplayPlayer.Viewer
      ? PositionKey.ViewerDeck
      : PositionKey.OpponentDeck;
  }

  if (position.DiscardPile != null) {
    return position.DiscardPile === DisplayPlayer.Viewer
      ? PositionKey.ViewerDiscardPile
      : PositionKey.OpponentDiscardPile;
  }

  if (position.Exile != null) {
    return position.Exile === DisplayPlayer.Viewer
      ? PositionKey.ViewerExile
      : PositionKey.OpponentExile;
  }

  if (position.CommandZone != null) {
    return position.CommandZone === DisplayPlayer.Viewer
      ? PositionKey.ViewerCommandZone
      : PositionKey.OpponentCommandZone;
  }

  if (position.Battlefield != null) {
    return position.Battlefield === DisplayPlayer.Viewer
      ? PositionKey.ViewerBattlefield
      : PositionKey.OpponentBattlefield;
  }

  throw Error("Unknown position: " + JSON.stringify(position));
}

enum PositionKey {
  Stack,
  ViewerHand,
  OpponentHand,
  ViewerDeck,
  OpponentDeck,
  ViewerDiscardPile,
  OpponentDiscardPile,
  ViewerExile,
  OpponentExile,
  ViewerCommandZone,
  OpponentCommandZone,
  ViewerBattlefield,
  OpponentBattlefield,
  ViewerBattlefieldMana,
  OpponentBattlefieldMana,
}
