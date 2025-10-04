use rand::Rng;

use game_logic::core::{Agent, GameLogic};
use game_logic::tournament::AgentFactory;

use super::game::{NimGameLogic, NimMove, NimState};

#[derive(Copy, Clone)]
pub struct NimPerfectAgent {
    mod_base: u32,
}

impl NimPerfectAgent {
    pub fn new(game: &NimGameLogic) -> Self {
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

pub struct NimRandomAgent {
    max_takes: u32,
}

impl NimRandomAgent {
    pub fn new(max_takes: u32) -> Self {
        NimRandomAgent { max_takes }
    }
}

impl Agent for NimRandomAgent {
    type Game = NimGameLogic;

    fn calculate_next_move(
        &self,
        _new_state: <Self::Game as GameLogic>::MaskedState,
    ) -> <Self::Game as GameLogic>::Move {
        let mut rng = rand::rng();
        NimMove {
            amount: rng.random_range(1..=self.max_takes),
        }
    }

    fn digest_state(&self, _new_state: <Self::Game as GameLogic>::MaskedState) {}
}

pub struct PerfectFactory {
    mod_base: u32,
}

impl PerfectFactory {
    pub fn new(mod_base: u32) -> Self {
        PerfectFactory { mod_base }
    }
}

impl AgentFactory for PerfectFactory {
    type Agent = NimPerfectAgent;
    fn create_agent(&self) -> Self::Agent {
        NimPerfectAgent {
            mod_base: self.mod_base,
        }
    }
}

pub struct RandomFactory {
    max_takes: u32,
}

impl RandomFactory {
    pub fn new(max_takes: u32) -> Self {
        RandomFactory { max_takes }
    }
}

impl AgentFactory for RandomFactory {
    type Agent = NimRandomAgent;
    fn create_agent(&self) -> Self::Agent {
        NimRandomAgent {
            max_takes: self.max_takes,
        }
    }
}
