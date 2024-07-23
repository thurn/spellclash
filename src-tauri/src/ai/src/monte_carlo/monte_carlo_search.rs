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

//! An implementation of Monte Carlo Tree Search.
//!
//! This implementation is based on the pseudocode given in "A Survey of Monte
//! Carlo Tree Search Methods" by Browne et al. in IEEE Transactions on
//! Computational Intelligence and AI in Games, Vol. 4, No. 1, March 2012.

use std::collections::{BTreeMap, BTreeSet};
use std::marker::PhantomData;
use std::time::Instant;

use ai_core::core::agent_state::AgentState;
use ai_core::core::monte_carlo_agent_state::{
    MonteCarloAgentState, SearchEdge, SearchGraph, SearchNode, SearchOperation,
};
use data::actions::agent_action::AgentAction;
use data::core::primitives;
use data::core::primitives::PlayerName;
use data::game_states::game_state::GameState;
use petgraph::prelude::{EdgeRef, NodeIndex};
use petgraph::Direction;
use rand::prelude::IteratorRandom;
use rand::SeedableRng;
use rand_xoshiro::SplitMix64;
use tracing::{info, instrument};
use utils::command_line;
use utils::command_line::TracingStyle;

use crate::core::game_state_node::{GameStateNode, GameStatus};
use crate::core::selection_algorithm::SelectionAlgorithm;
use crate::core::state_evaluator::StateEvaluator;
use crate::monte_carlo::child_score::{ChildScoreAlgorithm, SelectionMode};

/// Plays out a game using random moves until a terminal state is reached, then
/// evaluates the result using the provided state evaluator.
///
/// Pseudocode:
/// ```text
/// 𝐟𝐮𝐧𝐜𝐭𝐢𝐨𝐧 DEFAULTPOLICY(s)
///   𝐰𝐡𝐢𝐥𝐞 s is non-terminal 𝐝𝐨
///     choose 𝒂 ∈ A(s) uniformly at random
///     s ← f(s,𝒂)
///   𝐫𝐞𝐭𝐮𝐫𝐧 reward for state s
/// ```
#[derive(Debug, Clone)]
pub struct RandomPlayoutEvaluator<TState: GameStateNode + Send, TEvaluator: StateEvaluator<TState>>
{
    pub evaluator: TEvaluator,
    pub phantom_data: PhantomData<TState>,
}

impl<TState: GameStateNode + Send, TEvaluator: StateEvaluator<TState>> StateEvaluator<TState>
    for RandomPlayoutEvaluator<TState, TEvaluator>
{
    #[instrument(level = "debug", skip_all)]
    fn evaluate(&self, input: &TState, player: TState::PlayerName) -> i32 {
        let mut game = input.make_copy();
        game.set_agent_state(AgentState::MonteCarlo(MonteCarloAgentState {
            graph: SearchGraph::new(),
            search_operation: Some(SearchOperation::EvaluateNode {
                rng: SplitMix64::seed_from_u64(156562599311216480),
            }),
        }));
        let mut rng = SplitMix64::seed_from_u64(156562599311216480);
        loop {
            match game.status() {
                GameStatus::Completed { .. } => {
                    return self.evaluator.evaluate(&game, player);
                }
                GameStatus::InProgress { current_turn } => {
                    let action = game
                        .legal_actions(current_turn)
                        .choose(&mut rng)
                        .expect("No actions found");
                    game.execute_action(current_turn, action);
                }
            }
        }
    }
}

/// Monte Carlo search algorithm.
///
/// Monte carlo tree search operates over a tree of game state nodes
/// connected by game actions. The search follows these three steps
/// repeatedly:
///
/// 1) **Tree Policy:** Find a node in the tree which has not previously
/// been explored. The UCT algorithm is one mathematical heuristic
/// for how to prioritize nodes to explore.
///
/// 2) **Default Policy:** Score this node to determine its reward value
/// (∆), typically by playing random moves until the game terminates.
///
/// 3) **Backpropagation:** Walk back up the tree, adding the resulting
/// reward value to each parent node.
///
/// Pseudocode:
/// ```text
/// 𝐟𝐮𝐧𝐜𝐭𝐢𝐨𝐧 UCTSEARCH(s₀)
///   create root node v₀ with state s₀
///   𝐰𝐡𝐢𝐥𝐞 within computational budget 𝐝𝐨
///     v₁ ← TREEPOLICY(v₀)
///     ∆ ← DEFAULTPOLICY(s(v₁))
///     BACKUP(v₁, ∆)
///   𝐫𝐞𝐭𝐮𝐫𝐧 𝒂(BESTCHILD(v₀, 0))
/// ```
#[derive(Debug, Clone)]
pub struct MonteCarloAlgorithm<TState, TScoreAlgorithm: ChildScoreAlgorithm>
where
    TState: GameStateNode + Clone + Send,
{
    pub child_score_algorithm: TScoreAlgorithm,
    pub max_iterations: Option<u32>,
    pub phantom_data: PhantomData<TState>,
}

