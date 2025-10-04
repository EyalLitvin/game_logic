use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use game_logic::core::{GameLogic, GameResult, Id, MoveResult};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct NimPlayerId(pub u32);

impl Id for NimPlayerId {}

pub struct NimGameLogic {
    pub max_takes: u32,
    pub initial_pile_size: u32,
}

pub struct NimMove {
    pub amount: u32,
}

#[derive(Clone, Debug)]
pub struct NimState {
    pub pile_size: u32,
    pub players: Vec<NimPlayerId>,
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
