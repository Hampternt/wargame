/// Game Start struct declares, time: i32, money: i32, exhange_rate: i32 and game_year: TimeValues.
pub struct GameStart {
    /// Time is the time value that is measures how many month's since game start.
    time: i32,
    /// Money is the curent value of money, and a rough idea of how much labour you can buy.
    money: i32,
    /// Money to labour exhange rate is how much labour per money or the value of money.
    exhange_rate: i32,
    /// Game year start.
    pub game_time: TimeValues,

    pub resoucres: 
}

impl GameStart {
    /// the new function engages a new instance of the Gamse start struct declaring time and money.
    pub fn new() -> GameStart {
        GameStart {
            time: 0,
            money: 0,
            exhange_rate: 1,
            game_time: TimeValues {
                months: 1,
                year: -50_000,
                century: -500,
            },
        }
    }

    /// returns the TimeValue year in i32 Value.
    pub fn get_year(&self) -> i32 {
        self.game_time.year
    }

    /// retrieves the Timevalue month in i32 Value.
    pub fn get_month(&self) -> i32 {
        self.game_time.months
    }

    /// retrieve the Timevalue century in i32 Value.
    pub fn get_century(&self) -> i32 {
        self.game_time.century
    }
}
/// Time values for time tracking
pub struct TimeValues {
    months: i32,
    year: i32,
    century: i32,
}

impl TimeValues {
    pub fn pass_time(&mut self) {
        if (self.months == 12) {
            self.months = 1;
            self.year += 1;
        } else if (self.months > 12) {
            println!("What the shit how do you get to month {}?", self.months);
            self.months = 1;
        } else if (self.months < 1) {
            println!(
                "Your time is before time itself how did this happen? month:{}",
                self.months
            );
            self.months = 1;
        } else {
            self.months += 1;
        }
    }
}

/// The resouces that is contained within a tile.
/// # the iron wood and coal resources are  valued between 0 to 100 to show the rate of resoucres
/// within a tile, populatin and tile size is declared and scales how much resources are available.
pub struct LandResouces {
    iron: i32,
    wood: i32,
    coal: i32,
    populatin: i32,
    /// Tile size number is measuered in square kilometer's. sq^2.
    tile_size: i32,
}
