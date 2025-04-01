use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub trait PlayerID: Hash + Eq + Copy {}

pub type GameResult<PID: PlayerID> = HashMap<PID, i32>;

pub enum MoveResult<GameState, PID: PlayerID> {
    NextState(GameState, HashSet<PID>),
    GameOver(GameResult<PID>),
}

pub enum MoveResultSingleAction<GameState, PID: PlayerID> {
    NextState(GameState, PID),
    GameOver(GameResult<PID>),
}

pub trait GameLogic {
    type PID: PlayerID;
    type Move;
    type State;
    type MaskedState;

    fn init(&self, players: Vec<Self::PID>) -> (Self::State, HashSet<Self::PID>);

    fn apply_moves(
        &self,
        state: Self::State,
        moves: HashMap<Self::PID, Self::Move>,
    ) -> MoveResult<Self::State, Self::PID>;

    fn make_move(
        &self,
        state: Self::State,
        player: Self::PID,
        player_move: Self::Move,
    ) -> Option<MoveResultSingleAction<Self::State, Self::PID>> {
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
