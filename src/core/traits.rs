use std::collections::{HashMap, HashSet};

use super::types::{GameError, Id, MoveResult};

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

    /// Applies the given moves to the game state in-place and returns the result.
    ///
    /// # Arguments
    /// * `state` - A mutable reference to the current game state.
    /// * `moves` - A mapping of player IDs to their respective moves.
    ///
    /// # Returns
    /// A `Result` containing either:
    /// - `Ok(MoveResult)` - The game continues with active players, or the game is over with final scores
    /// - `Err(GameError)` - An error occurred (invalid move, wrong player, etc.)
    ///
    /// # Errors
    /// Returns `GameError` if:
    /// - A move is invalid for the current state
    /// - A move is made by a non-active player
    /// - Required moves are missing from active players
    fn apply_moves(
        &self,
        state: &mut Self::State,
        moves: HashMap<Self::PID, Self::Move>,
    ) -> Result<MoveResult<Self::PID>, GameError<Self::PID>>;

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
/// Parameterized by the game type G rather than using an associated type,
/// which keeps the trait object-safe and allows heterogeneous agents
/// (e.g. Box<dyn Agent<G>>) in the same collection.
pub trait Agent<G: GameLogic> {
    /// Updates the agent's internal state based on the new game state.
    /// This is called with a new game state when it's not the agent's turn.
    fn digest_state(&mut self, new_state: G::MaskedState);

    /// Calculates the next move for the agent based on the new game state.
    /// This is called when it's the agent's turn to make a move.
    fn calculate_next_move(&mut self, new_state: G::MaskedState) -> G::Move;
}

/// Blanket impl so that Box<dyn Agent<G>> can be used wherever Agent<G> is expected.
impl<G: GameLogic, A: Agent<G> + ?Sized> Agent<G> for Box<A> {
    fn digest_state(&mut self, new_state: G::MaskedState) {
        (**self).digest_state(new_state);
    }

    fn calculate_next_move(&mut self, new_state: G::MaskedState) -> G::Move {
        (**self).calculate_next_move(new_state)
    }
}

/// Extension trait for games that can enumerate legal moves from a player's perspective.
/// Not all games need this -- it is an optional capability for agents that want to
/// query available moves before deciding. The enumeration is based on MaskedState,
/// not the full State, so it only reflects what the player can observe.
pub trait LegalMoves: GameLogic {
    /// Returns the legal moves for a player given their view of the game state.
    ///
    /// # Arguments
    /// * `state` - The masked game state visible to the player.
    /// * `player` - The player ID to get legal moves for.
    ///
    /// # Returns
    /// A vector of legal moves for the player. Empty if no legal moves are available.
    fn legal_moves(&self, state: &Self::MaskedState, player: Self::PID) -> Vec<Self::Move>
    where
        Self::Move: Clone;
}
