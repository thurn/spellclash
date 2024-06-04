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
import { Event, listen } from '@tauri-apps/api/event';

function defaultGameResponse(): GameResponse {
  return {
    commands: [],
    clientData: {
      userId: '',
      scene: 'loading',
      modalPanel: null,
      displayState: {},
    },
  };
}

function defaultGlobalContext(): GlobalContextType {
  return {} as GlobalContextType;
}

export interface GlobalContextType {
  readonly response: GameResponse;
  readonly setState: Dispatch<SetStateAction<GameResponse>>;
}

export const GlobalContext: React.Context<GlobalContextType> =
  createContext(defaultGlobalContext());

export function App(): ReactNode {
  const [globalState, setGlobalState] = useState(defaultGameResponse());
  useEffect(() => {
    connect(setGlobalState);
  }, []);
  useEffect(() => {
    const unlisten = listen('game_response', (e: Event<GameResponse>) => {
      console.log('Got game response');
      setGlobalState(e.payload);
    });
    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  let scene = <h1>Loading...</h1>;
  let gameMessage = null;
  for (const command of globalState.commands) {
    if ('updateMainMenuView' in command) {
      scene = <MainMenu view={command.updateMainMenuView} />;
    } else if ('updateGameView' in command) {
      scene = <Game view={command.updateGameView!.view} />;
    } else if ('displayGameMessage' in command) {
      const text = {
        yourTurn: 'Your Turn',
        opponentTurn: 'Opponent Turn',
        victory: 'Victory!',
        defeat: 'Defeat',
      };
      gameMessage = (
        <Modal isOpen={true} hideCloseButton={true}>
          <ModalContent>
            <ModalBody>{text[command.displayGameMessage.message]}</ModalBody>
          </ModalContent>
        </Modal>
      );
    }
  }

  const modalPanel = globalState.clientData.modalPanel;
  const { isOpen, onOpenChange } = useDisclosure({ isOpen: modalPanel != null });
  let modal;
  if (modalPanel != null) {
    let modalContent;
    const onCloseModal = modalPanel.on_close;
    if ('Debug' in modalPanel.data) {
      modalContent = <DebugPanelContent data={modalPanel.data.Debug} />;
    }

    modal = (
      <Modal
        isOpen={isOpen}
        onOpenChange={onOpenChange}
        onClose={() => {
          handleAction(globalState, onCloseModal);
        }}
      >
        <ModalContent>
          <ModalHeader>{modalPanel.title ?? ''}</ModalHeader>
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
      {gameMessage}
    </GlobalContext.Provider>
  );
}
