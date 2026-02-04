use std::collections::HashMap;

use indexmap::IndexMap;

use crate::{
    simulation::simulate_game,
    core::{Agent, FinalScores, GameLogic, Id},
};

use super::matchmaker::{self, MatchMakerOutput};

pub type TournamentResult<PID> = HashMap<PID, i32>;

/// Factory trait for creating agents. Parameterized by game type G.
/// Object-safe: implementations can be stored as Box<dyn AgentFactory<G>>,
/// allowing a tournament to hold factories for different agent types.
pub trait AgentFactory<G: GameLogic> {
    fn create_agent(&self) -> Box<dyn Agent<G> + Send>;
}

pub trait IdGenerator {
    type Id: Id;
    fn generate_id(&mut self) -> Self::Id;
}

pub fn host_tournament<G, GG, M>(
    game: &G,
    agent_factories: HashMap<G::PID, Box<dyn AgentFactory<G>>>,
    matchmaker: &mut M,
    game_id_generator: &mut GG,
    max_turns: Option<usize>,
) -> TournamentResult<G::PID>
where
    G: GameLogic + Sync,
    G::PID: Send + std::fmt::Debug,
    GG: IdGenerator,
    GG::Id: Send,
    M: matchmaker::MatchMaker<PID = G::PID, GID = GG::Id>,
{
    // Channel carries raw game results back from worker threads.
    // digest_result is called on the main thread only.
    let (tx, rx) = std::sync::mpsc::channel::<(GG::Id, FinalScores<G::PID>)>();

    crossbeam::thread::scope(|scope| {
        // Spawn workers for initial games
        for players in matchmaker.initial_games() {
            let game_id = game_id_generator.generate_id();
            let thread_tx = tx.clone();
            let mut agents: IndexMap<G::PID, Box<dyn Agent<G> + Send>> = agent_factories
                .iter()
                .filter(|(pid, _)| players.contains(pid))
                .map(|(pid, factory)| (pid.clone(), factory.create_agent()))
                .collect();

            scope.spawn(move |_| {
                let game_result = match simulate_game(game, &mut agents, max_turns) {
                    Ok(scores) => scores,
                    Err(_) => FinalScores::new(), // no winner: timed out or game error
                };
                thread_tx.send((game_id, game_result)).unwrap();
            });
        }

        // Main loop: receive results, run matchmaker, schedule or finish
        loop {
            let (game_id, game_result) = rx.recv().unwrap();

            match matchmaker.digest_result(game_id, game_result) {
                MatchMakerOutput::Done(tournament_result) => {
                    break tournament_result;
                }
                MatchMakerOutput::Continue(next_matchups) => {
                    debug_assert!(
                        !next_matchups.is_empty(),
                        "MatchMaker returned Continue with no games -- tournament will deadlock"
                    );

                    for players in next_matchups {
                        let game_id = game_id_generator.generate_id();
                        let thread_tx = tx.clone();
                        let mut agents: IndexMap<G::PID, Box<dyn Agent<G> + Send>> =
                            agent_factories
                                .iter()
                                .filter(|(pid, _)| players.contains(pid))
                                .map(|(pid, factory)| (pid.clone(), factory.create_agent()))
                                .collect();

                        scope.spawn(move |_| {
                            let game_result =
                                match simulate_game(game, &mut agents, max_turns) {
                                    Ok(scores) => scores,
                                    Err(_) => FinalScores::new(),
                                };
                            thread_tx.send((game_id, game_result)).unwrap();
                        });
                    }
                }
            }
        }
    })
    .unwrap()
}
