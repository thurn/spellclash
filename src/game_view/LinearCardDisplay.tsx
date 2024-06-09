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
import { CardOrderLocation, CardView } from '../generated_types';
import { Card } from './Card';
import { useDroppable } from '@dnd-kit/core';
import { PositionKey, PositionMap } from './PlayArea';

export interface Props {
  readonly name: string;
  readonly positionKey: PositionKey;
  readonly positionMap: PositionMap;
  readonly dropTarget?: CardOrderLocation;
  readonly omitIfEmpty?: boolean;
}

export function LinearCardDisplay({
  name,
  positionKey,
  positionMap,
  dropTarget,
  omitIfEmpty = false,
}: Props): ReactNode {
  const { isOver, setNodeRef } = useDroppable({
    id: positionKey,
    data: { dropTarget },
  });
  const cards = getPosition(positionMap, positionKey);
  const isDropTarget = dropTarget != null;
  const ref = isDropTarget ? setNodeRef : undefined;

  let background;
  if (isOver && isDropTarget) {
    background = 'bg-green-300';
  } else if (isDropTarget) {
    background = 'bg-green-600';
  } else {
    background = 'bg-slate-300';
  }

  if (cards.length === 0 && omitIfEmpty && !isDropTarget) {
    return null;
  }

  const cardViews = cards.map((card, i) => <Card card={card} key={i} />);
  const className = `${background} m-1 rounded flex flex-row items-center`;
  return (
    <div
      ref={ref}
      className={className}
      style={{
        height: '13.5vh',
      }}
    >
      {cardViews}
      <div className="w-32 text-center text-sm">{name}</div>
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
