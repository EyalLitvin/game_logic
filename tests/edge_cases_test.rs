// Edge case and error handling tests

mod common;

use indexmap::IndexMap;
use game_logic::{simulate_game, core::{GameLogic, MoveResult}};
use common::{NimGameLogic, NimMove, NimPerfectAgent, NimPlayerId};
use std::collections::HashMap;

#[test]
fn test_minimal_game_single_match() {
    // Edge case: game with only 1 match
    let game = NimGameLogic {
        initial_pile_size: 1,
        max_takes: 3,
    };

    let agent_1 = NimPerfectAgent::new(&game);
    let agent_2 = NimPerfectAgent::new(&game);

    let agents: IndexMap<NimPlayerId, NimPerfectAgent> =
        [(NimPlayerId(1), agent_1), (NimPlayerId(2), agent_2)].into();

    let result = simulate_game(&game, agents);

    // Player 1 goes first and takes the last match
    assert_eq!(result[&NimPlayerId(1)], 1);
}

#[test]
fn test_invalid_move_too_many() {
    // Test that taking more than max_takes results in a loss
    let game = NimGameLogic {
        initial_pile_size: 10,
        max_takes: 3,
    };

    let (state, _) = game.init(vec![NimPlayerId(1), NimPlayerId(2)]);

    // Try to take more than allowed
    let result = game.apply_moves(
        state,
        HashMap::from([(NimPlayerId(1), NimMove { amount: 5 })]),
    );

    match result {
        MoveResult::GameOver(scores) => {
            assert_eq!(scores[&NimPlayerId(1)], -1, "Player should lose for invalid move");
        }
        _ => panic!("Should be game over due to invalid move"),
    }
}

#[test]
fn test_invalid_move_zero() {
    // Test that taking 0 matches results in a loss
    let game = NimGameLogic {
        initial_pile_size: 10,
        max_takes: 3,
    };

    let (state, _) = game.init(vec![NimPlayerId(1), NimPlayerId(2)]);

    let result = game.apply_moves(
        state,
        HashMap::from([(NimPlayerId(1), NimMove { amount: 0 })]),
    );

    match result {
        MoveResult::GameOver(scores) => {
            assert_eq!(scores[&NimPlayerId(1)], -1, "Player should lose for taking 0");
        }
        _ => panic!("Should be game over due to invalid move"),
    }
}

#[test]
fn test_invalid_move_more_than_pile() {
    // Test that taking more matches than available results in a loss
    let game = NimGameLogic {
        initial_pile_size: 2,
        max_takes: 3,
    };

    let (state, _) = game.init(vec![NimPlayerId(1), NimPlayerId(2)]);

    let result = game.apply_moves(
        state,
        HashMap::from([(NimPlayerId(1), NimMove { amount: 3 })]),
    );

    match result {
        MoveResult::GameOver(scores) => {
            assert_eq!(scores[&NimPlayerId(1)], -1, "Player should lose for taking too many");
        }
        _ => panic!("Should be game over due to invalid move"),
    }
}

#[test]
fn test_exact_match_wins() {
    // Test that taking the exact remaining matches wins
    let game = NimGameLogic {
        initial_pile_size: 3,
        max_takes: 3,
    };

    let (state, _) = game.init(vec![NimPlayerId(1), NimPlayerId(2)]);

    let result = game.apply_moves(
        state,
        HashMap::from([(NimPlayerId(1), NimMove { amount: 3 })]),
    );

    match result {
        MoveResult::GameOver(scores) => {
            assert_eq!(scores[&NimPlayerId(1)], 1, "Player should win by taking last match");
        }
        _ => panic!("Should be game over with a win"),
    }
}

#[test]
fn test_state_masking_is_identity() {
    // In Nim, state masking should return the same state (perfect information game)
    let game = NimGameLogic {
        initial_pile_size: 10,
        max_takes: 3,
    };

    let (state, _) = game.init(vec![NimPlayerId(1), NimPlayerId(2)]);

    let masked_state_p1 = game.mask_state(&state, NimPlayerId(1));
    let masked_state_p2 = game.mask_state(&state, NimPlayerId(2));

    // Both players should see the same state
    assert_eq!(masked_state_p1.pile_size, state.pile_size);
    assert_eq!(masked_state_p2.pile_size, state.pile_size);
    assert_eq!(masked_state_p1.pile_size, masked_state_p2.pile_size);
}

#[test]
fn test_init_creates_correct_initial_state() {
    let game = NimGameLogic {
        initial_pile_size: 42,
        max_takes: 7,
    };

    let (state, active_players) = game.init(vec![NimPlayerId(1), NimPlayerId(2)]);

    assert_eq!(state.pile_size, 42, "Initial pile size should match");
    assert_eq!(state.players.len(), 2, "Should have 2 players");
    assert_eq!(active_players.len(), 1, "Only 1 player should be active initially");
    assert!(active_players.contains(&NimPlayerId(1)), "Player 1 should go first");
}

#[test]
fn test_game_alternates_players() {
    let game = NimGameLogic {
        initial_pile_size: 10,
        max_takes: 2,
    };

    let (mut state, mut active) = game.init(vec![NimPlayerId(1), NimPlayerId(2)]);

    // Player 1 moves
    assert_eq!(active.len(), 1);
    assert!(active.contains(&NimPlayerId(1)));

    match game.apply_moves(state.clone(), HashMap::from([(NimPlayerId(1), NimMove { amount: 1 })])) {
        MoveResult::NextState(next_state, next_active) => {
            state = next_state;
            active = next_active;
        }
        _ => panic!("Game shouldn't be over"),
    }

    // Player 2 moves
    assert_eq!(active.len(), 1);
    assert!(active.contains(&NimPlayerId(2)));

    match game.apply_moves(state.clone(), HashMap::from([(NimPlayerId(2), NimMove { amount: 1 })])) {
        MoveResult::NextState(next_state, next_active) => {
            state = next_state;
            active = next_active;
        }
        _ => panic!("Game shouldn't be over"),
    }

    // Back to Player 1
    assert_eq!(active.len(), 1);
    assert!(active.contains(&NimPlayerId(1)));
}
