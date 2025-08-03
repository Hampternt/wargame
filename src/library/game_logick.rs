use crate::library; 

pub fn next_turn(game: &mut library::game_struct::GameStart) {
    println!("Month: {month}, Year: {year}, century: {century}", month = game.get_month(), year = game.get_year(), century = game.get_century());
    game.game_time.pass_time();
}
