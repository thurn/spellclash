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

import { Dispatch, ReactNode, SetStateAction, createContext, useEffect, useState } from 'react';
import { GameResponse } from './generated_types';
import MainMenu from './MainMenu';
import { Game } from './game_view/Game';
import { connect, handleAction } from './server';
import { DebugPanelContent } from './panels/DebugPanelContent';
import { Modal, ModalBody, ModalContent, ModalHeader, useDisclosure } from '@nextui-org/react';

function defaultGameResponse(): GameResponse {
  return {
    commands: [],
    modalPanel: null,
    clientData: {
      userId: "",
      displayPreferences: {},
      scene: 'loading',
      opponentIds: []
    },
    opponentResponses: []
  };
}

function defaultGlobalContext(): GlobalContextType {
  return {} as GlobalContextType;
}

export interface GlobalContextType {
  readonly response: GameResponse;
  readonly setState: Dispatch<SetStateAction<GameResponse>>;
}

export const GlobalContext: React.Context<GlobalContextType> = createContext(defaultGlobalContext());

export function App(): ReactNode {
  const [globalState, setGlobalState] = useState(defaultGameResponse());
  useEffect(() => {
    connect(setGlobalState);
  }, []);
  const sceneIdentifier = globalState.clientData.scene;
  console.log('Global state scene is ' + sceneIdentifier);

  let scene;
  if (sceneIdentifier === 'mainMenu') {
    const command = globalState.commands.at(-1)!;
    if ('updateMainMenuView' in command) {
      scene = <MainMenu view={command.updateMainMenuView} />;
    }
  } else if (sceneIdentifier === 'loading') {
    scene = <h1>Loading...</h1>;
  } else if ('game' in sceneIdentifier) {
    const command = globalState.commands.at(-1)!;
    if ('updateGameView' in command) {
      scene = <Game view={command.updateGameView!.view} />;
    }
  }

  const { isOpen, onOpenChange } = useDisclosure({ isOpen: globalState.modalPanel != null });
  let modal;
  if (globalState.modalPanel != null) {
    let modalContent;
    let onCloseModal = globalState.modalPanel.on_close;
    if ('Debug' in globalState.modalPanel.data) {
      modalContent = <DebugPanelContent data={globalState.modalPanel.data.Debug} />;
    }

    modal = (
      <Modal
        isOpen={isOpen}
        onOpenChange={onOpenChange}
        onClose={() => {
          handleAction(setGlobalState, globalState, onCloseModal);
        }}
      >
        <ModalContent>
          <ModalHeader>{globalState.modalPanel.title ?? ''}</ModalHeader>
          <ModalBody>{modalContent}</ModalBody>
        </ModalContent>
      </Modal>
    );
  }

  return (
    <GlobalContext.Provider
      value={{
        response: globalState,
        setState: setGlobalState,
      }}
    >
      {scene}
      {modal}
    </GlobalContext.Provider>
  );
}
