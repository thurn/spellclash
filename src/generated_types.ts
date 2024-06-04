// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

export const commands = {
  async clientConnect(): Promise<void> {
    await TAURI_INVOKE('client_connect');
  },
  async clientHandleAction(clientData: ClientData, action: unknown): Promise<void> {
    await TAURI_INVOKE('client_handle_action', { clientData, action });
  },
  async clientUpdateField(clientData: ClientData, key: FieldKey, value: FieldValue): Promise<void> {
    await TAURI_INVOKE('client_update_field', { clientData, key, value });
  },
};

export const events = __makeEvents__<{
  gameResponseEvent: GameResponseEvent;
}>({
  gameResponseEvent: 'game-response-event',
});

/** user-defined types **/

/**
 * Sub-positions for objects within the battlefield.
 */
export type BattlefieldPosition = 'mana' | 'permanents';
/**
 * Facing for this card, corresponding to the [PrintedCard] faces.
 */
export type CardFacing =
  | 'faceDown'
  /**
   * The indicated card face is currently up
   */
  | { faceUp: Face };
/**
 * Describes how the multiple faces of a card are organized in relation to each
 * other.
 *
 * See <https://scryfall.com/docs/api/layouts>
 */
export type CardLayout =
  | 'adventure'
  | 'aftermath'
  | 'battle'
  | 'flip'
  | 'meld'
  | 'modal_dfc'
  | 'normal'
  | 'split'
  | 'transform';
/**
 * Represents the visual state of a card or ability in a game
 */
export type CardView = {
  /**
   * Identifier for this card
   */
  id: ClientCardId;
  /**
   * Position of this card in the UI
   */
  position: ObjectPosition;
  /**
   * Card back image
   */
  cardBack: string;
  /**
   * If this card is revealed to the viewer, contains information on the
   * revealed face of the card.
   */
  revealed: RevealedCardView | null;
  /**
   * True if this card is in a hidden zone but known to one or more opponents
   */
  revealedToOpponents: boolean;
  /**
   * Face up/face down state for this card
   */
  cardFacing: CardFacing;
  /**
   * Tapped/untapped state for this card
   */
  tappedState: TappedState;
  /**
   * Damage marked on this card
   *
   * Note that the rules engine internally uses 64-bit integers, but in the
   * display layer we use floats for JavaScript compatibility.
   */
  damage: number;
  /**
   * Optionally, a position at which to create this card.
   *
   * If this card does not already exist, it will be created at this position
   * before being animated to [Self::position].
   */
  createPosition: ObjectPosition | null;
  /**
   * Optionally, a position at which to destroy this card.
   *
   * If provided, the card will be animated to this position before being
   * destroyed.
   */
  destroyPosition: ObjectPosition | null;
};
/**
 * Identifies a card in client code
 *
 * Serialized u64, represented as string because JavaScript is a silly
 * language.
 */
export type ClientCardId = string;
/**
 * Standard parameters for a client request & response
 */
export type ClientData = {
  /**
   * User who is currently connected
   */
  userId: UserId;
  /**
   * Currently-displayed top level screen
   */
  scene: SceneIdentifier;
  /**
   * Options for how the game state should be visually rendered
   */
  displayState: unknown;
};
/**
 * Represents an instruction to the client to perform some visual update.
 */
export type Command =
  /**
   * Update the primary visual state of the game.
   */
  | { updateScene: SceneView }
  /**
   * Hide or show a modal panel on top of the scene view.
   */
  | { setModalPanel: ModalPanel | null }
  /**
   * Display a message to the player.
   */
  | { displayGameMessage: DisplayGameMessageCommand };
/**
 * Debug options
 */
export type DebugPanel = { buttons: GameButtonView[] };
export type DisplayGameMessageCommand = {
  /**
   * Top-level status message to display to the player
   */
  message: GameMessage;
};
/**
 * Identifies a player in the context of the user interface.
 */
export type DisplayPlayer =
  /**
   * Player who is currently operating the client
   */
  | 'viewer'
  /**
   * Opponent of viewer
   */
  | 'opponent';
export type Face = 'primary' | 'faceB';
/**
 * Describes how a single face of a card is laid out.
 *
 * See <https://scryfall.com/docs/api/layouts>
 */
