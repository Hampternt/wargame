pub struct GameStart {
    pub time: i32,
    pub money: i32,
}

impl GameStart {
    pub fn new() -> GameStart {
        GameStart {
        time: 0,
        money: 0,
        }
    }
}
