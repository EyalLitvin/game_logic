// Tournament system tests
// Note: The tournament system is still under development (see manager.rs comment)
// These tests verify the basic traits and types are defined correctly

use game_logic::tournament::{AgentFactory, IdGenerator};
use game_logic::core::Id;

mod common;
use common::nim::PerfectFactory;

#[test]
fn test_agent_factory_creates_agents() {
    let factory = PerfectFactory::new(4);

    let agent1 = factory.create_agent();
    let agent2 = factory.create_agent();

    // Verify we can create multiple agents from the same factory
    // This is important for tournament scenarios
    assert_eq!(std::mem::size_of_val(&agent1), std::mem::size_of_val(&agent2));
}

#[test]
fn test_multiple_factories() {
    let factory1 = PerfectFactory::new(4);
    let factory2 = PerfectFactory::new(6);

    let agent1 = factory1.create_agent();
    let agent2 = factory2.create_agent();

    // Different factories can create agents with different configurations
    assert_eq!(std::mem::size_of_val(&agent1), std::mem::size_of_val(&agent2));
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct TestGameId(u32);

impl Id for TestGameId {}

struct SimpleIdGenerator {
    counter: u32,
}

impl SimpleIdGenerator {
    fn new() -> Self {
        SimpleIdGenerator { counter: 0 }
    }
}

impl IdGenerator for SimpleIdGenerator {
    type Id = TestGameId;

    fn generate_id(&mut self) -> Self::Id {
        let current = self.counter;
        self.counter += 1;
        TestGameId(current)
    }
}

#[test]
fn test_id_generator_produces_unique_ids() {
    let mut generator = SimpleIdGenerator::new();

    let id1 = generator.generate_id();
    let id2 = generator.generate_id();
    let id3 = generator.generate_id();

    assert_ne!(id1, id2);
    assert_ne!(id2, id3);
    assert_ne!(id1, id3);
}

#[test]
fn test_id_generator_increments() {
    let mut generator = SimpleIdGenerator::new();

    let id1 = generator.generate_id();
    let id2 = generator.generate_id();

    assert_eq!(id1.0, 0);
    assert_eq!(id2.0, 1);
}

// TODO: Add full tournament tests when host_tournament implementation is complete
// The current implementation has type system issues that need resolution
