use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use indexmap::IndexMap;

use crate::{
    game_simulation,
    game_types::{Agent, GameLogic, GameResult, MoveResult, PlayerId},
};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct NimPlayerId(u32);

impl PlayerId for NimPlayerId {}

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
    players: Vec<NimPlayerId>,
}

impl NimState {
    fn next_player(&self, player: NimPlayerId) -> NimPlayerId {
        self.players[(self
            .players
            .iter()
            .position(|&x| x == player)
            .expect("Couldn't find current player in players")
            + 1)
            % self.players.len()]
    }
}

impl GameLogic for NimGameLogic {
    type PID = NimPlayerId;
    type Move = NimMove;
    type State = NimState;
    type MaskedState = NimState;

    fn init(&self, players: Vec<NimPlayerId>) -> (Self::State, HashSet<NimPlayerId>) {
        assert!(players.len() == 2);
        (
            NimState {
                pile_size: self.initial_pile_size,
                players: players.iter().copied().collect(),
            },
            HashSet::from([players[0]]),
        )
    }

    fn apply_moves(
        &self,
        state: Self::State,
        moves: HashMap<Self::PID, Self::Move>,
    ) -> MoveResult<Self::State, Self::PID> {
        let mut moves_iter = moves.iter();

        match (moves_iter.next(), moves_iter.next()) {
            (Some((&player, player_move)), None) => {
                if player_move.amount > self.max_takes || player_move.amount == 0 {
                    return MoveResult::GameOver(vec![(player, -1)].into_iter().collect());
                }
                let next_player = state.next_player(player);
                match state.pile_size.cmp(&player_move.amount) {
                    Ordering::Less => {
                        MoveResult::GameOver(vec![(player, -1)].into_iter().collect())
                    }
                    Ordering::Equal => {
                        MoveResult::GameOver(vec![(player, 1)].into_iter().collect())
                    }
                    Ordering::Greater => MoveResult::NextState(
                        NimState {
                            pile_size: state.pile_size - player_move.amount,
                            players: state.players,
                        },
                        HashSet::from([next_player]),
                    ),
                }
            }
            _ => {
                panic!("not exactly one player tried to make a move")
            }
        }
    }

    fn mask_state(&self, state: &Self::State, _player: NimPlayerId) -> Self::MaskedState {
        state.clone()
    }
}

#[test]
fn standard_nim_game() {
    let nim_logic = NimGameLogic {
        initial_pile_size: 10,
        max_takes: 4,
    };
    let first_player = NimPlayerId(0);
    let second_player = NimPlayerId(1);
    let (mut nim_state, _) = nim_logic.init(vec![first_player, second_player]);
    let mut current_player: NimPlayerId;
    match nim_logic
        .make_move(nim_state, first_player, NimMove { amount: 3 })
        .unwrap()
    {
        MoveResultSingleAction::GameOver(_) => panic!(),
        MoveResultSingleAction::NextState(next_state, next_player) => {
            (nim_state, current_player) = (next_state, next_player)
        }
    };
    assert!(nim_state.pile_size == 7);
    assert!(current_player == second_player);
    match nim_logic
        .make_move(nim_state, current_player, NimMove { amount: 2 })
        .unwrap()
    {
        MoveResultSingleAction::GameOver(_) => panic!(),
        MoveResultSingleAction::NextState(next_state, next_player) => {
            (nim_state, current_player) = (next_state, next_player)
        }
    };
    assert!(nim_state.pile_size == 5);
    assert!(current_player == first_player);
    match nim_logic
        .make_move(nim_state, current_player, NimMove { amount: 1 })
        .unwrap()
    {
        MoveResultSingleAction::GameOver(_) => panic!(),
        MoveResultSingleAction::NextState(next_state, next_player) => {
            (nim_state, current_player) = (next_state, next_player)
        }
    };
    assert!(nim_state.pile_size == 4);
    assert!(current_player == second_player);
    match nim_logic
        .make_move(nim_state, current_player, NimMove { amount: 4 })
        .unwrap()
    {
        MoveResultSingleAction::GameOver(game_result) => {
            assert!(game_result[&second_player] == 1);
        }
        MoveResultSingleAction::NextState(_s, _p) => panic!(),
    };
}
pub enum MoveResultSingleAction<GameState, PID: PlayerId> {
    NextState(GameState, PID),
    GameOver(GameResult<PID>),
}

impl NimGameLogic {
    fn make_move(
        &self,
        state: NimState,
        player: NimPlayerId,
        player_move: NimMove,
    ) -> Option<MoveResultSingleAction<NimState, NimPlayerId>> {
        match self.apply_moves(state, HashMap::from([(player, player_move)])) {
            MoveResult::NextState(state, next_players) => Some(MoveResultSingleAction::NextState(
                state,
                if next_players.len() > 1 {
                    return None;
                } else {
                    next_players.iter().next()?.clone()
                },
            )),
            MoveResult::GameOver(game_result) => {
                Some(MoveResultSingleAction::GameOver(game_result))
            }
        }
    }
}

#[derive(Copy, Clone)]
struct NimPerfectAgent {
    mod_base: u32,
}

impl NimPerfectAgent {
    fn new(game: &NimGameLogic) -> Self {
        NimPerfectAgent {
            mod_base: game.max_takes + 1,
        }
    }
}

impl Agent for NimPerfectAgent {
    type Game = NimGameLogic;

    fn calculate_next_move(&self, new_state: NimState) -> NimMove {
        match new_state.pile_size % self.mod_base {
            0 => NimMove { amount: 1 },
            x => NimMove { amount: x },
        }
    }

    fn digest_state(&self, _new_state: <Self::Game as GameLogic>::MaskedState) {}
}

#[test]
fn test_agents() {
    for pile_size in 1..=300 {
        for max_takes in 1..=20 {
            let nim_logic = NimGameLogic {
                initial_pile_size: pile_size,
                max_takes: max_takes,
            };

            let agent_1 = NimPerfectAgent::new(&nim_logic);
            let agent_2 = NimPerfectAgent::new(&nim_logic);

            let agents: IndexMap<NimPlayerId, NimPerfectAgent> =
                vec![(NimPlayerId(1), agent_1), (NimPlayerId(2), agent_2)]
                    .into_iter()
                    .collect();

            use game_simulation::simulate_game;
            let result = simulate_game(&nim_logic, agents);
            let winner = if nim_logic.initial_pile_size % (nim_logic.max_takes + 1) == 0 {
                NimPlayerId(2)
            } else {
                NimPlayerId(1)
            };
            assert!(
                result.contains_key(&winner),
                "winner lost {}, {}, winner: {}",
                nim_logic.initial_pile_size,
                nim_logic.max_takes,
                winner.0
            );
            assert!(
                result[&winner] == 1,
                "winner won! {}, {}, {}",
                nim_logic.initial_pile_size,
                nim_logic.max_takes,
                winner.0
            );
        }
    }
}