impl<TState, TEvaluator, TScoreAlgorithm: ChildScoreAlgorithm>
    SelectionAlgorithm<TState, TEvaluator> for MonteCarloAlgorithm<TState, TScoreAlgorithm>
where
    TState: GameStateNode + Clone + Send,
    TEvaluator: StateEvaluator<TState>,
{
    #[instrument(level = "debug", skip_all)]
    fn pick_action(
        &self,
        deadline: Instant,
        node: &TState,
        evaluator: &TEvaluator,
        player: TState::PlayerName,
    ) -> TState::Action
    where
        TState: GameStateNode,
        TEvaluator: StateEvaluator<TState>,
    {
        let max_iterations = self.max_iterations;
        self.run_search(
            |i| deadline < Instant::now() || max_iterations.map_or(false, |max| i > max),
            node,
            evaluator,
            player,
        )
    }

    fn pick_prompt_action(
        &self,
        game: &mut TState,
        player: TState::PlayerName,
        actions: BTreeSet<TState::Action>,
    ) -> TState::Action {
        let current_position = match &mut game.state_mut().search_operation {
            Some(SearchOperation::EvaluateNode { rng }) => {
                return actions.iter().choose(rng).copied().unwrap()
            }
            Some(SearchOperation::TreeSearch { target_position, .. }) => *target_position,
            None => {
                panic!("Expected search operation")
            }
        };

        let explored = game
            .state()
            .graph
            .edges(current_position)
            .map(|e| e.weight().action)
            .collect::<BTreeSet<_>>();
        if let Some(action) = actions.iter().find(|a| !explored.contains(a)) {
            // An action exists which has not yet been tried
            let target = game.state_mut().graph.add_node(SearchNode {
                player,
                total_reward: 0.0,
                visit_count: 0,
            });
            game.state_mut()
                .graph
                .add_edge(current_position, target, SearchEdge { action: *action });
            game.state_mut().search_operation = Some(SearchOperation::TreeSearch {
                source_position: current_position,
                target_position: target,
            });
            *action
        } else {
            // All actions have been tried, recursively search the best candidate
            let (action, action_index) = self.best_child(
                &game.state().graph,
                current_position,
                actions,
                SelectionMode::Exploration,
            );
            game.state_mut().search_operation = Some(SearchOperation::TreeSearch {
                source_position: current_position,
                target_position: action_index,
            });
            action
        }
    }
}