export type FaceLayout =
  | 'adventure'
  | 'aftermath'
  | 'art_series'
  | 'augment'
  | 'battle'
  | 'case'
  | 'class'
  | 'double_faced_token'
  | 'emblem'
  | 'flip'
  | 'host'
  | 'leveler'
  | 'meld'
  | 'modal_dfc'
  | 'mutate'
  | 'normal'
  | 'planar'
  | 'prototype'
  | 'reversible_card'
  | 'saga'
  | 'scheme'
  | 'split'
  | 'token'
  | 'transform'
  | 'vanguard';
export type FieldKey = 'pickNumberPrompt';
export type FieldValue = { string: string };
/**
 * Controls color for buttons
 */
export type GameButtonKind =
  /**
   * Emphasized button, primary game action
   */
  | 'primary'
  /**
   * Deemphasized button, additional game actions
   */
  | 'default';
export type GameButtonView = { label: string; action: unknown; kind: GameButtonKind };
export type GameControlView = { button: GameButtonView } | { textInput: TextInputView };
/**
 * Unique identifier for a game
 */
export type GameId = string;
export type GameMessage = 'yourTurn' | 'opponentTurn' | 'victory' | 'defeat';
/**
 * A response to a user request.
 */
export type GameResponse = {
  /**
   * Current context, must be returned to server with all future requests
   */
  clientData: ClientData;
  /**
   * Update to visual game state
   */
  command: Command;
};
export type GameResponseEvent = GameResponse;
/**
 * Represents the visual state of an ongoing game
 */
export type GameView = {
  /**
   * Player who is operating the client
   */
  viewer: PlayerView;
  /**
   * Opponent of viewer
   */
  opponent: PlayerView;
  /**
   * Visual state of cards in the game
   */
  cards: CardView[];
  /**
   * Describes the status of the game, e.g. which phase & step the game is in
   */
  statusDescription: string;
  /**
   * High level visual game state
   */
  state: GameViewState;
  /**
   * Top user interaction options
   */
  topControls: GameControlView[];
  /**
   * Bottom user interaction options
   */
  bottomControls: GameControlView[];
};
export type GameViewState =
  | 'none'
  /**
   * There is an ongoing combat phase
   */
  | 'combatActive';
/**
 * Represents the visual state of the main menu
 */
export type MainMenuView = {
  /**
   * Primary buttons to show
   */
  buttons: GameButtonView[];
};
/**
 * Rendering options for a modal window which can be displayed on top of other
 * game content
 */
export type ModalPanel = { title: string | null; on_close: unknown; data: PanelData };
/**
 * Represents the position of some object in the UI
 */
export type ObjectPosition = {
  /**
   * Position category
   */
  position: Position;
  /**
   * Sorting key, determines order within the position
   */
  sortingKey: number;
  /**
   * Sub-key, used to break ties in sorting
   */
  sortingSubKey: number;
};
/**
 * Types of content which can appear in a modal panel
 */
export type PanelData = { Debug: DebugPanel };
/**
 * Represents the visual state of a player in a game
 */
export type PlayerView = {
  /**
   * Current life total
   *
   * Note that the rules engine internally uses 64-bit integers, but in the
   * display layer we use floats for JavaScript compatibility.
   */
  life: number;
  /**
   * Can this player currently take a game action?
   */
  canAct: boolean;
};
/**
 * Possible types of display positions
 */
export type Position =
  /**
   * Object position used in interface elements like the deck viewer which
   * don't rely on game positioning.
   */
  | 'default'
  /**
   * Object is not visible.
   */
  | 'offscreen'
  /**
   * Object is prominently revealed, being shown at a large size after
   * being played.
   */
  | 'played'
  /**
   * Object is on the stack
   */
  | 'stack'
  /**
   * Object is in this player's hand
   */
  | { hand: DisplayPlayer }
  /**
   * Object is in this player's deck
   */
  | { deck: DisplayPlayer }
  /**
   * Object is in this player's discard pile
   */
  | { discardPile: DisplayPlayer }
  /**
   * Object is in this player's exile zone
   */
  | { exile: DisplayPlayer }
  /**
   * Object is in this player's command zone
   */
  | { commandZone: DisplayPlayer }
  /**
   * Object is controlled by this player in a given battlefield position
   */
  | { battlefield: [DisplayPlayer, BattlefieldPosition] }
  /**
   * Object is in attack position for this player
   */
  | { attacking: DisplayPlayer }
  /**
   * Object is controlled by this player and is blocking the provided set of
   * attackers
   */
  | { blocking: [DisplayPlayer, ClientCardId[]] }
  /**
   * Object is being displayed in a card browser, e.g. to select from a list
   * of cards
   */
  | 'browser'
  /**
   * Object has just been revealed to this viewer
   */
  | 'revealed'
  /**
   * Object is in a temporary holding space for cards in hand while resolving
   * some other 'play card' ability.
   */
  | 'handStorage'
  /**
   * Object is not visible because it is inside the indicated card.
   */
  | { insideCard: ClientCardId }
  /**
   * Object is attached to the indicated card.
   */
  | { attachedToCard: ClientCardId };
