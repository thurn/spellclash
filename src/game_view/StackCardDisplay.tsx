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
import { Card } from './Card';
import { CardMap, PositionKey } from './PlayArea';
import { useContext } from 'react';
import { ItemsContext } from '../draggables/DragManager';

export interface Props {
  readonly positionKey: PositionKey;
  readonly cardMap: CardMap;
}

export function StackCardDisplay({ positionKey, cardMap }: Props): ReactNode {
  const items = useContext(ItemsContext);
  const cardIds = items[positionKey] ?? [];
  const cardViews = cardIds.map((cardId) => (
    <div className="absolute" key={cardId}>
      <Card key={cardId} cardId={cardId} map={cardMap} />
    </div>
  ));
  return (
    <div
      className="bg-slate-300 m-1 rounded"
      style={{
        height: '13.5vh',
      }}
    >
      {cardViews}
    </div>
  );
}
