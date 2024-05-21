         // This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

         export const commands = {
async clientConnect() : Promise<Result<GameResponse, null>> {
try {
    return { status: "ok", data: await TAURI_INVOKE("client_connect") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async clientHandleAction(clientData: ClientData, action: unknown) : Promise<Result<GameResponse, null>> {
try {
    return { status: "ok", data: await TAURI_INVOKE("client_handle_action", { clientData, action }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
}
}



/** user-defined types **/

/**
 * Sub-positions for objects within the battlefield.
 */
export type BattlefieldPosition = "Mana" | "Permanents"
/**
 * Facing for this card, corresponding to the [PrintedCard] faces.
 */
export type CardFacing = "FaceDown" | 
/**
 * The indicated card face is currently up
 */
{ FaceUp: Face }
/**
 * Describes how the multiple faces of a card are organized in relation to each
 * other.
 * 
 * See <https://scryfall.com/docs/api/layouts>
 */
export type CardLayout = "Adventure" | "Aftermath" | "Battle" | "Flip" | "Meld" | "ModalDfc" | "Normal" | "Split" | "Transform"
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
card_back: string; 
/**
 * If this card is revealed to the viewer, contains information on the
 * revealed face of the card.
 */
revealed: RevealedCardView | null; 
/**
 * True if this card is in a hidden zone but known to one or more opponents
 */
revealed_to_opponents: boolean; 
/**
 * Face up/face down state for this card
 */
card_facing: CardFacing; 
/**
 * Tapped/untapped state for this card
 */
tapped_state: TappedState; 
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
create_position: ObjectPosition | null; 
/**
 * Optionally, a position at which to destroy this card.
 * 
 * If provided, the card will be animated to this position before being
 * destroyed.
 */
destroy_position: ObjectPosition | null }
/**
 * Identifies a card in client code
 * 
 * Serialized u64, represented as string because JavaScript is a silly
 * language.
 */
export type ClientCardId = string
/**
 * Standard parameters for a client request & response
 */
export type ClientData = { user_id: UserId; scene: SceneIdentifier; 
/**
 * Options for how the game state should be visually rendered
 */
display_preferences: DisplayPreferences; 
/**
 * Other user who are opponents in this game.
 */
opponent_ids: UserId[] }
/**
 * Represents an instruction to the client to perform some visual update.
 */
export type Command = { UpdateGameView: UpdateGameViewCommand } | { UpdateMainMenuView: MainMenuView }
/**
 * Debug options
 */
export type DebugPanel = { buttons: GameButtonView[] }
/**
 * Identifies a player in the context of the user interface.
 */
export type DisplayPlayer = 
/**
 * Player who is currently operating the client
 */
"Viewer" | 
/**
 * Opponent of viewer
 */
"Opponent"
export type DisplayPreferences = Record<string, never>
export type Face = "Primary" | "FaceB"
/**
 * Describes how a single face of a card is laid out.
 * 
 * See <https://scryfall.com/docs/api/layouts>
 */
export type FaceLayout = "adventure" | "aftermath" | "art_series" | "augment" | "battle" | "case" | "class" | "double_faced_token" | "emblem" | "flip" | "host" | "leveler" | "meld" | "modal_dfc" | "mutate" | "normal" | "planar" | "prototype" | "reversible_card" | "saga" | "scheme" | "split" | "token" | "transform" | "vanguard"
/**
 * Controls color for buttons
 */
export type GameButtonKind = 
/**
 * Emphasized button, primary game action
 */
"Primary" | 
/**
 * Deemphasized button, additional game actions
 */
"Default"
export type GameButtonView = { label: string; action: unknown; kind: GameButtonKind }
/**
 * Unique identifier for a game
 */
export type GameId = string
/**
 * A response to a user request.
 */
export type GameResponse = { 
/**
 * Optionally, a panel to display on top of the primary scene content
 */
modal_panel: ModalPanel | null; 
/**
 * Current context, must be returned to server with all future requests
 */
client_data: ClientData; 
/**
 * Animated updates to game state
 */
commands: Command[]; 
/**
 * Responses to send to other connected players in the game
 */
opponent_responses: ([UserId, Command[]])[] }
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
status_description: string; 
/**
 * High level visual game state
 */
state: GameViewState; 
/**
 * Top user interaction options
 */
top_buttons: GameButtonView[]; 
/**
 * Bottom user interaction options
 */
bottom_buttons: GameButtonView[] }
export type GameViewState = "None" | 
/**
 * There is an ongoing combat phase
 */
"CombatActive"
/**
 * Represents the visual state of the main menu
 */
export type MainMenuView = { 
/**
 * Primary buttons to show
 */
buttons: GameButtonView[] }
/**
 * Rendering options for a modal window which can be displayed on top of other
 * game content
 */
export type ModalPanel = { title: string | null; on_close: unknown; data: PanelData }
/**
 * Represents the position of some object in the UI
 */
export type ObjectPosition = { 
/**
 * Position category
 */
position: Position; 
/**
 * String representation of the [Position], used to simplify client lookup
 * logic.
 */
position_string: string; 
/**
 * Sorting key, determines order within the position
 */
sorting_key: number; 
/**
 * Sub-key, used to break ties in sorting
 */
sorting_sub_key: number }
/**
 * Types of content which can appear in a modal panel
 */
export type PanelData = { Debug: DebugPanel }
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
can_act: boolean }
/**
 * Possible types of display positions
 */
export type Position = 
/**
 * Object position used in interface elements like the deck viewer which
 * don't rely on game positioning.
 */
"Default" | 
/**
 * Object is not visible.
 */
"Offscreen" | 
/**
 * Object is prominently revealed, being shown at a large size after
 * being played.
 */
"Played" | 
/**
 * Object is on the stack
 */
"Stack" | 
/**
 * Object is in this player's hand
 */
{ Hand: DisplayPlayer } | 
/**
 * Object is in this player's deck
 */
{ Deck: DisplayPlayer } | 
/**
 * Object is in this player's discard pile
 */
{ DiscardPile: DisplayPlayer } | 
/**
 * Object is in this player's exile zone
 */
{ Exile: DisplayPlayer } | 
/**
 * Object is in this player's command zone
 */
{ CommandZone: DisplayPlayer } | 
/**
 * Object is controlled by this player in a given battlefield position
 */
{ Battlefield: [DisplayPlayer, BattlefieldPosition] } | 
/**
 * Object is in attack position for this player
 */
{ Attacking: DisplayPlayer } | 
/**
 * Object is controlled by this player and is blocking the provided set of
 * attackers
 */
{ Blocking: [DisplayPlayer, ClientCardId[]] } | 
/**
 * Object is being displayed in a card browser, e.g. to select from a list
 * of cards
 */
"Browser" | 
/**
 * Object has just been revealed to this viewer
 */
"Revealed" | 
/**
 * Object is in a temporary holding space for cards in hand while resolving
 * some other 'play card' ability.
 */
"HandStorage" | 
/**
 * Object is not visible because it is inside the indicated card.
 */
{ InsideCard: ClientCardId } | 
/**
 * Object is attached to the indicated card.
 */
{ AttachedToCard: ClientCardId }
/**
 * Visual state of a revealed card face
 */
export type RevealedCardFace = { 
/**
 * Name of this face
 */
name: string; 
/**
 * Image URL for this card
 */
image: string; 
/**
 * Visual style of specifically this face
 */
layout: FaceLayout; 
/**
 * Rules text_strings for this face, if any.
 */
rules_text: string | null }
export type RevealedCardStatus = "CanPlay" | { Attacking: string } | { Blocking: string }
/**
 * Visual state of a revealed card
 */
export type RevealedCardView = { 
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
click_action: unknown | null; 
/**
 * Secondary or additional face of this card, if any
 */
face_b: RevealedCardFace | null; 
/**
 * Visual style of this card, how the faces are displayed
 */
layout: CardLayout }
/**
 * Top-level states the user interface can be in.
 * 
 * This is used in a similar way to a browser url for the game.
 */
export type SceneIdentifier = "Loading" | "MainMenu" | { Game: GameId }
/**
 * Whether a card is tapped or untapped.
 * 
 * I assume within 10 years WoTC will introduce a third tapped state somehow,
 * so might as well make this an enum.
 */
export type TappedState = "Untapped" | "Tapped"
export type UpdateGameViewCommand = { 
/**
 * New visual game state
 */
view: GameView; 
/**
 * Whether to animate updates to this state
 */
animate: boolean }
/**
 * Unique identifier for a user
 * 
 * A 'user' is an operator of this software outside of the context of any game.
 * A 'player' is a participate within a game who may or may not be a user.
 */
export type UserId = string

/** tauri-specta globals **/

         import { invoke as TAURI_INVOKE } from "@tauri-apps/api/core";
import * as TAURI_API_EVENT from "@tauri-apps/api/event";
import { type WebviewWindow as __WebviewWindow__ } from "@tauri-apps/api/webviewWindow";

type __EventObj__<T> = {
  listen: (
    cb: TAURI_API_EVENT.EventCallback<T>
  ) => ReturnType<typeof TAURI_API_EVENT.listen<T>>;
  once: (
    cb: TAURI_API_EVENT.EventCallback<T>
  ) => ReturnType<typeof TAURI_API_EVENT.once<T>>;
  emit: T extends null
    ? (payload?: T) => ReturnType<typeof TAURI_API_EVENT.emit>
    : (payload: T) => ReturnType<typeof TAURI_API_EVENT.emit>;
};

export type Result<T, E> =
  | { status: "ok"; data: T }
  | { status: "error"; error: E };

function __makeEvents__<T extends Record<string, any>>(
  mappings: Record<keyof T, string>
) {
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
              case "listen":
                return (arg: any) => TAURI_API_EVENT.listen(name, arg);
              case "once":
                return (arg: any) => TAURI_API_EVENT.once(name, arg);
              case "emit":
                return (arg: any) => TAURI_API_EVENT.emit(name, arg);
            }
          },
        });
      },
    }
  );
}

     