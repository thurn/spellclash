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
import { GameResponse, Result, commands } from './generated_types';

export async function connect(setter: Dispatch<SetStateAction<GameResponse>>): Promise<void> {
  console.log('Connecting...');
  const result: Result<GameResponse, null> = await commands.clientConnect();
  if (result.status === "ok") {
    console.log('Connected!');
    console.dir(result.data);
    setter(result.data);
  } else {
    console.error('Connect error!');
  }
}

export async function handleAction(
  setter: Dispatch<SetStateAction<GameResponse>>,
  lastResponse: GameResponse,
  action?: unknown,
): Promise<void> {
  if (action == null) {
    return;
  }

  console.log('Handling action...');
  console.dir(action);
  let result: Result<GameResponse, null> = await commands.clientHandleAction(lastResponse.client_data, action);
  if (result.status === "ok") {
    let data = result.data;
    if (data.commands.length === 0) {
      // Propagate previous command state if no UI update provided
      data = {
        modal_panel: data.modal_panel,
        commands: lastResponse.commands,
        client_data: data.client_data,
        opponent_responses: data.opponent_responses
      };
    }
    console.log('Got action response');
    console.dir(data);
    setter(data);
  } else {
    console.error('Error handling action!', action);
  }
}
