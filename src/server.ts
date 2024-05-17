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

import { invoke } from "@tauri-apps/api/core";
import { ClientData, GameResponse, UserAction } from "./display_types";
import { Dispatch, SetStateAction } from "react";

export async function connect(
  setter: Dispatch<SetStateAction<GameResponse>>
): Promise<void> {
  console.log("Connecting...");
  const result: GameResponse = await invoke("client_connect", {});
  console.log("Connected!");
  console.dir(result);
  setter(result);
}

export async function handleAction(
  setter: Dispatch<SetStateAction<GameResponse>>,
  clientData: ClientData,
  action?: UserAction
): Promise<void> {
  if (action == null) {
    return;
  }

  console.log("Handling action...");
  console.dir(action);
  const result: GameResponse = await invoke("client_handle_action", {
    clientData,
    action,
  });
  console.log("Got action response");
  console.dir(result);
  setter(result);
}
