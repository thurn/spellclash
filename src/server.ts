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

import { Dispatch, SetStateAction } from 'react';
import { FieldKey, FieldValue, GameResponse, Result, commands } from './generated_types';

export async function connect(setter: Dispatch<SetStateAction<GameResponse>>): Promise<void> {
  console.log('Connecting...');
  const result: Result<GameResponse, null> = await commands.clientConnect();
  if (result.status === 'ok') {
    console.log('Connected!');
    console.dir(result.data);
    setter(result.data);
  } else {
    console.error('Connect error!');
  }
}

export async function handleAction(lastResponse: GameResponse, action?: unknown): Promise<void> {
  if (action == null) {
    return;
  }
  console.log('Handling action...');
  console.dir(action);
  await commands.clientHandleAction(lastResponse.clientData, action);
}

export async function updateField(
  setter: Dispatch<SetStateAction<GameResponse>>,
  lastResponse: GameResponse,
  key: FieldKey,
  value: FieldValue,
): Promise<void> {
  const result: Result<GameResponse, null> = await commands.clientUpdateField(
    lastResponse.clientData,
    key,
    value,
  );
  if (result.status === 'ok') {
    let data = result.data;
    if (data.commands.length === 0) {
      data = {
        commands: lastResponse.commands,
        clientData: data.clientData,
      };
    }
    setter(data);
  } else {
    console.error('Error updating field', key, value);
  }
}
