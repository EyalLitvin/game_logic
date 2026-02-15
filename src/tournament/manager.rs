use std::collections::{HashMap, HashSet};

use indexmap::IndexMap;

use crate::{
    simulation::simulate_game,
    core::{Agent, FinalScores, GameLogic, Id},
};

use super::matchmaker::{self, MatchMakerOutput};

pub type TournamentResult<PID> = HashMap<PID, i32>;

/// Factory trait for creating agents.
/// For heterogeneous agent support, implement with `type Agent = Box<dyn Agent<Game = G> + Send>`.
pub trait AgentFactory {
    type Agent: Agent;
    fn create_agent(&self) -> Self::Agent;
}

pub trait IdGenerator {
    type Id: Id;
    fn generate_id(&mut self) -> Self::Id;
}

pub fn host_tournament<G, AF, GG, M>(
    game: &G,
    agent_factories: HashMap<G::PID, AF>,
    matchmaker: &mut M,
    game_id_generator: &mut GG,
    max_turns: Option<usize>,
) -> TournamentResult<G::PID>
where
    G: GameLogic + Sync,
    G::PID: Send + std::fmt::Debug,
    AF: AgentFactory,
    AF::Agent: Agent<Game = G> + Send,
    GG: IdGenerator,
    GG::Id: Send,
    M: matchmaker::MatchMaker<PID = G::PID, GID = GG::Id>,
{
    // Channel carries raw game results back from worker threads.
    // digest_result is called on the main thread only.
    let (sender, receiver) = std::sync::mpsc::channel::<(GG::Id, FinalScores<G::PID>)>();

    crossbeam::thread::scope(|scope| {
        // Captures scope, game_id_generator, sender, agent_factories, game, max_turns.
        // Does not capture matchmaker or receiver -- those are used freely in the loop below.
        let mut spawn_game = |players: &HashSet<G::PID>| {
            let game_id = game_id_generator.generate_id();
            let thread_sender = sender.clone();
            let mut agents: IndexMap<G::PID, AF::Agent> = agent_factories
                .iter()
                .filter(|(pid, _)| players.contains(pid))
                .map(|(pid, factory)| (pid.clone(), factory.create_agent()))
                .collect();

            scope.spawn(move |_| {
                let game_result = match simulate_game(game, &mut agents, max_turns) {
                    Ok(scores) => scores,
                    Err(_) => FinalScores::new(),
                };
                thread_sender.send((game_id, game_result)).unwrap();
            });
        };

        for players in matchmaker.initial_games() {
            spawn_game(&players);
        }

        // Main loop: receive results, run matchmaker, schedule or finish
        loop {
            let (game_id, game_result) = receiver.recv().unwrap();

            match matchmaker.digest_result(game_id, game_result) {
                MatchMakerOutput::Done(tournament_result) => {
                    break tournament_result;
                }
                MatchMakerOutput::Continue(next_matchups) => {
                    for players in next_matchups {
                        spawn_game(&players);
                    }
                }
            }
        }
    })
    .unwrap()
}
