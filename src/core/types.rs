use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt,
    hash::Hash,
};

/// Represents a unique identifier for a player in the game.
pub trait Id: Hash + Eq + Copy {}

/// Represents the final scores of a game, mapping each player ID to their score.
pub type FinalScores<PID> = HashMap<PID, i32>;

/// Errors that can occur during game operations.
#[derive(Debug, Clone)]
pub enum GameError<PID: Id> {
    /// An invalid move was attempted.
    InvalidMove {
        player: PID,
        reason: String
    },
    /// A move was made by the wrong player.
    WrongPlayer {
        expected: HashSet<PID>,
        got: PID
    },
    /// The game is in an illegal state.
    IllegalState(String),
    /// Missing moves from active players.
    MissingMoves {
        expected: HashSet<PID>,
        got: HashSet<PID>,
    },
}

impl<PID: Id + fmt::Debug> fmt::Display for GameError<PID> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::InvalidMove { player, reason } => {
                write!(f, "Invalid move by player {:?}: {}", player, reason)
            }
            GameError::WrongPlayer { expected, got } => {
                write!(f, "Wrong player: expected {:?}, got {:?}", expected, got)
            }
            GameError::IllegalState(msg) => {
                write!(f, "Illegal game state: {}", msg)
            }
            GameError::MissingMoves { expected, got } => {
                write!(f, "Missing moves from players: expected {:?}, got {:?}", expected, got)
            }
        }
    }
}

impl<PID: Id + fmt::Debug> Error for GameError<PID> {}

/// Represents the result of applying moves to a game state.
/// It can either continue with a new set of active players, or indicate the game is over with final scores.
pub enum MoveResult<PID: Id> {
    /// The game continues with the specified set of active players.
    Continue(HashSet<PID>),
    /// The game is over with the final scores.
    GameOver(FinalScores<PID>),
}
