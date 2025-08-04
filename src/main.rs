use std::io::stdin;
mod library;

fn main() {
    println!("Hello, world!");
    let mut game = library::game_start::game_start();
    loop {
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

        match input.trim().parse::<i32>() {
            Ok(1) => {
                library::game_logick::next_turn(&mut game);
                println!(
                    "Next month is now, month {}, year {}",
                    game.get_month(),
                    game.get_year()
                );
            }
            _ => {
                println!("Something went wrong")
            }
        }
    }
}
