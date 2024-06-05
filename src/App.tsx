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

import { ReactNode, createContext, useEffect, useState } from 'react';
import { ClientData, GameMessage, GameResponse, ModalPanel, SceneView } from './generated_types';
import MainMenu from './MainMenu';
import { Game } from './game_view/Game';
import { connect, handleAction } from './server';
import { DebugPanelContent } from './panels/DebugPanelContent';
import { Modal, ModalBody, ModalContent, ModalHeader, useDisclosure } from '@nextui-org/react';
import { Event, listen } from '@tauri-apps/api/event';

export const GlobalContext: React.Context<ClientData> = createContext({
  userId: '',
  scene: 'loading',
  displayState: {},
} as ClientData);

export function App(): ReactNode {
  const [clientData, setClientData] = useState({} as ClientData);
  const [sceneView, setSceneView] = useState('loading' as SceneView);
  const [modalPanel, setModalPanel] = useState(null as ModalPanel | null);
  const [gameMessage, setGameMessage] = useState(null as GameMessage | null);

  useEffect(() => {
    connect();
  }, []);

  useEffect(() => {
    const unlisten = listen('game_response', (e: Event<GameResponse>) => {
      const state = e.payload;
      setClientData(state.clientData);
      if ('updateScene' in state.command) {
        setSceneView(state.command.updateScene);
      } else if ('setModalPanel' in state.command) {
        setModalPanel(state.command.setModalPanel);
      } else if ('displayGameMessage' in state.command) {
        setGameMessage(state.command.displayGameMessage.message);
      }
    });
    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  let scene;
  if (sceneView === 'loading') {
    scene = <h1>Loading...</h1>;
  } else if ('gameView' in sceneView) {
    scene = <Game view={sceneView.gameView} />;
  } else if ('mainMenuView' in sceneView) {
    scene = <MainMenu view={sceneView.mainMenuView} />;
  }

  let message;
  if (gameMessage != null) {
    const text = {
      yourTurn: 'Your Turn',
      opponentTurn: 'Opponent Turn',
      victory: 'Victory!',
      defeat: 'Defeat',
    };
    message = (
      <Modal isOpen={true} hideCloseButton={true}>
        <ModalContent>
          <ModalBody>{text[gameMessage]}</ModalBody>
        </ModalContent>
      </Modal>
    );
  }

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
          handleAction(clientData, onCloseModal);
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
    <GlobalContext.Provider value={clientData}>
      {scene}
      {modal}
      {message}
    </GlobalContext.Provider>
  );
}
