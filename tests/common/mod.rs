// Common test utilities
// Include the Nim example implementation for use as a test fixture

#[path = "../../examples/nim/game.rs"]
pub mod game;

#[path = "../../examples/nim/agents.rs"]
pub mod agents;

pub use game::{NimGameLogic, NimMove, NimPlayerId, NimState};
pub use agents::{NimPerfectAgent, NimRandomAgent, PerfectFactory, RandomFactory};
