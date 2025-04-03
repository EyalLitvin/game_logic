use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub trait PlayerId: Hash + Eq + Copy {}

pub type GameResult<PID> = HashMap<PID, i32>;

pub enum MoveResult<GameState, PID: PlayerId> {
    NextState(GameState, HashSet<PID>),
    GameOver(GameResult<PID>),
}

pub trait GameLogic {
    type PID: PlayerId;
    type Move;
    type State;
    type MaskedState;

    fn init(&self, players: Vec<Self::PID>) -> (Self::State, HashSet<Self::PID>);

    fn apply_moves(
        &self,
        state: Self::State,
        moves: HashMap<Self::PID, Self::Move>,
    ) -> MoveResult<Self::State, Self::PID>;

    fn mask_state(&self, state: &Self::State, player: Self::PID) -> Self::MaskedState;
}

pub trait Agent {
    type Game: GameLogic;

    // type State = <Self::Game as GameLogic>::MaskedState;
    // type Move = <Self::Game as GameLogic>::Move;

    fn digest_state(&self, new_state: <Self::Game as GameLogic>::MaskedState);

    fn calculate_next_move(
        &self,
        new_state: <Self::Game as GameLogic>::MaskedState,
    ) -> <Self::Game as GameLogic>::Move;
}
