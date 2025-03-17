use std::{cmp::Ordering, collections::HashMap};

use indexmap::IndexMap;
use rand::Rng;

use crate::{game_simulation, game_types::*};

struct NimGameLogic {
    max_takes: u32,
    initial_pile_size: u32,
}

struct NimMove {
    amount: u32,
}
#[derive(Clone)]
struct NimState {
    pile_size: u32,
    players: Vec<PlayerID>,
}

impl NimState {
    fn next_player(&self, player: PlayerID) -> PlayerID {
        self.players[(self.players.iter().position(|&x| x == player).expect("Couldn't find current player in players") + 1) % self.players.len()]
    }
}

impl GameLogic for NimGameLogic {
    type Move = NimMove;
    type State = NimState;
    type MaskedState = NimState;

    fn init(&self, players: Vec<PlayerID>) -> (Self::State, PlayerID) {
        assert!(players.len() == 2);
        (
            NimState {
                pile_size: self.initial_pile_size,
                players: players.iter().copied().collect(),
            },
            players[0],
        )

    }

    fn make_move(&self, state: Self::State, player: PlayerID, player_move: Self::Move) -> MoveResult<Self::State> {
        println!("player {} making a move", player.0);
        if player_move.amount > self.max_takes || player_move.amount == 0 {
            return MoveResult::GameOver(vec![(player, -1)].into_iter().collect());
        }
        let next_player = state.next_player(player);
        match state.pile_size.cmp(&player_move.amount) {
            Ordering::Less => MoveResult::GameOver(vec![(player, -1)].into_iter().collect()),
            Ordering::Equal => {
                println!("the game should end! {}", player.0);
                MoveResult::GameOver(vec![(player, 1)].into_iter().collect())
            },
            Ordering::Greater => MoveResult::NextState(NimState {
                pile_size: state.pile_size - player_move.amount,
                players: state.players,
            }, next_player)
        }
    }

    fn mask_state(&self, state: &Self::State, _player: PlayerID) -> Self::MaskedState {
        state.clone()
    }
}



fn standard_nim_game() {
    let nim_logic = NimGameLogic {
        initial_pile_size: 10,
        max_takes: 4,
    };
    let first_player = PlayerID(0);
    let second_player = PlayerID(1);
    let (mut nim_state , mut current_player)= nim_logic.init(vec![first_player, second_player]);
    match nim_logic.make_move(nim_state, first_player, NimMove {amount: 3}) {
        MoveResult::GameOver(_) => panic!(),
        MoveResult::NextState(next_state, next_player) => (nim_state, current_player) = (next_state, next_player)
    };
    assert!(nim_state.pile_size == 7);
    assert!(current_player == second_player);
    match nim_logic.make_move(nim_state, current_player,  NimMove {amount: 2}) {
        MoveResult::GameOver(_) => panic!(),
        MoveResult::NextState(next_state, next_player) => (nim_state, current_player) = (next_state, next_player)
    };
    assert!(nim_state.pile_size == 5);
    assert!(current_player == first_player);
    match nim_logic.make_move(nim_state, current_player, NimMove {amount: 1}) {
        MoveResult::GameOver(_) => panic!(),
        MoveResult::NextState(next_state, next_player) => (nim_state, current_player) = (next_state, next_player)
    };
    assert!(nim_state.pile_size == 4);
    assert!(current_player == second_player);
    match nim_logic.make_move(nim_state, current_player, NimMove {amount: 4}) {
        MoveResult::GameOver(game_result) => {
            assert!(game_result[&second_player] == 1);
        },
        MoveResult::NextState(_s, _p) => panic!(),
    };
}


#[derive(Copy, Clone)]
struct NimPerfectAgent {
    mod_base: u32,
}

impl Agent for NimPerfectAgent {
    type Game = NimGameLogic;

    fn new(game: &Self::Game) -> Self {
        NimPerfectAgent {
            mod_base: game.max_takes + 1,
        }
    }

    fn calculate_next_move(&self, new_state: &NimState, chosen_move: &mut Option<NimMove>) {
        match new_state.pile_size % self.mod_base {
            0 => *chosen_move = Some(NimMove {amount: 1}),
            x => *chosen_move = Some(NimMove {amount: x}),
        }
        println!("player tried to take {} when pile was {}", chosen_move.as_mut().expect("didn't make move").amount, new_state.pile_size);
    }

    fn digest_state(&self, _new_state: &<Self::Game as GameLogic>::MaskedState) {
        
    }
}


fn test_agents() {
    println!("start!");
    let mut rand = rand::rng();
    for pile_size in 1..=300 {
        for max_takes in 1..=20 {
            let nim_logic = NimGameLogic {
                initial_pile_size: rand.random_range(1..=300),
                max_takes: rand.random_range(1..=10),
            };

            let agent_1 = NimPerfectAgent::new(&nim_logic); 
            let agent_2 = NimPerfectAgent::new(&nim_logic); 

            let agents: IndexMap<PlayerID, NimPerfectAgent> = vec![(PlayerID(1), agent_1), (PlayerID(2), agent_2)].into_iter().collect();

            use game_simulation::simulate_game;
            let result = simulate_game(&nim_logic, agents);
            let winner = if nim_logic.initial_pile_size % (nim_logic.max_takes + 1) == 0 {PlayerID(2)} else {PlayerID(1)};
            assert!(result.contains_key(&winner), "winner lost {}, {}, winner: {}", nim_logic.initial_pile_size, nim_logic.max_takes, winner.0);
            assert!(result[&winner] == 1, "winner won! {}, {}, {}", nim_logic.initial_pile_size, nim_logic.max_takes, winner.0);     

        }

    }
}

#[test]
fn test_many_times() {
    for _ in 0..10_000 {
        test_agents();
    }
}