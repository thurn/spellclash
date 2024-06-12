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
import { CardView, ClientCardId, GameView, Position } from '../generated_types';
import { LinearCardDisplay } from './LinearCardDisplay';
import { StackCardDisplay } from './StackCardDisplay';
import { DragManager, Items } from '../draggables/DragManager';
import { Card } from './Card';

export type PositionKey = string;

export type PositionMap = Map<PositionKey, CardView[]>;

export interface CardMap {
  readonly positions: PositionMap;
  readonly cards: Map<ClientCardId, CardView>;
}

export interface Props {
  readonly view: GameView;
}

export function PlayArea({ view }: Props): ReactNode {
  //  const clientData = useContext(GlobalContext);
  const map = cardPositions(view);

  return (
    <DragManager
      items={toDraggableItems(map)}
      renderItem={(id) => <Card cardId={id as string} map={map} />}
    >
      <div className="flex flex-row">
        <div className="w-11/12">
          <LinearCardDisplay
            key="oh"
            name="Opponent Hand"
            positionKey={keyForPosition({ hand: 'opponent' })}
            cardMap={map}
          />
          <LinearCardDisplay
            key="om"
            name="Opponent Mana"
            positionKey={keyForPosition({ battlefield: ['opponent', 'mana'] })}
            cardMap={map}
          />
          <LinearCardDisplay
            key="op"
            name="Opponent Permanents"
            positionKey={keyForPosition({ battlefield: ['opponent', 'permanents'] })}
            cardMap={map}
          />
          <LinearCardDisplay
            key="stack"
            name="Stack"
            positionKey={keyForPosition('stack')}
            cardMap={map}
            omitIfEmpty={true}
          />
          <LinearCardDisplay
            key="selection"
            name="Selection"
            positionKey={keyForPosition('cardSelectionChoices')}
            cardMap={map}
            omitIfEmpty={true}
          />
          <LinearCardDisplay
            key="unordered"
            name="Unordered"
            positionKey={keyForPosition({ cardOrderLocation: 'unordered' })}
            cardMap={map}
            omitIfEmpty={true}
            dropTarget={view.cardDragTargets.includes('unordered') ? 'unordered' : undefined}
          />
          <LinearCardDisplay
            key="topOfLibrary"
            name="Top Of Library"
            positionKey={keyForPosition({ cardOrderLocation: 'topOfLibrary' })}
            cardMap={map}
            omitIfEmpty={true}
            dropTarget={view.cardDragTargets.includes('topOfLibrary') ? 'topOfLibrary' : undefined}
          />
          <LinearCardDisplay
            key="vp"
            name="Viewer Permanents"
            positionKey={keyForPosition({ battlefield: ['viewer', 'permanents'] })}
            cardMap={map}
          />
          <LinearCardDisplay
            key="vm"
            name="Viewer Mana"
            positionKey={keyForPosition({ battlefield: ['viewer', 'mana'] })}
            cardMap={map}
          />
          <LinearCardDisplay
            key="vh"
            name="Viewer Hand"
            positionKey={keyForPosition({ hand: 'viewer' })}
            cardMap={map}
          />
        </div>
        <div className="w-1/12 flex flex-col justify-between">
          <StackCardDisplay
            key="og"
            cards={getPosition(map, keyForPosition({ discardPile: 'opponent' }))}
          />
          <StackCardDisplay
            key="vg"
            cards={getPosition(map, keyForPosition({ discardPile: 'viewer' }))}
          />
        </div>
      </div>
    </DragManager>
  );
}

function getPosition(map: CardMap, position: PositionKey): CardView[] {
  if (map.positions.has(position)) {
    return map.positions.get(position)!;
  } else {
    return [];
  }
}

function cardPositions(view: GameView): CardMap {
  const withKeys = new Map<PositionKey, [number, CardView][]>();
  const cards = new Map<ClientCardId, CardView>();
  for (const card of view.cards) {
    cards.set(card.id, card);
    const p = card.position;
    if (!withKeys.has(keyForPosition(p.position))) {
      withKeys.set(keyForPosition(p.position), []);
    }
    withKeys.get(keyForPosition(p.position))!.push([p.sortingKey, card]);
  }

  const positions = new Map<PositionKey, CardView[]>();
  for (const [position, array] of withKeys) {
    array.sort(function (a, b) {
      const x = a[0];
      const y = b[0];
      return x < y ? -1 : x > y ? 1 : 0;
    });
    positions.set(
      position,
      array.map(([_, card]) => card),
    );
  }
  return { positions, cards };
}

function toDraggableItems(map: CardMap): Items {
  const result: Items = {};
  for (const [position, cards] of map.positions) {
    result[position] = cards.map((card) => card.id);
  }
  result[keyForPosition({ cardOrderLocation: 'topOfLibrary' })] = [];
  return result;
}

function keyForPosition(position: Position): string {
  return JSON.stringify(position);
}
