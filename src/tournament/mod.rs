pub mod matchmaker;
pub mod manager;

pub use manager::{host_tournament, AgentFactory, IdGenerator, TournamentResult};
pub use matchmaker::{MatchMaker, MatchMakerOutput};