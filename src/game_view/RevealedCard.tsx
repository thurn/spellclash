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
import { RevealedCardView } from '../generated_types';
import { GlobalContext } from '../App';
import { handleAction } from '../server';

export interface Props {
  readonly revealed: RevealedCardView;
}

export function RevealedCard({ revealed }: Props): ReactNode {
  const clientData = useContext(GlobalContext);

  let borderClass = 'border-2 border-black';
  let label = '';
  if (revealed.status === 'canPlay') {
    borderClass = 'border-2 border-amber-300';
  } else if (revealed.status === 'selected') {
    borderClass = 'border-2 border-lime-300';
  } else if (revealed.status === 'canSelect') {
    borderClass = 'border-2 border-orange-600';
  } else if (revealed.status != null && 'attacking' in revealed.status) {
    borderClass = 'border-2 border-teal-300';
    label = revealed.status.attacking;
  } else if (revealed.status != null && 'alocking' in revealed.status) {
    borderClass = 'border-2 border-purple-300';
    label = revealed.status.blocking;
  } else if (revealed.isAbility) {
    borderClass = 'border-2 border-pink-300';
    label = 'ability';
  } else if (revealed.isToken) {
    label = 'token';
  }

  if (revealed.canDrag) {
    return (
      <div className={borderClass}>
        <img
          src={revealed.image}
          style={{
            width: '100%',
            height: '100%',
          }}
        />
        <span className="absolute bg-slate-900 text-white text-xs">{label}</span>
      </div>
    );
  } else {
    return (
      <div className={borderClass} onClick={() => handleAction(clientData, revealed.clickAction)}>
        <img
          src={revealed.image}
          style={{
            width: '100%',
            height: '100%',
          }}
        />
        <span className="absolute bg-slate-900 text-white text-xs">{label}</span>
      </div>
    );
  }
}
