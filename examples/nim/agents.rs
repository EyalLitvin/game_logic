use rand::Rng;

use game_logic::core::Agent;
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

impl Agent<NimGameLogic> for NimPerfectAgent {
    fn calculate_next_move(&mut self, new_state: NimState) -> NimMove {
        match new_state.pile_size % self.mod_base {
            0 => NimMove { amount: 1 },
            x => NimMove { amount: x },
        }
    }

    fn digest_state(&mut self, _new_state: NimState) {}
}

pub struct NimRandomAgent {
    max_takes: u32,
}

impl NimRandomAgent {
    pub fn new(max_takes: u32) -> Self {
        NimRandomAgent { max_takes }
    }
}

impl Agent<NimGameLogic> for NimRandomAgent {
    fn calculate_next_move(&mut self, new_state: NimState) -> NimMove {
        let mut rng = rand::rng();
        NimMove {
            amount: rng.random_range(1..=self.max_takes.min(new_state.pile_size)),
        }
    }

    fn digest_state(&mut self, _new_state: NimState) {}
}

pub struct PerfectFactory {
    mod_base: u32,
}

impl PerfectFactory {
    pub fn new(mod_base: u32) -> Self {
        PerfectFactory { mod_base }
    }
}

impl AgentFactory<NimGameLogic> for PerfectFactory {
    fn create_agent(&self) -> Box<dyn Agent<NimGameLogic> + Send> {
        Box::new(NimPerfectAgent {
            mod_base: self.mod_base,
        })
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

impl AgentFactory<NimGameLogic> for RandomFactory {
    fn create_agent(&self) -> Box<dyn Agent<NimGameLogic> + Send> {
        Box::new(NimRandomAgent {
            max_takes: self.max_takes,
        })
    }
}
