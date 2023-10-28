use crate::Stat;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref BALLS_PER_PITCH: f64 =
        TOTAL_BALLS as f64 / TOTAL_PITCHES as f64;

    pub static ref SWINGS_PER_BALL: f64 =
        TOTAL_BALL_SWINGS as f64 / TOTAL_BALLS as f64;

    pub static ref CONTACTS_PER_BALL_SWING: f64 =
        TOTAL_BALL_SWING_CONTACTS as f64 / TOTAL_BALL_SWINGS as f64;

    pub static ref FOULS_PER_BALL_CONTACT: f64 =
        TOTAL_BALL_SWING_CONTACT_FOULS as f64 / TOTAL_BALL_SWING_CONTACTS as f64;

    pub static ref SWINGS_PER_STRIKE: f64 =
        TOTAL_STRIKE_SWINGS as f64 / TOTAL_STRIKES as f64;

    pub static ref CONTACTS_PER_STRIKE_SWING: f64 =
        TOTAL_STRIKE_SWING_CONTACTS as f64 / TOTAL_STRIKE_SWINGS as f64;

    pub static ref FOULS_PER_STRIKE_CONTACT: f64 =
        TOTAL_STRIKE_SWING_CONTACT_FOULS as f64 / TOTAL_STRIKE_SWING_CONTACTS as f64;

    pub static ref MIN_HOME_RUN_LAUNCH_PERCENTAGE: f64 = 1u64 as f64 / 5 as f64;
    pub static ref MAX_HOME_RUN_LAUNCH_PERCENTAGE: f64 = 5u64 as f64 / 9 as f64;
    pub static ref MIN_HOME_RUN_VELOCITY: f64 = 11.0f64 / 8 as f64;

    static ref HIT_AVERAGE_SPEED: f64 = 125.0;
    pub static ref HIT_EXIT_SPEED: Stat = Stat {
        average: *HIT_AVERAGE_SPEED,
        std_dev: 35.0,
        range: (0.0, 2.0 * *HIT_AVERAGE_SPEED),
    };

    pub static ref HIT_LAUNCH_ANGLE: Stat = Stat {
        average: 0.0,
        std_dev: 45.0,
        range: (-90.0, 90.0),
    };

    /// In feet per second
    pub static ref BASERUNNER_SPEED: Stat = Stat {
        average: 27.0,
        std_dev: 4.0,
        range: (0.0, 40.0),
    };

    pub static ref FIELDER_SPEED: Stat = Stat {
        average: 23.0,
        std_dev: 4.0,
        range: (0.0, 40.0),
    };

    pub static ref THROW_SPEED: Stat = Stat {
        average: 57.25,
        std_dev: 7.0,
        range: (0.0, 150.0),
    };

    /// In Seconds
    pub static ref PLAYER_REACTION_TIME: Stat = Stat{
        average: 1.85,
        std_dev: 0.1,
        range: (1.25, 2.5),
    };

    pub static ref FIELDER_TRANSFER_TIME: Stat = Stat {
        average: 1.5,
        std_dev: 0.2,
        range: (1.0, 3.0),
    };

    pub static ref BOX_EXIT_TIME: Stat = Stat {
        average: 2.0,
        std_dev: 0.3,
        range: (1.5, 3.5),
    };

    pub static ref BASE_TAKEOFF_DELAY: Stat = Stat {
        average: 1.0,
        std_dev: 0.3,
        range: (0.75, 2.0),
    };
}

const TOTAL_PITCHES: u32 = 10_894_935;
const TOTAL_BALLS: u32 = 5_359_317;
const TOTAL_BALL_SWINGS: u32 = 1_593_720;
const TOTAL_BALL_SWING_CONTACTS: u32 = 983_462;
const TOTAL_BALL_SWING_CONTACT_FOULS: u32 = 550_032;
const TOTAL_STRIKES: u32 = TOTAL_PITCHES - TOTAL_BALLS;
const TOTAL_STRIKE_SWINGS: u32 = 3_468_221;
const TOTAL_STRIKE_SWING_CONTACTS: u32 = 2_905_144;
const TOTAL_STRIKE_SWING_CONTACT_FOULS: u32 = 1_363_718;

pub const LAUNCH_STD_DEV: f64 = 0.35;
pub const LAUNCH_OFFSET: u64 = 10;
pub const EXIT_VELOCITY_STD_DEV: f64 = 0.35;