/**
 * Visual state of a revealed card face
 */
export type RevealedCardFace = {
  /**
   * Name of this face
   */
  name: string;
  /**
   * Visual style of specifically this face
   */
  layout: FaceLayout;
  /**
   * Rules text_strings for this face, if any.
   */
  rulesText: string | null;
};
export type RevealedCardStatus = 'canPlay' | { attacking: string } | { blocking: string };
/**
 * Visual state of a revealed card
 */
export type RevealedCardView = {
  /**
   * Image URL for this card
   *
   * For double-faced cards, this is the image of the face which is currently
   * face-up. For other kinds of multi-faced cards, this is an image
   * containing both faces.
   */
  image: string;
  /**
   * Primary face of this card
   */
  face: RevealedCardFace;
  /**
   * Visual status of this card
   */
  status: RevealedCardStatus | null;
  /**
   * Action to take when this card is clicked, if any.
   */
  clickAction: unknown | null;
  /**
   * Secondary or additional face of this card, if any
   */
  faceB: RevealedCardFace | null;
  /**
   * Visual style of this card, how the faces are displayed
   */
  layout: CardLayout;
};
/**
 * Top-level states the user interface can be in.
 *
 * This is used in a similar way to a browser url for the game.
 */
export type SceneIdentifier = 'loading' | 'mainMenu' | { game: GameId };
export type SceneView = 'loading' | { gameView: GameView } | { mainMenuView: MainMenuView };
/**
 * Whether a card is tapped or untapped.
 *
 * I assume within 10 years WoTC will introduce a third tapped state somehow,
 * so might as well make this an enum.
 */
export type TappedState = 'untapped' | 'tapped';
/**
 * Data to render a text input field
 */
export type TextInputView = {
  /**
   * Unique identifier for this field
   */
  key: FieldKey;
};
/**
 * Unique identifier for a user
 *
 * A 'user' is an operator of this software outside of the context of any game.
 * A 'player' is a participate within a game who may or may not be a user.
 */
export type UserId = string;

/** tauri-specta globals **/

import { invoke as TAURI_INVOKE } from '@tauri-apps/api/core';
import * as TAURI_API_EVENT from '@tauri-apps/api/event';
import { type WebviewWindow as __WebviewWindow__ } from '@tauri-apps/api/webviewWindow';

type __EventObj__<T> = {
  listen: (cb: TAURI_API_EVENT.EventCallback<T>) => ReturnType<typeof TAURI_API_EVENT.listen<T>>;
  once: (cb: TAURI_API_EVENT.EventCallback<T>) => ReturnType<typeof TAURI_API_EVENT.once<T>>;
  emit: T extends null
    ? (payload?: T) => ReturnType<typeof TAURI_API_EVENT.emit>
    : (payload: T) => ReturnType<typeof TAURI_API_EVENT.emit>;
};

export type Result<T, E> = { status: 'ok'; data: T } | { status: 'error'; error: E };

function __makeEvents__<T extends Record<string, any>>(mappings: Record<keyof T, string>) {
  return new Proxy(
    {} as unknown as {
      [K in keyof T]: __EventObj__<T[K]> & {
        (handle: __WebviewWindow__): __EventObj__<T[K]>;
      };
    },
    {
      get: (_, event) => {
        const name = mappings[event as keyof T];

        return new Proxy((() => {}) as any, {
          apply: (_, __, [window]: [__WebviewWindow__]) => ({
            listen: (arg: any) => window.listen(name, arg),
            once: (arg: any) => window.once(name, arg),
            emit: (arg: any) => window.emit(name, arg),
          }),
          get: (_, command: keyof __EventObj__<any>) => {
            switch (command) {
              case 'listen':
                return (arg: any) => TAURI_API_EVENT.listen(name, arg);
              case 'once':
                return (arg: any) => TAURI_API_EVENT.once(name, arg);
              case 'emit':
                return (arg: any) => TAURI_API_EVENT.emit(name, arg);
            }
          },
        });
      },
    },
  );
}
