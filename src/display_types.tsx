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

export interface GameResponse {
  readonly scene: SceneName;
  readonly commands: Command[];
  readonly client_data: ClientData;
}

export enum SceneName {
  MainMenu = "MainMenu",
  Game = "Game",
}

export interface ClientData {}

export interface Command {
  readonly UpdateGameView?: UpdateGameView;
}

export interface UpdateGameView {
  readonly view: GameView;
  readonly animate: boolean;
}

export interface GameView {
  readonly viewer: PlayerView;
  readonly opponent: PlayerView;
  readonly cards: CardView[];
  readonly status_description: string;
  readonly state: GameViewState;
  readonly top_buttons: GameButton[];
  readonly bottom_buttons: GameButton[];
}

export enum GameViewState {
  None = "None",
  CombatActive = "CombatActive",
}

export interface GameButton {
  readonly label: string;
  readonly action: UserAction;
}

export interface UserAction {}

export enum DisplayPlayer {
  Viewer = "Viewer",
  Opponent = "Opponent",
}

export interface PlayerView {
  readonly life: number;
  readonly can_act: boolean;
}

export interface CardView {
  readonly id: CardId;
  readonly position: ObjectPosition;
  readonly card_back: string;
  readonly revealed?: RevealedCardView;
  readonly revealed_to_opponents: boolean;
  readonly tapped_state: TappedState;
  readonly card_facing: CardFacing;
  readonly damage: number;
  readonly create_position?: ObjectPosition;
  readonly destroy_position?: ObjectPosition;
}

export interface CardId {}

export type CardFacing = "FaceDown" | FaceUpFacing;

export interface FaceUpFacing {
  readonly FaceUp: Face;
}

export enum Face {
  Primary = "Primary",
  FaceB = "FaceB",
}

export enum TappedState {
  Untapped = "Untapped",
  Tapped = "Tapped",
}

export interface RevealedCardView {
  readonly face: RevealedCardFace;
  readonly status: RevealedCardStatus;
  readonly click_action?: UserAction;
  readonly face_b?: RevealedCardFace;
  readonly layout: CardLayout;
}

export interface RevealedCardStatus {
  readonly CanPlay?: {};
  readonly Attacking?: string;
  readonly Blocking?: string;
}

export type CardLayout = string;

export interface RevealedCardFace {
  readonly name: string;
  readonly image: string;
  readonly layout: FaceLayout;
  readonly rules_text?: string;
}

export type FaceLayout = string;

export interface ObjectPosition {
  position: Position;
  sorting_key: number;
  sorting_sub_key: number;
}

export interface Position {
  readonly Stack?: {};
  readonly Hand?: DisplayPlayer;
  readonly Deck?: DisplayPlayer;
  readonly DiscardPile?: DisplayPlayer;
  readonly Exile?: DisplayPlayer;
  readonly CommandZone?: DisplayPlayer;
  readonly Battlefield?: DisplayPlayer;
}