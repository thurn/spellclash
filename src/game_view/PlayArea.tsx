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
import { CardOrderLocation, CardView, ClientCardId, GameView, Position } from '../generated_types';
import { LinearCardDisplay } from './LinearCardDisplay';
import { StackCardDisplay } from './StackCardDisplay';
import { Popover, PopoverContent, PopoverTrigger } from '@nextui-org/react';
import { DndContext } from '@dnd-kit/core';
import { useContext } from 'react';
import { GlobalContext } from '../App';
import { dragCard } from '../server';

export type PositionKey = string;
export type PositionMap = Map<PositionKey, CardView[]>;

export interface CardPreviewImageContext {
  readonly previewImage: string;
  readonly setPreviewImage?: Dispatch<SetStateAction<string>>;
}
export const CardPreviewImage: React.Context<CardPreviewImageContext> = createContext({
  previewImage: '',
});

export interface Props {
  readonly view: GameView;
}

export function PlayArea({ view }: Props): ReactNode {
  const clientData = useContext(GlobalContext);
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
      <DndContext
        onDragEnd={(e) => {
          const cardId = e.active.data.current?.cardId as ClientCardId;
          const location = e.over?.data?.current?.dropTarget as CardOrderLocation;
          console.log('onDragEnd for card ' + cardId + ' to ' + location);
          if (cardId != null && location != null) {
            dragCard(clientData, cardId, location, 0);
          }
        }}
      >
        <div className="flex flex-row">
          {cardPreview}
          <div className="w-11/12">
            <LinearCardDisplay
              key="oh"
              name="Opponent Hand"
              positionKey={keyForPosition({ hand: 'opponent' })}
              positionMap={map}
            />
            <LinearCardDisplay
              key="om"
              name="Opponent Mana"
              positionKey={keyForPosition({ battlefield: ['opponent', 'mana'] })}
              positionMap={map}
            />
            <LinearCardDisplay
              key="op"
              name="Opponent Permanents"
              positionKey={keyForPosition({ battlefield: ['opponent', 'permanents'] })}
              positionMap={map}
            />
            <LinearCardDisplay
              key="stack"
              name="Stack"
              positionKey={keyForPosition('stack')}
              positionMap={map}
              omitIfEmpty={true}
            />
            <LinearCardDisplay
              key="selection"
              name="Selection"
              positionKey={keyForPosition('cardSelectionChoices')}
              positionMap={map}
              omitIfEmpty={true}
            />
            <LinearCardDisplay
              key="unordered"
              name="Unordered"
              positionKey={keyForPosition({ cardOrderLocation: 'unordered' })}
              positionMap={map}
              omitIfEmpty={true}
              dropTarget={view.cardDragTargets.includes('unordered') ? 'unordered' : undefined}
            />
            <LinearCardDisplay
              key="topOfLibrary"
              name="Top Of Library"
              positionKey={keyForPosition({ cardOrderLocation: 'topOfLibrary' })}
              positionMap={map}
              omitIfEmpty={true}
              dropTarget={
                view.cardDragTargets.includes('topOfLibrary') ? 'topOfLibrary' : undefined
              }
            />
            <LinearCardDisplay
              key="vp"
              name="Viewer Permanents"
              positionKey={keyForPosition({ battlefield: ['viewer', 'permanents'] })}
              positionMap={map}
            />
            <LinearCardDisplay
              key="vm"
              name="Viewer Mana"
              positionKey={keyForPosition({ battlefield: ['viewer', 'mana'] })}
              positionMap={map}
            />
            <LinearCardDisplay
              key="vh"
              name="Viewer Hand"
              positionKey={keyForPosition({ hand: 'viewer' })}
              positionMap={map}
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
      </DndContext>
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
