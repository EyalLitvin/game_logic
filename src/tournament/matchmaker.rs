use std::collections::HashSet;

use crate::game_types::{GameResult, Id};

use super::tournament_manager::TournamentResult;

pub enum MatchMakerResult<PID: Id, GID: Id> {
    GameConfig(GID, HashSet<PID>),
    Result(TournamentResult<PID>),
}

pub trait MatchMaker: Sync {
    type PID: Id;
    type GID: Id;

    fn initial_games(&self) -> impl Iterator<Item = HashSet<Self::PID>>;

    fn digest_result(
        &self,
        game_id: Self::GID,
        result: GameResult<Self::PID>,
    ) -> Vec<MatchMakerResult<Self::PID, Self::GID>>;
}
