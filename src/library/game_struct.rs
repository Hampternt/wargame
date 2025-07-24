pub struct GameStart {
    time: i32,
    money: i32,
}

impl GameStart {
    fn new(&mut self) {
        self.time = 0;
        self.money = 0;
    }
}
