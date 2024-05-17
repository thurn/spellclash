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
  Dispatch,
  ReactNode,
  SetStateAction,
  createContext,
  useEffect,
  useState,
} from "react";
import { GameResponse, SceneName } from "./display_types";
import MainMenu from "./MainMenu";
import { Game } from "./game_view/Game";
import { connect } from "./server";

function defaultGameResponse(): GameResponse {
  return {
    scene: SceneName.Loading,
    commands: [],
    client_data: {},
  };
}

function defaultGlobalContext(): GlobalContextType {
  return {} as GlobalContextType;
}


export interface GlobalContextType {
  readonly response: GameResponse;
  readonly setState: Dispatch<SetStateAction<GameResponse>>;
}

export const GlobalContext: React.Context<GlobalContextType> = createContext(
  defaultGlobalContext()
);

export function App(): ReactNode {
  const [globalState, setGlobalState] = useState(defaultGameResponse());
  useEffect(() => {
    connect(setGlobalState);
  }, []);
  console.log("Global state scene is " + globalState.scene);
  let scene;
  switch (globalState.scene) {
    case SceneName.MainMenu: {
      scene = <MainMenu view={globalState.commands.at(-1)!.UpdateMainMenuView!}  />;
      break;
    }
    case SceneName.Game: {
      scene = <Game view={globalState.commands.at(-1)!.UpdateGameView!.view} />;
      break;
    }
    case SceneName.Loading: {
      scene = <h1>Loading...</h1>;
      break;
    }
  }
  return (
    <GlobalContext.Provider
      value={{
        response: globalState,
        setState: setGlobalState,
      }}
    >
      {scene}
    </GlobalContext.Provider>
  );
}
