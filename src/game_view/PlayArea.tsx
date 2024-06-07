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

import { Dispatch, ReactNode, SetStateAction, createContext, useState } from 'react';
import { CardView, GameView, Position } from '../generated_types';
import { LinearCardDisplay } from './LinearCardDisplay';
import { StackCardDisplay } from './StackCardDisplay';
import { Popover, PopoverContent, PopoverTrigger } from '@nextui-org/react';

export type PositionKey = string;

export interface CardPreviewImageContext {
  readonly previewImage: string;
  readonly setPreviewImage?: Dispatch<SetStateAction<string>>;
}
export const CardPreviewImage: React.Context<CardPreviewImageContext> = createContext({
  previewImage: '',
});

export function PlayArea({ view }: { view: GameView }): ReactNode {
  const map = cardPositions(view);
  const previewHeight = 40;
  const [previewImage, setPreviewImage] = useState('');
  const cardPreview = (
    <Popover isOpen={previewImage !== ''}>
      <PopoverTrigger> </PopoverTrigger>
      <PopoverContent>
        <img
          src={previewImage}
          style={{
            height: `${previewHeight}vh`,
            width: `${previewHeight * (5 / 7)}vh`,
          }}
        />
      </PopoverContent>
    </Popover>
  );

  return (
    <CardPreviewImage.Provider
      value={{ previewImage: previewImage, setPreviewImage: setPreviewImage }}
    >
      <div className="flex flex-row">
        {cardPreview}
        <div className="w-11/12">
          <LinearCardDisplay
            key="oh"
            name="Opponent Hand"
            cards={getPosition(map, keyForPosition({ hand: 'opponent' }))}
          />
          <LinearCardDisplay
            key="om"
            name="Opponent Mana"
            cards={getPosition(map, keyForPosition({ battlefield: ['opponent', 'mana'] }))}
          />
          <LinearCardDisplay
            key="op"
            name="Opponent Permanents"
            cards={getPosition(map, keyForPosition({ battlefield: ['opponent', 'permanents'] }))}
          />
          <LinearCardDisplay
            key="st"
            name="Stack"
            cards={getPosition(map, keyForPosition('stack'))}
            omitIfEmpty={true}
          />
          <LinearCardDisplay
            key="st"
            name="Selection"
            cards={getPosition(map, keyForPosition('cardSelectionChoices'))}
            omitIfEmpty={true}
          />
          <LinearCardDisplay
            key="st"
            name="Top Of Library"
            cards={getPosition(map, keyForPosition({ cardSelectionLocation: 'topOfLibrary' }))}
            omitIfEmpty={!view.cardDragTargets.includes('topOfLibrary')}
          />
          <LinearCardDisplay
            key="vp"
            name="Viewer Permanents"
            cards={getPosition(map, keyForPosition({ battlefield: ['viewer', 'permanents'] }))}
          />
          <LinearCardDisplay
            key="vm"
            name="Viewer Mana"
            cards={getPosition(map, keyForPosition({ battlefield: ['viewer', 'mana'] }))}
          />
          <LinearCardDisplay
            key="vh"
            name="Viewer Hand"
            cards={getPosition(map, keyForPosition({ hand: 'viewer' }))}
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
    </CardPreviewImage.Provider>
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
    withKeys.get(keyForPosition(p.position))!.push([p.sortingKey, card]);
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
