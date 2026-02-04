use std::collections::HashSet;

use crate::core::{FinalScores, Id};

use super::manager::TournamentResult;

/// The output of a matchmaker after processing a game result.
/// Either more games to play, or the final tournament result.
/// A matchmaker must return exactly one of these -- never a mix.
pub enum MatchMakerOutput<PID: Id> {
    /// The tournament continues with the given matchups.
    Continue(Vec<HashSet<PID>>),
    /// The tournament is over with the given final scores.
    Done(TournamentResult<PID>),
}

pub trait MatchMaker {
    type PID: Id;
    type GID: Id;

    /// Returns the initial set of matchups for the tournament.
    fn initial_games(&self) -> Vec<HashSet<Self::PID>>;

    /// Processes the result of a completed game and returns either
    /// the next round of matchups or the final tournament result.
    /// Called sequentially on the tournament host thread -- safe to
    /// mutate internal state without synchronization.
    fn digest_result(
        &mut self,
        game_id: Self::GID,
        result: FinalScores<Self::PID>,
    ) -> MatchMakerOutput<Self::PID>;
}
