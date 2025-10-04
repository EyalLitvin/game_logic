// Framework tests using the Nim game as a test fixture
// This tests the core game_logic framework, not the Nim implementation itself

mod common;

use indexmap::IndexMap;
use game_logic::simulate_game;
use common::nim::{NimGameLogic, NimPerfectAgent, NimPlayerId};

#[test]
fn test_perfect_agent_wins_from_losing_position() {
    // When pile % (max_takes + 1) == 0, player 1 is in a losing position
    // Player 2 (perfect agent) should always win
    for pile_size in (6..=60).step_by(6) {
        for max_takes in [2, 5] {
            let game = NimGameLogic {
                initial_pile_size: pile_size,
                max_takes,
            };

            let agent_1 = NimPerfectAgent::new(&game);
            let agent_2 = NimPerfectAgent::new(&game);

            let agents: IndexMap<NimPlayerId, NimPerfectAgent> =
                [(NimPlayerId(1), agent_1), (NimPlayerId(2), agent_2)].into();

            let result = simulate_game(&game, agents);

            // When pile % (max_takes + 1) == 0, player 2 should win
            assert_eq!(
                result[&NimPlayerId(2)],
                1,
                "Player 2 should win with pile={}, max_takes={}",
                pile_size,
                max_takes
            );
            assert_eq!(
                result.get(&NimPlayerId(1)),
                None,
                "Player 1 should not be in results (didn't win)"
            );
        }
    }
}

#[test]
fn test_perfect_agent_wins_from_winning_position() {
    // When pile % (max_takes + 1) != 0, player 1 is in a winning position
    // Player 1 (perfect agent) should always win
    let test_cases = vec![
        (7, 3),   // 7 % 4 = 3
        (10, 3),  // 10 % 4 = 2
        (13, 4),  // 13 % 5 = 3
        (25, 6),  // 25 % 7 = 4
    ];

    for (pile_size, max_takes) in test_cases {
        let game = NimGameLogic {
            initial_pile_size: pile_size,
            max_takes,
        };

        let agent_1 = NimPerfectAgent::new(&game);
        let agent_2 = NimPerfectAgent::new(&game);

        let agents: IndexMap<NimPlayerId, NimPerfectAgent> =
            [(NimPlayerId(1), agent_1), (NimPlayerId(2), agent_2)].into();

        let result = simulate_game(&game, agents);

        assert_eq!(
            result[&NimPlayerId(1)],
            1,
            "Player 1 should win with pile={}, max_takes={}",
            pile_size,
            max_takes
        );
    }
}

#[test]
fn test_simulate_game_returns_correct_result_format() {
    let game = NimGameLogic {
        initial_pile_size: 10,
        max_takes: 3,
    };

    let agent_1 = NimPerfectAgent::new(&game);
    let agent_2 = NimPerfectAgent::new(&game);

    let agents: IndexMap<NimPlayerId, NimPerfectAgent> =
        [(NimPlayerId(1), agent_1), (NimPlayerId(2), agent_2)].into();

    let result = simulate_game(&game, agents);

    // Result should contain exactly one winner with score 1
    assert_eq!(result.len(), 1, "Result should contain exactly one entry");
    assert_eq!(
        *result.values().next().unwrap(),
        1,
        "Winner should have score 1"
    );
}

#[test]
fn test_game_terminates_in_reasonable_time() {
    // Test that games don't hang indefinitely
    let game = NimGameLogic {
        initial_pile_size: 100,
        max_takes: 5,
    };

    let agent_1 = NimPerfectAgent::new(&game);
    let agent_2 = NimPerfectAgent::new(&game);

    let agents: IndexMap<NimPlayerId, NimPerfectAgent> =
        [(NimPlayerId(1), agent_1), (NimPlayerId(2), agent_2)].into();

    // If this hangs, the test will timeout
    let result = simulate_game(&game, agents);

    assert!(result.len() > 0, "Game should produce a result");
}

#[test]
fn test_agents_initialized_correctly() {
    let game = NimGameLogic {
        initial_pile_size: 15,
        max_takes: 4,
    };

    // Just verify we can create agents with the game configuration
    let agent_1 = NimPerfectAgent::new(&game);
    let agent_2 = NimPerfectAgent::new(&game);

    let agents: IndexMap<NimPlayerId, NimPerfectAgent> =
        [(NimPlayerId(1), agent_1), (NimPlayerId(2), agent_2)].into();

    assert_eq!(agents.len(), 2, "Should have 2 agents");
}
