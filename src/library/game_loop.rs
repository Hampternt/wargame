use std::io::stdin;
use crate::library::game_struct::GameStart;
use crate::library::game_logic::clear_console;
use crate::library::game_logic::print_time_format;

/// Main game loop function is here, it takes a active game state as a imput and then starts the
/// main turn loop.
pub fn main_game_loop(game: &mut GameStart) {
    loop {
        // clear's the console readies it for the game.
        clear_console();
        // prints the curent in game time. TODO maybe add how many month's has passed since start
        // as well?
        print_time_format(&game);
        println!("Provide input:");
        println!("1. Next month.");
        println!("2. Buy something.");
        println!("3. Sell something");

        println!("Provide input:");
        let mut input: String = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Something is wrong with your input.");
        println!("your input was this, regard: {input}");

        match input.trim() {
            "" => {
                next_turn(&mut game);
            }
            _ => match input.trim().parse::<i32>() {
                Ok(1) => {
                    print_time_format(&game);
                    next_turn(&mut game);
                }
                _ => {
                    println!("Something went wrong")
                }
            },
        }
    }
}
