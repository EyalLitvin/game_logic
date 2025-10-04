pub mod game;
pub mod agents;

pub use game::{NimGameLogic, NimMove, NimPlayerId, NimState};
pub use agents::{NimPerfectAgent, NimRandomAgent, PerfectFactory, RandomFactory};
