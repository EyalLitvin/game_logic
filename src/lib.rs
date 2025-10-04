pub mod core;
pub mod simulation;
pub mod tournament;
pub mod prelude;

// Re-export commonly used items at the crate root for convenience
pub use core::{Agent, GameLogic, GameResult, Id, MoveResult};
pub use simulation::simulate_game;
