pub enum Consts {}

impl Consts {
    pub const COUNT_INNINGS: u8 = 9;
    pub const OUTS_PER_HALF_INNING: u8 = 3;

    pub const BALLS_PER_WALK: u8 = 4;
    pub const STRIKES_PER_STRIKEOUT: u8 = 3;

    pub const FIRST: usize = 0;
    pub const SECOND: usize = 1;
    pub const THIRD: usize = 2;
    pub const HOME: usize = 3;

    pub const PLAYERS_PER_LINEUP: u8 = 9;
}
