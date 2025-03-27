use std::collections::HashMap;

use indexmap::IndexMap;

use crate::game_types::{Agent, GameLogic, GameResult, MoveResult, PlayerID};
struct AgentProcessPlaceHolder<A> {
    my_agent: A,
}
    
impl<A: Agent> AgentProcessPlaceHolder<A> {
    fn get_move(&self, current_state: &<A::Game as GameLogic>::MaskedState) -> <A::Game as GameLogic>::Move {
        let mut chosen_move: Option<<A::Game as GameLogic>::Move> = None;
        self.my_agent.calculate_next_move(current_state, &mut chosen_move);
        chosen_move.expect("agent didn't return move")
    }

    fn update(&self, current_state: &<A::Game as GameLogic>::MaskedState) {
        self.my_agent.digest_state(current_state);
    }
}

fn setup_agent_processes<A: Agent + Copy>(agents: &IndexMap<<A::Game as GameLogic>::PID, A>) -> HashMap<<A::Game as GameLogic>::PID, AgentProcessPlaceHolder<A>> {
    agents.iter().map(|(&p, &a)| (p, AgentProcessPlaceHolder {my_agent: a})).collect()
}

pub fn simulate_game<G, A>(game: &G, agents: IndexMap<G::PID, A>) -> GameResult<G::PID>
where 
    G: GameLogic,
    A: Agent<Game = G> + Copy,
{
    let (mut current_state, mut current_player) = game.init(agents.keys().copied().collect());
    let agent_processes = setup_agent_processes(&agents);
    loop {
        let mut player_move: Option<G::Move> = None;
        // update all agents on current state
        for (&agent, agent_process) in agent_processes.iter() {
            if agent == current_player {
                player_move = Some(agent_process.get_move(&game.mask_state(&current_state, agent)));
            } else {
                agent_process.update(&game.mask_state(&current_state, agent));
            }
        }

        match game.make_move(current_state, current_player, player_move.expect("didn't get move from player")) {
            MoveResult::GameOver(result) => {
                return result;
            },
            MoveResult::NextState(state, player, ) => {
                (current_state, current_player) = (state, player);
            }
        }
    }
}

