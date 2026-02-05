use std::collections::HashMap;

use indexmap::IndexMap;

use crate::{
    simulation::simulate_game,
    core::{Agent, GameLogic, Id},
};

use super::matchmaker::{self, MatchMakerResult};

pub type TournamentResult<PID> = HashMap<PID, i32>;

pub trait AgentFactory {
    type Agent: Agent;
    fn create_agent(&self) -> Self::Agent;
}

pub trait IdGenerator {
    type Id: Id;
    fn generate_id(&mut self) -> Self::Id;
}

// this neeads a complete rehaul - I cant Have a concrete AF type, as I cannot have a concrete Agent Type.

pub fn host_tournament<G, A, AF, GG, M>(
    game: &G,
    agent_factories: HashMap<G::PID, AF>,
    matchmaker: &M,
    game_id_generator: &mut GG,
) -> TournamentResult<G::PID>
where
    G: GameLogic + Sync,
    G::PID: Send + std::fmt::Debug,
    A: Agent<Game = G> + Send,
    AF: AgentFactory<Agent = A>,
    GG: IdGenerator,
    GG::Id: Send,
    M: matchmaker::MatchMaker<PID = G::PID, GID = GG::Id> + Sync,
{
    let (sender, receiver) = std::sync::mpsc::channel();

    for game_config in matchmaker.initial_games() {
        sender.send(MatchMakerResult::GameConfig(
            game_id_generator.generate_id(),
            game_config,
        ))
        .unwrap();
    }

    crossbeam::thread::scope(|scope| {
        loop {
            match receiver.recv().unwrap() {
                MatchMakerResult::Result(tournament_result) => {
                    break tournament_result;
                }
                MatchMakerResult::GameConfig(game_id, game_config) => {
                    let thread_sender = sender.clone();
                    let mut agents = agent_factories
                        .iter()
                        .filter(|(pid, _)| game_config.contains(pid))
                        .map(|(pid, agent_factory)| (pid.clone(), agent_factory.create_agent()))
                        .collect::<IndexMap<_, _>>();

                    scope.spawn(move |_| {
                        // perhaps the thread should open a new process for this game and listen to its result. TODO
                        let game_result = simulate_game::<G, A>(&game, &mut agents, None).expect("Game should complete");

                        for match_result in matchmaker.digest_result(game_id, game_result) {
                            thread_sender.send(match_result).unwrap();
                        }
                    });
                }
            }
        }
    })
    .unwrap()
}
