use crate::common::id::Id;
use std::collections::{HashMap, HashSet};

/// Represents the result of a game, mapping each player ID to their score.
pub type GameResult<PID> = HashMap<PID, i32>;

/// Represents the result of applying moves to a game state.
/// It can either be the next game state along with the set of players who can make moves, or an indication that the game is over, together with the final scores.
pub enum MoveResult<GameState, PID: Id> {
    NextState(GameState, HashSet<PID>),
    GameOver(GameResult<PID>),
}

/// Represents the logic of a turn-based game.
pub trait GameLogic {
    /// The type of player ID used in the game.
    type PID: Id;
    /// The type of move that can be made in the game.
    type Move;
    /// The type of game state used in the game.
    type State;
    /// The type of masked state, which is a representation of the game state that is visible to a player.
    type MaskedState;

    /// Initializes the game state with the given players.
    ///
    /// # Arguments
    /// * `players` - A vector of player IDs representing the players in the game.
    ///
    /// # Returns
    /// A tuple containing the initial game state and a set of player IDs that can make moves.
    fn init(&self, players: Vec<Self::PID>) -> (Self::State, HashSet<Self::PID>);

    /// Applies the given moves to the game state and returns the result.
    ///
    /// # Arguments
    /// * `state` - The current game state.
    /// * `moves` - A mapping of player IDs to their respective moves.
    ///
    /// # Returns
    /// A `MoveResult` that can either be the next game state along with the set of players who can make moves, or an indication that the game is over, together with the final scores.
    ///
    /// # Panics
    /// There is currently no validation of moves, so this function may panic if the moves are invalid.
    /// Additionally, it is up to the caller to ensure that the moves are made by the correct players.
    fn apply_moves(
        &self,
        state: Self::State,
        moves: HashMap<Self::PID, Self::Move>,
    ) -> MoveResult<Self::State, Self::PID>;

    /// Masks the game state for a specific player, returning a representation of the state that is visible to that player.
    ///
    /// # Arguments
    /// * `state` - The current game state.
    /// * `player` - The player ID for whom the state should be masked.
    ///
    /// # Returns
    /// A masked state that is a representation of the game state that is visible to the specified player.
    fn mask_state(&self, state: &Self::State, player: Self::PID) -> Self::MaskedState;
}

/// Represents an agent that can play a game.
pub trait Agent {
    // The game the agent is playing.
    type Game: GameLogic;

    // type State = <Self::Game as GameLogic>::MaskedState;
    // type Move = <Self::Game as GameLogic>::Move;

    /// The for the agent to update its internal state based on the new game state.
    /// This is called with a new game state that the agent should digest, but not decide on a move for (as it is not the agent's turn).
    ///
    /// # Arguments
    /// * `new_state` - The new game state that the agent should digest.
    fn digest_state(&self, new_state: <Self::Game as GameLogic>::MaskedState);

    /// Calculates the next move for the agent based on the new game state.
    /// This is called with a new game state that the agent should decide on a move for (as it is the agent's turn).
    ///
    /// # Arguments
    /// * `new_state` - The new game state that the agent should decide on a move for.
    ///
    /// # Returns
    /// The move that the agent has decided to make.
    fn calculate_next_move(
        &self,
        new_state: <Self::Game as GameLogic>::MaskedState,
    ) -> <Self::Game as GameLogic>::Move;
}
