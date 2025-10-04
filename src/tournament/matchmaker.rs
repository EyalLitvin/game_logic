use std::collections::HashSet;

use crate::core::{FinalScores, Id};

use super::manager::TournamentResult;

pub enum MatchMakerResult<PID: Id, GID: Id> {
    GameConfig(GID, HashSet<PID>),
    Result(TournamentResult<PID>),
}

pub trait MatchMaker: Sync {
    type PID: Id;
    type GID: Id;

    fn initial_games(&self) -> Vec<HashSet<Self::PID>>;

    fn digest_result(
        &self,
        game_id: Self::GID,
        result: FinalScores<Self::PID>,
    ) -> Vec<MatchMakerResult<Self::PID, Self::GID>>;
}
