use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

/// Represents a unique identifier for a player in the game.
pub trait Id: Hash + Eq + Copy {}

/// Represents the result of a game, mapping each player ID to their score.
pub type GameResult<PID> = HashMap<PID, i32>;

/// Represents the result of applying moves to a game state.
/// It can either be the next game state along with the set of players who can make moves, or an indication that the game is over, together with the final scores.
pub enum MoveResult<GameState, PID: Id> {
    NextState(GameState, HashSet<PID>),
    GameOver(GameResult<PID>),
}
