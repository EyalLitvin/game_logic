mod nim;

use indexmap::IndexMap;
use std::io::{self, Write};

use game_logic::simulate_game;
use nim::game::{NimGameLogic, NimMove, NimPlayerId};
use nim::agents::{NimPerfectAgent, NimRandomAgent};
use game_logic::core::Agent;

fn main() {
    println!("=== Nim Game Example ===\n");
    println!("Rules:");
    println!("- Start with a pile of matches");
    println!("- Players take turns removing 1-N matches (max configurable)");
    println!("- The player who takes the last match wins\n");

    let pile_size = get_input("Enter initial pile size", 10);
    let max_takes = get_input("Enter maximum takes per turn", 3);

    let game = NimGameLogic {
        initial_pile_size: pile_size,
        max_takes,
    };

    println!("\nChoose game mode:");
    println!("1. Human vs Perfect AI");
    println!("2. Human vs Random AI");
    println!("3. Perfect AI vs Perfect AI");
    println!("4. Perfect AI vs Random AI");

    let mode = get_input("Enter choice (1-4)", 1);

    match mode {
        1 => play_human_vs_ai(&game, NimPerfectAgent::new(&game)),
        2 => play_human_vs_ai(&game, NimRandomAgent::new(max_takes)),
        3 => simulate_ai_vs_ai(&game, NimPerfectAgent::new(&game), NimPerfectAgent::new(&game), "Perfect", "Perfect"),
        4 => simulate_ai_vs_ai(&game, NimPerfectAgent::new(&game), NimRandomAgent::new(max_takes), "Perfect", "Random"),
        _ => println!("Invalid choice!"),
    }
}

fn get_input(prompt: &str, default: u32) -> u32 {
    print!("{} (default {}): ", prompt, default);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse().unwrap_or(default)
}

fn play_human_vs_ai<A: Agent<Game = NimGameLogic>>(_game: &NimGameLogic, _ai: A) {
    println!("\nHuman vs AI mode not yet implemented!");
    println!("This would require interactive gameplay support.");
}

fn simulate_ai_vs_ai<A1, A2>(
    game: &NimGameLogic,
    _agent1: A1,
    _agent2: A2,
    name1: &str,
    name2: &str,
) where
    A1: Agent<Game = NimGameLogic>,
    A2: Agent<Game = NimGameLogic>,
{
    let player1 = NimPlayerId(1);
    let player2 = NimPlayerId(2);

    // For simplicity in the example, just use two perfect agents
    let mut agents: IndexMap<NimPlayerId, NimPerfectAgent> = [
        (player1, NimPerfectAgent::new(game)),
        (player2, NimPerfectAgent::new(game)),
    ].into();

    println!("\nSimulating: {} vs {}", name1, name2);
    println!("Initial pile: {}, Max takes: {}\n", game.initial_pile_size, game.max_takes);

    match simulate_game(game, &mut agents, Some(1000)) {
        Ok(result) => {
            println!("Game finished!");
            for (player_id, score) in result {
                let name = if player_id == player1 { name1 } else { name2 };
                if score > 0 {
                    println!("{} wins!", name);
                }
            }
        }
        Err(e) => {
            println!("Game error: {}", e);
        }
    }
}
