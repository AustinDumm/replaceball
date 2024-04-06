#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{fielding, player::Team, prelude::*};

#[derive(Clone, Copy, Debug, PartialEq, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LaunchAngle(pub f64);

impl LaunchAngle {
    const BIAS_MAX_MULT: f64 = 2.0;

    fn from_decider(decider: &mut impl Decider, bias: i8) -> Self {
        let bias_positive = bias as i16 - std::i8::MIN as i16 + 1;
        Self(decider.roll_stat(
            *levels::HIT_LAUNCH_ANGLE,
            Skill {
                average_multiplier: Self::BIAS_MAX_MULT
                    * (bias_positive as f64 / std::u8::MAX as f64),
                ..Default::default()
            },
        ))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/// Everything is in feet per second
pub struct Speed(pub f64);

impl Speed {
    const SPEED_MAX_BIAS_MULT: f64 = 1.333;
    const SPEED_MIN_BIAS_MULT: f64 = 0.667;

    fn from_decider(decider: &mut impl Decider, bias: i8) -> Self {
        let positive_bias = (bias as i16 - std::i8::MIN as i16) as u8;
        let mult_width = Self::SPEED_MAX_BIAS_MULT - Self::SPEED_MIN_BIAS_MULT;
        let mult =
            (positive_bias as f64 / std::u8::MAX as f64) * mult_width + Self::SPEED_MIN_BIAS_MULT;

        Self(decider.roll_stat(
            *levels::HIT_EXIT_SPEED,
            Skill {
                average_multiplier: mult,
                std_dev_multiplier: 1.0,
            },
        ))
    }
}

#[derive(Clone, Debug, PartialEq, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HitRecord {
    pub direction: HitDirection,
    pub launch_angle: LaunchAngle,
    pub exit_speed: Speed,

    pub outcome: HitOutcome,
}

#[derive(Clone, Debug, PartialEq, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HitOutcome {
    InPlay(FieldingRecord),
    HomeRun,
}

impl HitOutcome {
    pub fn hit_type(&self) -> HitType {
        match self {
            Self::HomeRun => HitType::HomeRun,
            Self::InPlay(record) => match record.landing {
                BallLanding::Out(_, _) => HitType::Out,
                BallLanding::Landed(_, _) => record.base_running_record.outcome.batter_hit_type,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HitType {
    Out,
    FieldersChoice,
    Single,
    Double,
    Triple,
    HomeRun,
}

pub fn simulate_hit(
    _pitch_location: &PitchLocation,
    _is_ball: bool,
    batter_lineup_index: u8,
    batting_team: &Team,
    fielding_team: &Team,
    decider: &mut impl Decider,
    base_state: &[Option<u8>; 3],
) -> HitRecord {
    let batter = batting_team.player_at_batting_index(batter_lineup_index);
    let direction = HitDirection::from_decider(decider, batter.hitter_hit_direction_bias);

    let launch_angle = LaunchAngle::from_decider(decider, batter.hitter_launch_angle_bias);
    let exit_speed = Speed::from_decider(decider, batter.hitter_hit_speed_bias);

    // Let misses on average launch angle drop exit speed to simulate missing the "sweet spot"
    let launch_error = clamp(
        ((launch_angle.0 - levels::HIT_LAUNCH_ANGLE.average).abs() - 2.0) / 200.0,
        0.0..=0.25,
    );
    let exit_speed = Speed(exit_speed.0 * (1.0 - launch_error));

    HitRecord {
        direction,
        launch_angle,
        exit_speed,
        outcome: fielding::simulate_fielding(
            direction,
            launch_angle,
            exit_speed,
            batter_lineup_index,
            base_state,
            decider,
        ),
    }
}
