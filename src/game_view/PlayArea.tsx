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
import { BattlefieldPosition, CardView, DisplayPlayer, GameView, Position } from '../display_types';
import { LinearCardDisplay } from './LinearCardDisplay';
import { StackCardDisplay } from './StackCardDisplay';

export function PlayArea({ view }: { view: GameView }): ReactNode {
  const map = cardPositions(view);
  return (
    <div className="flex flex-row">
      <div className="w-11/12">
        <LinearCardDisplay key="oh" name="Opponent Hand" cards={getPosition(map, PositionKey.OpponentHand)} />
        <LinearCardDisplay
          key="om"
          name="Opponent Mana"
          cards={getPosition(map, PositionKey.OpponentBattlefieldMana)}
        />
        <LinearCardDisplay
          key="op"
          name="Opponent Permanents"
          cards={getPosition(map, PositionKey.OpponentBattlefield)}
        />
        <LinearCardDisplay key="st" name="Stack" cards={getPosition(map, PositionKey.Stack)} />
        <LinearCardDisplay key="vp" name="Viewer Permanents" cards={getPosition(map, PositionKey.ViewerBattlefield)} />
        <LinearCardDisplay key="vm" name="Viewer Mana" cards={getPosition(map, PositionKey.ViewerBattlefieldMana)} />
        <LinearCardDisplay key="vh" name="Viewer Hand" cards={getPosition(map, PositionKey.ViewerHand)} />
      </div>
      <div className="w-1/12 flex flex-col justify-between">
        <StackCardDisplay key="og" cards={getPosition(map, PositionKey.OpponentDiscardPile)} />
        <StackCardDisplay key="vg" cards={getPosition(map, PositionKey.ViewerDiscardPile)} />
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

function keyForPosition(position: Position): PositionKey {
  if (position === 'Stack') {
    return PositionKey.Stack;
  }

  if ('Hand' in position) {
    return position.Hand === DisplayPlayer.Viewer ? PositionKey.ViewerHand : PositionKey.OpponentHand;
  }

  if ('Deck' in position) {
    return position.Deck === DisplayPlayer.Viewer ? PositionKey.ViewerDeck : PositionKey.OpponentDeck;
  }

  if ('DiscardPile' in position) {
    return position.DiscardPile === DisplayPlayer.Viewer
      ? PositionKey.ViewerDiscardPile
      : PositionKey.OpponentDiscardPile;
  }

  if ('Exile' in position) {
    return position.Exile === DisplayPlayer.Viewer ? PositionKey.ViewerExile : PositionKey.OpponentExile;
  }

  if ('CommandZone' in position) {
    return position.CommandZone === DisplayPlayer.Viewer
      ? PositionKey.ViewerCommandZone
      : PositionKey.OpponentCommandZone;
  }

  if ('Battlefield' in position) {
    const [player, location] = position.Battlefield;
    if (player === DisplayPlayer.Viewer && location === BattlefieldPosition.Mana) {
      return PositionKey.ViewerBattlefieldMana;
    } else if (player === DisplayPlayer.Opponent && location === BattlefieldPosition.Mana) {
      return PositionKey.OpponentBattlefieldMana;
    } else if (player === DisplayPlayer.Viewer && location === BattlefieldPosition.Permanents) {
      return PositionKey.ViewerBattlefield;
    } else if (player === DisplayPlayer.Opponent && location === BattlefieldPosition.Permanents) {
      return PositionKey.OpponentBattlefield;
    }
  }

  throw Error('Unknown position: ' + JSON.stringify(position));
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
