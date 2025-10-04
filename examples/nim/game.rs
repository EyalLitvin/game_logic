use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use game_logic::core::{FinalScores, GameError, GameLogic, Id, MoveResult};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct NimPlayerId(pub u32);

impl Id for NimPlayerId {}

pub struct NimGameLogic {
    pub max_takes: u32,
    pub initial_pile_size: u32,
}

#[derive(Clone)]
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
        state: &mut Self::State,
        moves: HashMap<Self::PID, Self::Move>,
    ) -> Result<MoveResult<Self::PID>, GameError<Self::PID>> {
        let mut moves_iter = moves.iter();

        match (moves_iter.next(), moves_iter.next()) {
            (Some((&player, player_move)), None) => {
                // Validate move amount
                if player_move.amount > self.max_takes {
                    return Err(GameError::InvalidMove {
                        player,
                        reason: format!("Cannot take {} (max is {})", player_move.amount, self.max_takes),
                    });
                }
                if player_move.amount == 0 {
                    return Err(GameError::InvalidMove {
                        player,
                        reason: "Cannot take 0 matches".to_string(),
                    });
                }

                let next_player = state.next_player(player);

                match state.pile_size.cmp(&player_move.amount) {
                    Ordering::Less => {
                        Err(GameError::InvalidMove {
                            player,
                            reason: format!("Cannot take {} matches (only {} remaining)", player_move.amount, state.pile_size),
                        })
                    }
                    Ordering::Equal => {
                        // Player wins by taking the last match
                        state.pile_size = 0;
                        Ok(MoveResult::GameOver(vec![(player, 1)].into_iter().collect()))
                    }
                    Ordering::Greater => {
                        // Update state in-place
                        state.pile_size -= player_move.amount;
                        Ok(MoveResult::Continue(HashSet::from([next_player])))
                    }
                }
            }
            (None, None) => {
                Err(GameError::MissingMoves {
                    expected: HashSet::from_iter(state.players.iter().copied()),
                    got: HashSet::new(),
                })
            }
            _ => {
                Err(GameError::IllegalState(
                    "Multiple players attempted to move in a turn-based game".to_string()
                ))
            }
        }
    }

    fn legal_moves(&self, state: &Self::State, _player: Self::PID) -> Vec<Self::Move>
    where
        Self::Move: Clone,
    {
        (1..=self.max_takes.min(state.pile_size))
            .map(|amount| NimMove { amount })
            .collect()
    }

    fn mask_state(&self, state: &Self::State, _player: NimPlayerId) -> Self::MaskedState {
        state.clone()
    }
}
