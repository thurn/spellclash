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

import {
  CardOrderLocation,
  ClientCardId,
  ClientData,
  FieldKey,
  FieldValue,
  commands,
} from './generated_types';

export async function connect(): Promise<void> {
  console.log('Connecting...');
  await commands.connect();
}

export async function handleAction(clientData: ClientData, action?: unknown): Promise<void> {
  if (action == null) {
    return;
  }
  console.log('Handling action...');
  console.dir(action);
  await commands.handleAction(clientData, action);
}

export async function updateField(
  clientData: ClientData,
  key: FieldKey,
  value: FieldValue,
): Promise<void> {
  await commands.updateField(clientData, key, value);
}

export async function dragCard(
  clientData: ClientData,
  cardId: ClientCardId,
  location: CardOrderLocation,
  index: number,
): Promise<void> {
  console.log(
    'Handling drag card action for cardId:',
    cardId,
    'location:',
    location,
    'index:',
    index,
  );
  await commands.dragCard(clientData, cardId, location, index);
}
