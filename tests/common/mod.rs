// Common test utilities and fixtures
// This module can be extended with other game implementations in the future

pub mod nim {
    #[path = "../../../examples/nim/game.rs"]
    pub mod game;

    #[path = "../../../examples/nim/agents.rs"]
    pub mod agents;

    pub use game::{NimGameLogic, NimMove, NimPlayerId, NimState};
    pub use agents::{NimPerfectAgent, NimRandomAgent, PerfectFactory, RandomFactory};
}
