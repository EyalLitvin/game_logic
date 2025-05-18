use std::collections::HashMap;

use super::types::{MatchMaker, MatchMakerResult, TournamentResult};

pub fn host_tournament<G, A, GG, M>(
    game: &G,
    agents: &HashMap<G::PID, A>,
    matchmaker: &M,
    game_id_generator: &GG,
) -> TournamentResult<G::PID>
where
    G: crate::game::types::GameLogic + Sync,
    G::PID: Send,
    A: crate::game::types::Agent<Game = G> + Clone + Send,
    GG: super::types::IdGenerator,
    GG::Id: Send,
    M: MatchMaker<PID = G::PID, GID = GG::Id> + Sync,
{
    let (rx, tx) = std::sync::mpsc::channel();

    for game_config in matchmaker.initial_games() {
        rx.send(MatchMakerResult::GameConfig(
            game_id_generator.generate_id(),
            game_config,
        ))
        .unwrap();
    }

    crossbeam::thread::scope(|scope| {
        loop {
            match tx.recv().unwrap() {
                MatchMakerResult::Result(tournament_result) => {
                    break tournament_result;
                }
                MatchMakerResult::GameConfig(game_id, game_config) => {
                    let thread_rx = rx.clone();
                    let agents = agents
                        .iter()
                        .filter(|(pid, _)| game_config.contains(pid))
                        .map(|(pid, agent_ref)| (pid.clone(), agent_ref.clone()))
                        .collect();

                    scope.spawn(move |_| {
                        // perhaps the thread should open a new process for this game and listen to its result. TODO
                        let game_result = crate::game::engine::simulate_game::<G, A>(&game, agents);

                        for match_result in matchmaker.digest_result(game_id, game_result) {
                            thread_rx.send(match_result).unwrap();
                        }
                    });
                }
            }
        }
    })
    .unwrap()
}
