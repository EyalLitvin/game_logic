use indexmap::IndexMap;

use crate::game_types::{Agent, GameLogic, GameResult, MoveResult};

pub fn simulate_game<G, A>(game: &G, agents: IndexMap<G::PID, A>) -> GameResult<G::PID>
where
    G: GameLogic,
    A: Agent<Game = G>,
{
    let (mut current_state, mut current_players) = game.init(agents.keys().copied().collect());

    loop {
        let player_moves = agents
            .iter()
            .filter_map(|(&pid, agent_ref)| {
                if current_players.contains(&pid) {
                    Some((
                        pid,
                        agent_ref.calculate_next_move(game.mask_state(&current_state, pid)),
                    ))
                } else {
                    agent_ref.digest_state(game.mask_state(&current_state, pid));
                    None
                }
            })
            .collect();

        match game.apply_moves(current_state, player_moves) {
            MoveResult::GameOver(result) => {
                return result;
            }
            MoveResult::NextState(state, players) => {
                (current_state, current_players) = (state, players);
            }
        }
    }
}
