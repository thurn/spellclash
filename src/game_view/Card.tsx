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
import { ClientCardId } from '../generated_types';
import { HiddenCard } from './HiddenCard';
import { RevealedCard } from './RevealedCard';
import { CardMap } from './PlayArea';

interface Props {
  readonly cardId: ClientCardId;
  readonly map: CardMap;
}

export function Card({ cardId, map }: Props): ReactNode {
  const card = map.cards.get(cardId);
  if (card == null) {
    throw new Error(`Card not found: ${cardId}`);
  }

  const height = 13.5;
  let body;
  if (card.revealed != null) {
    body = <RevealedCard revealed={card.revealed} />;
  } else {
    body = <HiddenCard card={card} />;
  }
  return (
    <div
      style={{
        height: `${height}vh`,
        width: `${height * (5 / 7)}vh`,
        margin: '0.1vh',
        transform: card.tappedState === 'tapped' ? `rotate(90deg)` : undefined,
      }}
    >
      {body}
    </div>
  );
}
