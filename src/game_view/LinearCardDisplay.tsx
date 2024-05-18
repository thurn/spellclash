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
import { CardView } from '../display_types';
import { Card } from './Card';

export function LinearCardDisplay({ name, cards }: { name: string; cards: CardView[] }): ReactNode {
  const cardViews = cards.map((card, i) => <Card card={card} key={i} />);
  return (
    <div
      className="bg-slate-300 m-1 rounded flex flex-row items-center"
      style={{
        height: '13.5vh',
      }}
    >
      {cardViews}
      <div className="w-32 text-center text-sm">{name}</div>
    </div>
  );
}