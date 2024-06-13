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

import { ReactNode, useContext } from 'react';
import { CardOrderLocation } from '../generated_types';
import { Card } from './Card';
import { CardMap, PositionKey } from './PlayArea';
import { DropTargetContainer } from '../draggables/DropTargetContainer';
import { ItemsContext } from '../draggables/DragManager';
import { DraggableItem } from '../draggables/DraggableItem';

export interface Props {
  readonly name: string;
  readonly positionKey: PositionKey;
  readonly cardMap: CardMap;
  readonly dropTarget?: CardOrderLocation;
  readonly omitIfEmpty?: boolean;
}

export function LinearCardDisplay({
  name,
  positionKey,
  cardMap,
  dropTarget,
  omitIfEmpty = false,
}: Props): ReactNode {
  const items = useContext(ItemsContext);
  const cardIds = items[positionKey] ?? [];
  const isDropTarget = dropTarget != null;
  if (cardIds.length === 0 && omitIfEmpty && !isDropTarget) {
    return null;
  }

  let background;
  if (isDropTarget) {
    background = 'bg-green-600';
  } else {
    background = 'bg-slate-300';
  }

  const className = `${background} m-1 rounded flex grow flex-row items-center`;

  let content;
  if (isDropTarget) {
    content = (
      <DropTargetContainer key={positionKey} id={positionKey} items={cardIds} height="14vh">
        {cardIds.map((id, index) => {
          return (
            <DraggableItem key={id} id={id} index={index}>
              <Card key={id} cardId={id} map={cardMap} />
            </DraggableItem>
          );
        })}
      </DropTargetContainer>
    );
  } else {
    content = cardIds.map((cardId) => <Card key={cardId} cardId={cardId} map={cardMap} />);
  }

  return (
    <div
      className={className}
      style={{
        height: '14vh',
      }}
    >
      {content}
      <div className="w-32 text-center text-sm">{name}</div>
    </div>
  );
}
