use std::{collections::HashMap, hash::Hash};



pub trait PlayerID: Hash + Eq + Copy {

}

pub type GameResult<PID: PlayerID> = HashMap<PID, i32>;

pub enum MoveResult<GameState, PID: PlayerID> {
    NextState(GameState, PID),
    GameOver(GameResult<PID>),
}


pub trait GameLogic {
    type PID: PlayerID;
    type Move;
    type State;
    type MaskedState;
    
    fn init(&self, players: Vec<Self::PID>) -> (Self::State, Self::PID);
    
    fn make_move(&self, state: Self::State, player: Self::PID, player_move: Self::Move) -> MoveResult<Self::State, Self::PID>;
    
    fn mask_state(&self, state: &Self::State, player: Self::PID) -> Self::MaskedState;
}

pub trait Agent {
    type Game: GameLogic;

    // type State = <Self::Game as GameLogic>::MaskedState;
    // type Move = <Self::Game as GameLogic>::Move;

    fn new(game: &Self::Game) -> Self;

    fn digest_state(&self, new_state: &<Self::Game as GameLogic>::MaskedState);

    fn calculate_next_move(&self, new_state: &<Self::Game as GameLogic>::MaskedState, chosen_move: &mut Option<<Self::Game as GameLogic>::Move>);
}