use std::fmt;
use indexmap::IndexMap;

use crate::core::{Agent, FinalScores, GameError, GameLogic, Id, MoveResult};

/// Errors that can occur during game simulation.
#[derive(Debug)]
pub enum SimulationError<PID: Id + fmt::Debug> {
    /// The maximum number of turns was exceeded without the game ending.
    MaxTurnsExceeded(usize),
    /// A game error occurred during simulation.
    GameError(GameError<PID>),
}

impl<PID: Id + fmt::Debug> fmt::Display for SimulationError<PID> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimulationError::MaxTurnsExceeded(max) => {
                write!(f, "Game exceeded maximum of {} turns without ending", max)
            }
            SimulationError::GameError(e) => write!(f, "Game error: {}", e),
        }
    }
}

impl<PID: Id + fmt::Debug> std::error::Error for SimulationError<PID> {}

/// Simulates a game using the provided game logic and agents.
///
/// # Arguments
/// * `game` - A reference to the game logic that defines the rules of the game.
/// * `agents` - A mutable mapping of player IDs to their respective agents that will play the game.
/// * `max_turns` - Optional maximum number of turns before the simulation terminates with an error.
///
/// # Returns
/// A `Result` containing either:
/// - `Ok(FinalScores)` - The game ended normally with final scores
/// - `Err(SimulationError)` - The game exceeded max turns or encountered an error
///
/// # Examples
/// ```ignore
/// let game = MyGame::new();
/// let mut agents = indexmap![
///     PlayerId(1) => Agent1::new(),
///     PlayerId(2) => Agent2::new(),
/// ];
/// let result = simulate_game(&game, &mut agents, Some(1000))?;
/// ```
pub fn simulate_game<G, A>(
    game: &G,
    agents: &mut IndexMap<G::PID, A>,
    max_turns: Option<usize>,
) -> Result<FinalScores<G::PID>, SimulationError<G::PID>>
where
    G: GameLogic,
    A: Agent<Game = G>,
    G::PID: fmt::Debug,
{
    let (mut current_state, mut current_players) = game.init(agents.keys().copied().collect());
    let mut turn_count = 0;

    loop {
        // Check turn limit
        if let Some(max) = max_turns {
            if turn_count >= max {
                return Err(SimulationError::MaxTurnsExceeded(max));
            }
        }
        turn_count += 1;

        // Collect moves from active players and notify inactive players
        let player_moves = agents
            .iter_mut()
            .filter_map(|(&pid, agent_ref)| {
                if current_players.contains(&pid) {
                    Some((
                        pid,
                        agent_ref.calculate_next_move(game.mask_state(&current_state, pid)),
                    ))
                } else {
                    agent_ref.digest_state(game.mask_state(&current_state, pid));
                    None
                }
            })
            .collect();

        // Apply moves and check result
        match game.apply_moves(&mut current_state, player_moves) {
            Ok(MoveResult::GameOver(result)) => {
                return Ok(result);
            }
            Ok(MoveResult::Continue(players)) => {
                current_players = players;
            }
            Err(e) => {
                return Err(SimulationError::GameError(e));
            }
        }
    }
}
