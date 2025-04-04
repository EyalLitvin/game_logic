# game_logic
A crate for simulating turn-based games with customizable game logic and agent behavior. 

## Usage
This crate provides traits for implementing your own turn-based games.

First, implement a `gameMove` and a `gameState` types. these types should represent a stateless frozen move in the game or a state of the game (respectively). then, you can create a `gameLogic` type. 

Such a type is responsible for managing a game. you should think of the type itself as the general game, and every instance of it as a "variant" of the game.

For example, you could have a `NimGameLogic`[^1] type (as is implemented in the tests), and every instance would have a specific initial pile size, and maximum "takes" per turn.

[^1]: This is not the classic universal combinatorial game (due to the Sprague-Grundy theorem), but a version where every turn you are allowed to take at most some specified amount of matches.

Once these are implemented, you will have to implement an `Agent` for your game - and then you can simulate the game using `game_logic::game_simulation::simulate_game`.

## The game process
Every game goes like this:
1. Some initial game state is provided by the `GameLogic::init` method
2. The `init` method also outputs the set of players who need to make a move
3. Then, every agent is either
    a. Notified of the current state, using the `Agent::digest_state` method
    b. Or is queried for a move, using the `Agent::calculate_next_move` method. this method is also implicitly in charge of notifying the agent of the new state
4. We now give the game object the current state and the relevant moves, and we get the new state, if the game did not end. if it did, we receive the game result.

This process should be flexible enough for almost any turn-based games.

Notice that each agent is getting a "masked state" - perhaps there are some details of the state that the game should be aware of, but some (or all) of the agents shouldn't be.

## Looking forward
This crate still requires some work. mainly:
- Creating way more examples
- Creating more and refactoring the tests
- Creating more ergonomic error handling (instead of making the implementation of `GameLogic` be fully in charge of it)
- Using this library for future projects[^2]:
    - creating an infrastructure for hosting bot tournaments
    - creating an infrastructure for hosting a game between human players over the net (using CLI or rendering to screen using something like bevy)
- Adding docstrings to all functions

[^2]: These projects practically amount to creating a generic implementation for the agent object, together with some wrapper around the simulation function. they will be linked here when created.

## A bit about me
I am very new to rust, and this is one of my first actual projects. Obviously I have a lot to learn about rust and development in general. Every criticism is welcome :)