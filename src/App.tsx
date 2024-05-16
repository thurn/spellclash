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

import { Dispatch, SetStateAction, createContext, useState } from "react";
import { GameResponse, SceneName } from "./display_types";
import MainMenu from "./MainMenu";

export interface GlobalContextType {
  readonly response?: GameResponse;
  readonly setState?: Dispatch<SetStateAction<GameResponse>>;
}

function defaultGameResponse(): GameResponse {
  return {
    scene: SceneName.MainMenu,
    commands: [],
    client_data: {},
  };
}

function defaultGlobalContext(): GlobalContextType {
  return {} as GlobalContextType;
}

export const GlobalContext: React.Context<GlobalContextType> = createContext(
  defaultGlobalContext()
);

export function App() {
  const [globalState, setGlobalState] = useState(defaultGameResponse());
  console.log("Global state scene is " + globalState.scene);
  return (
    <GlobalContext.Provider
      value={{
        response: globalState,
        setState: setGlobalState,
      }}
    >
      <MainMenu />
    </GlobalContext.Provider>
  );
}