impl<TState, TScoreAlgorithm: ChildScoreAlgorithm> MonteCarloAlgorithm<TState, TScoreAlgorithm>
where
    TState: GameStateNode + Clone + Send,
{
    #[instrument(level = "debug", skip_all)]
    pub fn run_search<TEvaluator: StateEvaluator<TState>>(
        &self,
        should_halt: impl Fn(u32) -> bool,
        initial_game: &TState,
        evaluator: &TEvaluator,
        player: TState::PlayerName,
    ) -> TState::Action {
        let mut agent_state =
            MonteCarloAgentState { graph: SearchGraph::new(), search_operation: None };
        let root =
            agent_state.graph.add_node(SearchNode { total_reward: 0.0, visit_count: 1, player });
        let mut i = 0;
        while !should_halt(i) {
            if i > 0 && i % 1000 == 0 {
                println!("Iteration {}", i);
            }
            let mut game_copy = initial_game.make_copy();
            game_copy.set_state(agent_state);
            let node = self.tree_policy(&mut game_copy, root);
            game_copy.state_mut().search_operation = Some(SearchOperation::EvaluateNode {
                rng: SplitMix64::seed_from_u64(156562599311216480),
            });
            let reward = f64::from(evaluator.evaluate(&game_copy, player));
            Self::backup(&mut game_copy.state_mut().graph, player, node, reward);
            i += 1;
            agent_state = game_copy.take_state();
        }

        let (action, _) = self.best_child(
            &agent_state.graph,
            root,
            initial_game.legal_actions(player).collect(),
            SelectionMode::Best,
        );

        self.log_results(i, &agent_state.graph, root);
        action
    }

    #[instrument(level = "debug", skip_all)]
    fn log_results(
        &self,
        count: u32,
        graph: &SearchGraph<TState::PlayerName, TState::Action>,
        root: NodeIndex,
    ) {
        info!("Search completed in {} iterations", count);
        if command_line::flags().tracing_style == TracingStyle::AggregateTime {
            println!(">>> Search completed in {} iterations\n", count);
        }
        let parent_visits = graph[root].visit_count;
        let mut edges = graph
            .edges(root)
            .map(|edge| {
                let child = &graph[edge.target()];
                (
                    edge,
                    self.child_score_algorithm.score(
                        f64::from(parent_visits),
                        f64::from(child.visit_count),
                        child.total_reward,
                        SelectionMode::Best,
                    ),
                )
            })
            .collect::<Vec<_>>();
        edges.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
        edges.reverse();

        for (child, weight) in edges.iter().map(|(edge, weight)| (edge.weight().action, *weight)) {
            info!("Action: {:?} at {:?}", weight, child);
        }
    }

    /// Returns a descendant node to examine next for the provided parent node,
    /// either:
    ///  * A node which has not yet been explored
    ///  * The best terminal node descendant, if all nodes have been explored.
    ///
    /// If possible actions are available from this node which have not yet been
    /// explored, selects an action and applies it, returning the result as a
    /// new child. Otherwise, selects the best child to explore based on
    /// visit counts and known rewards, using the [best_child] algorithm,
    /// and then repeats this process recursively until an unseen node is
    /// found (or the best child is terminal).
    ///
    /// Mutates the provided [GameStateNode] to represent the game state at the
    /// returned node.
    ///
    /// Cᵖ is the exploration constant, Cᵖ = 1/√2 was suggested by Kocsis and
    /// Szepesvári as a good choice.
    ///
    /// Pseudocode:
    /// ```text
    /// 𝐟𝐮𝐧𝐜𝐭𝐢𝐨𝐧 TREEPOLICY(v)
    ///   𝐰𝐡𝐢𝐥𝐞 v is nonterminal 𝐝𝐨
    ///     𝐢𝐟 v not fully expanded 𝐭𝐡𝐞𝐧
    ///       𝐫𝐞𝐭𝐮𝐫𝐧 EXPAND(v)
    ///     𝐞𝐥𝐬𝐞
    ///       v ← BESTCHILD(v, Cᵖ)
    ///   𝐫𝐞𝐭𝐮𝐫𝐧 v
    /// ```
    #[instrument(level = "debug", skip_all)]
    fn tree_policy(&self, game: &mut TState, mut node_index: NodeIndex) -> NodeIndex {
        while let GameStatus::InProgress { current_turn } = game.status() {
            let actions = game.legal_actions(current_turn).collect::<BTreeSet<_>>();
            let explored = game
                .state()
                .graph
                .edges(node_index)
                .map(|e| e.weight().action)
                .collect::<BTreeSet<_>>();
            if let Some(action) = actions.iter().find(|a| !explored.contains(a)) {
                // An action exists which has not yet been tried
                return self.expand(game, current_turn, node_index, *action);
            } else {
                // All actions have been tried, recursively search the best candidate
                let (action, action_index) = self.best_child(
                    &game.state().graph,
                    node_index,
                    actions,
                    SelectionMode::Exploration,
                );
                node_index =
                    game.execute_search_action(node_index, action_index, current_turn, action);
            }
        }
        node_index
    }

    /// Generates a new tree node by applying the next untried action from the
    /// provided input node. Mutates the provided [GameState] to apply the
    /// provided game action.
    ///
    /// Pseudocode:
    /// ```text
    /// 𝐟𝐮𝐧𝐜𝐭𝐢𝐨𝐧 EXPAND(v)
    ///   choose 𝒂 ∈ untried actions from A(s(v))
    ///   add a new child v′ to v
    ///     with s(v′) = f(s(v), 𝒂)
    ///     and 𝒂(v′) = 𝒂
    ///   𝐫𝐞𝐭𝐮𝐫𝐧 v′
    /// ```
    #[instrument(level = "debug", skip_all)]
    fn expand(
        &self,
        game: &mut TState,
        player: TState::PlayerName,
        source: NodeIndex,
        action: TState::Action,
    ) -> NodeIndex {
        let target = game.state_mut().graph.add_node(SearchNode {
            player,
            total_reward: 0.0,
            visit_count: 0,
        });
        game.state_mut().graph.add_edge(source, target, SearchEdge { action });
        game.execute_search_action(source, target, player, action)
    }

    /// Picks the most promising child node to explore, returning its associated
    /// action and node identifier.
    #[instrument(level = "debug", skip_all)]
    fn best_child(
        &self,
        graph: &SearchGraph<TState::PlayerName, TState::Action>,
        node: NodeIndex,
        legal: BTreeSet<TState::Action>,
        selection_mode: SelectionMode,
    ) -> (TState::Action, NodeIndex) {
        let parent_visits = graph[node].visit_count;
        let (edge, _) = graph
            .edges(node)
            // We re-check action legality here because the set of legal actions can change between
            // visits, e.g. if different cards are drawn
            .filter(|edge| legal.contains(&edge.weight().action))
            .map(|edge| {
                let child = &graph[edge.target()];
                // This can technically panic when invoked from root with a very small
                // simulation count, so don't do that :)
                assert_ne!(child.visit_count, 0);
                (
                    edge,
                    self.child_score_algorithm.score(
                        f64::from(parent_visits),
                        f64::from(child.visit_count),
                        child.total_reward,
                        selection_mode,
                    ),
                )
            })
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .expect("No children found");
        (edge.weight().action, edge.target())
    }

    /// Once a playout is completed, the backpropagation step walks back up the
    /// hierarchy of parent nodes, adding the resulting reward value to each
    /// one.
    ///
    /// Pseudocode:
    /// ```text
    /// 𝐟𝐮𝐧𝐜𝐭𝐢𝐨𝐧 BACKUP(v,∆)
    ///   𝐰𝐡𝐢𝐥𝐞 v is not null 𝐝𝐨
    ///     N(v) ← N(v) + 1
    ///     Q(v) ← Q(v) + ∆(v, p)
    ///     v ← parent of v
    /// ```
    #[instrument(level = "debug", skip_all)]
    fn backup(
        graph: &mut SearchGraph<TState::PlayerName, TState::Action>,
        maximizing_player: TState::PlayerName,
        mut node: NodeIndex,
        reward: f64,
    ) {
        loop {
            let weight = graph.node_weight_mut(node).expect("Node not found");
            weight.visit_count += 1;
            weight.total_reward +=
                if weight.player == maximizing_player { reward } else { -reward };

            node = match graph.neighbors_directed(node, Direction::Incoming).next() {
                Some(n) => n,
                _ => return,
            };
        }
    }
}

