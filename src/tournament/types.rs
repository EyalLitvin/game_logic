use crate::common::id::Id;
use std::collections::{HashMap, HashSet};

pub type TournamentResult<PID> = HashMap<PID, i32>;

pub type TournamentResult<PID> = HashMap<PID, i32>;

pub trait IdGenerator {
    type Id: Id;
    fn generate_id(&self) -> Self::Id;
}

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
        result: crate::game::types::GameResult<Self::PID>,
    ) -> Vec<MatchMakerResult<Self::PID, Self::GID>>;
}
