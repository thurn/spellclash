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
import { PlayArea } from './PlayArea';
import { GameInfo } from './GameInfo';
import { GameView } from '../generated_types';
import { DragManager, Items, ItemsContext } from '../draggables/DragManager';
import { DropTargetContainer } from '../draggables/DropTargetContainer';
import { DraggableItem } from '../draggables/DraggableItem';

export function Game({ view }: { view: GameView }): ReactNode {
  const itemCount = 3;
  const initialItems: Items = {
    A: createRange(itemCount, (index) => `A${index + 1}`),
    B: createRange(itemCount, (index) => `B${index + 1}`),
    C: createRange(itemCount, (index) => `C${index + 1}`),
    D: createRange(itemCount, (index) => `D${index + 1}`),
  };

  return (
    <div className="w-screen h-screen flex flex-row">
      <DragManager initialItems={initialItems}>
        <DragChildren />
      </DragManager>
    </div>
  );
}

function DragChildren(): ReactNode {
  const items = useContext(ItemsContext);
  const containerIds = Object.keys(items);

  return (
    <div
      style={{
        display: 'inline-grid',
        boxSizing: 'border-box',
        padding: 20,
        gridAutoFlow: 'row',
      }}
    >
      {containerIds.map((containerId) => (
        <DropTargetContainer key={containerId} id={containerId} items={items[containerId]}>
          {items[containerId].map((id, index) => {
            return (
              <DraggableItem key={id} id={id} index={index}>
                {id}
              </DraggableItem>
            );
          })}
        </DropTargetContainer>
      ))}
    </div>
  );
}

// export function Game({ view }: { view: GameView }): ReactNode {
//   return (
//     <div className="w-screen h-screen flex flex-row">
//       <div className="flex flex-col w-5/6">
//         <PlayArea view={view} />
//       </div>
//       <div className="flex flex-col justify-between w-1/6">
//         <GameInfo view={view} />
//       </div>
//     </div>
//   );
// }

const defaultInitializer = (index: number) => index;

function createRange<T = number>(
  length: number,
  initializer: (index: number) => any = defaultInitializer,
): T[] {
  return [...new Array(length)].map((_, index) => initializer(index));
}
