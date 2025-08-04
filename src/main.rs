mod library;
use crate::library::game_loop::main_game_loop;

fn main() {
    println!("Hello, world!");
    let mut game = library::game_start::game_start();

    /// Main game loop is here.
    main_game_loop(&mut game)
}