trait MonteCarloGameState<TPlayer, TAction> {
    fn execute_search_action(
        &mut self,
        source: NodeIndex,
        target: NodeIndex,
        player: TPlayer,
        action: TAction,
    ) -> NodeIndex;

    fn set_state(&mut self, state: MonteCarloAgentState<TPlayer, TAction>);

    fn state(&self) -> &MonteCarloAgentState<TPlayer, TAction>;

    fn state_mut(&mut self) -> &mut MonteCarloAgentState<TPlayer, TAction>;

    fn take_state(self) -> MonteCarloAgentState<TPlayer, TAction>;
}

impl<T: GameStateNode> MonteCarloGameState<T::PlayerName, T::Action> for T {
    fn execute_search_action(
        &mut self,
        source: NodeIndex,
        target: NodeIndex,
        player: T::PlayerName,
        action: T::Action,
    ) -> NodeIndex {
        self.state_mut().search_operation =
            Some(SearchOperation::TreeSearch { source_position: source, target_position: target });
        self.execute_action(player, action);
        let Some(SearchOperation::TreeSearch { target_position, .. }) =
            self.state().search_operation
        else {
            panic!("Expected tree search operation")
        };
        self.state_mut().search_operation = None;
        target_position
    }

    fn set_state(&mut self, state: MonteCarloAgentState<T::PlayerName, T::Action>) {
        self.set_agent_state(AgentState::MonteCarlo(state));
    }

    fn state(&self) -> &MonteCarloAgentState<T::PlayerName, T::Action> {
        match self.get_agent_state() {
            AgentState::MonteCarlo(ref data) => data,
            _ => panic!("Expected monte carlo agent state"),
        }
    }

    fn state_mut(&mut self) -> &mut MonteCarloAgentState<T::PlayerName, T::Action> {
        match self.get_agent_state_mut() {
            AgentState::MonteCarlo(ref mut data) => data,
            _ => panic!("Expected monte carlo agent state"),
        }
    }

    fn take_state(self) -> MonteCarloAgentState<T::PlayerName, T::Action> {
        match self.take_agent_state() {
            AgentState::MonteCarlo(data) => data,
            _ => panic!("Expected monte carlo agent state"),
        }
    }
}
