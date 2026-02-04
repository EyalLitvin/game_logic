use std::collections::HashMap;

use indexmap::IndexMap;

use crate::{
    simulation::simulate_game,
    core::{Agent, FinalScores, GameLogic, Id},
};

use super::matchmaker::{self, MatchMakerOutput};

pub type TournamentResult<PID> = HashMap<PID, i32>;

pub trait AgentFactory {
    type Agent: Agent;
    fn create_agent(&self) -> Self::Agent;
}

pub trait IdGenerator {
    type Id: Id;
    fn generate_id(&mut self) -> Self::Id;
}

// TODO: needs a complete rehaul - cannot have a concrete AF type, as cannot have a concrete Agent Type.

pub fn host_tournament<G, A, AF, GG, M>(
    game: &G,
    agent_factories: HashMap<G::PID, AF>,
    matchmaker: &mut M,
    game_id_generator: &mut GG,
    max_turns: Option<usize>,
) -> TournamentResult<G::PID>
where
    G: GameLogic + Sync,
    G::PID: Send + std::fmt::Debug,
    A: Agent<Game = G> + Send,
    AF: AgentFactory<Agent = A>,
    GG: IdGenerator,
    GG::Id: Send,
    M: matchmaker::MatchMaker<PID = G::PID, GID = GG::Id>,
{
    // Channel carries raw game results back from worker threads.
    // digest_result is called on the main thread only.
    let (sender, receiver) = std::sync::mpsc::channel::<(GG::Id, FinalScores<G::PID>)>();

    crossbeam::thread::scope(|scope| {
        // Spawn workers for initial games
        for players in matchmaker.initial_games() {
            let game_id = game_id_generator.generate_id();
            let thread_sender = sender.clone();
            let mut agents = agent_factories
                .iter()
                .filter(|(pid, _)| players.contains(pid))
                .map(|(pid, factory)| (pid.clone(), factory.create_agent()))
                .collect::<IndexMap<_, _>>();

            scope.spawn(move |_| {
                let game_result = match simulate_game::<G, A>(game, &mut agents, max_turns) {
                    Ok(scores) => scores,
                    Err(_) => FinalScores::new(), // no winner: timed out or game error
                };
                thread_sender.send((game_id, game_result)).unwrap();
            });
        }

        // Main loop: receive results, run matchmaker, schedule or finish
        loop {
            let (game_id, game_result) = receiver.recv().unwrap();

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
                        let thread_sender = sender.clone();
                        let mut agents = agent_factories
                            .iter()
                            .filter(|(pid, _)| players.contains(pid))
                            .map(|(pid, factory)| (pid.clone(), factory.create_agent()))
                            .collect::<IndexMap<_, _>>();

                        scope.spawn(move |_| {
                            let game_result =
                                match simulate_game::<G, A>(game, &mut agents, max_turns) {
                                    Ok(scores) => scores,
                                    Err(_) => FinalScores::new(),
                                };
                            thread_sender.send((game_id, game_result)).unwrap();
                        });
                    }
                }
            }
        }
    })
    .unwrap()
}
