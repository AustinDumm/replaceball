#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::Fielder;

pub struct Team {
    fielders: [Player; 9],
    batting_order: [Fielder; 9],
}

impl Default for Team {
    fn default() -> Self {
        Self {
            fielders: [
                Player {
                    name: None,
                    jersey_number: "0".to_string(),
                    ..Default::default()
                },
                Player {
                    name: None,
                    jersey_number: "1".to_string(),
                    ..Default::default()
                },
                Player {
                    name: None,
                    jersey_number: "2".to_string(),
                    ..Default::default()
                },
                Player {
                    name: None,
                    jersey_number: "3".to_string(),
                    ..Default::default()
                },
                Player {
                    name: None,
                    jersey_number: "4".to_string(),
                    ..Default::default()
                },
                Player {
                    name: None,
                    jersey_number: "5".to_string(),
                    ..Default::default()
                },
                Player {
                    name: None,
                    jersey_number: "6".to_string(),
                    ..Default::default()
                },
                Player {
                    name: None,
                    jersey_number: "7".to_string(),
                    ..Default::default()
                },
                Player {
                    name: None,
                    jersey_number: "8".to_string(),
                    ..Default::default()
                },
            ],
            batting_order: [
                Fielder::Catcher,
                Fielder::Pitcher,
                Fielder::FirstBase,
                Fielder::SecondBase,
                Fielder::ThirdBase,
                Fielder::Shortstop,
                Fielder::LeftFielder,
                Fielder::CenterFielder,
                Fielder::RightFielder,
            ],
        }
    }
}

impl Team {
    pub fn player_at_batting_index(&self, index: u8) -> &Player {
        &self.fielders[self.batting_order[index as usize] as usize]
    }

    pub fn pitcher(&self) -> &Player {
        &self.fielders[Fielder::Pitcher as usize]
    }
}

#[derive(Clone, Debug, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Player {
    pub name: Option<String>,
    pub jersey_number: String,

    pub pitch_height_bias: i8,
    pub pitch_width_bias: i8,
    pub pitch_strike_bias: i8,

    pub hitter_swing_on_ball_bias: i8,
    pub hitter_contact_on_ball_bias: i8,
    pub hitter_foul_on_ball_contact_bias: i8,

    pub hitter_swing_on_strike_bias: i8,
    pub hitter_contact_on_strike_bias: i8,
    pub hitter_foul_on_strike_contact_bias: i8,

    pub hitter_hit_direction_bias: i8,
    pub hitter_launch_angle_bias: i8,
    pub hitter_hit_speed_bias: i8,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            name: None,
            jersey_number: "0".to_string(),

            pitch_height_bias: 0,
            pitch_width_bias: 0,
            pitch_strike_bias: 0,

            hitter_swing_on_ball_bias: 0,
            hitter_contact_on_ball_bias: 0,
            hitter_foul_on_ball_contact_bias: 0,

            hitter_swing_on_strike_bias: 0,
            hitter_contact_on_strike_bias: 0,
            hitter_foul_on_strike_contact_bias: 0,

            hitter_hit_direction_bias: 0,
            hitter_launch_angle_bias: 0,
            hitter_hit_speed_bias: 0,
        }
    }
}
