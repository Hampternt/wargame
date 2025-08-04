use std::io::stdin;
mod library;
use crate::library::game_logick;
use crate::library::game_logick::clear_console;
use crate::library::game_logick::next_turn;
use crate::library::game_logick::print_time_format;

fn main() {
    println!("Hello, world!");
    let mut game = library::game_start::game_start();

    loop {
        // clear's the console readies it for the game.
        library::game_logick::clear_console();
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
