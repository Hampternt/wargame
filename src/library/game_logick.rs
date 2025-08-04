use std::process::Command;
use crate::library;
use crate::library::game_struct::GameStart;

pub fn next_turn(game: &mut library::game_struct::GameStart) {
    println!(
        "Month: {month}, Year: {year}, century: {century}",
        month = game.get_month(),
        year = game.get_year(),
        century = game.get_century()
    );
    game.game_time.pass_time();
}

pub fn print_time_format(game: &GameStart) {
    println!(
        "Next month is now, month {}, year {}",
        game.get_month(),
        game.get_year()
    );
}

pub fn clear_console() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/c", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}
