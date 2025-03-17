use std::{collections::HashMap, hash::Hash};


#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct PlayerID(pub u32);

pub type GameResult = HashMap<PlayerID, i32>;

pub enum MoveResult<GameState> {
    NextState(GameState, PlayerID),
    GameOver(GameResult),
}


pub trait GameLogic {
    type Move;
    type State;
    type MaskedState;
    
    fn init(&self, players: Vec<PlayerID>) -> (Self::State, PlayerID);
    
    fn make_move(&self, state: Self::State, player: PlayerID, player_move: Self::Move) -> MoveResult<Self::State>;
    
    fn mask_state(&self, state: &Self::State, player: PlayerID) -> Self::MaskedState;
}

pub trait Agent {
    type Game: GameLogic;

    // type State = <Self::Game as GameLogic>::MaskedState;
    // type Move = <Self::Game as GameLogic>::Move;

    fn new(game: &Self::Game) -> Self;

    fn digest_state(&self, new_state: &<Self::Game as GameLogic>::MaskedState);

    fn calculate_next_move(&self, new_state: &<Self::Game as GameLogic>::MaskedState, chosen_move: &mut Option<<Self::Game as GameLogic>::Move>);
}