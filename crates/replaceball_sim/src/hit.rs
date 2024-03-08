#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{prelude::*, fielding};

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LaunchAngle(pub f64);

impl LaunchAngle {
    fn from_decider(decider: &mut impl Decider) -> Self {
        Self(decider.roll_stat(*levels::HIT_LAUNCH_ANGLE, Skill::default()))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// Everything is in feet per second
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Speed(pub f64);

impl Speed {
    fn from_decider(decider: &mut impl Decider) -> Self {
        Self(decider.roll_stat(*levels::HIT_EXIT_SPEED, Skill::default()))
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HitRecord {
    pub direction: HitDirection,
    pub launch_angle: LaunchAngle,
    pub exit_speed: Speed,

    pub outcome: HitOutcome,
}

#[derive(Clone, Debug, PartialEq)]
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
                BallLanding::Landed(_, _) => {
                    record.base_running_record.outcome.batter_hit_type
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    decider: &mut impl Decider,
    base_state: &[bool; 3],
) -> HitRecord {
    let direction = HitDirection::from_decider(decider);
    let launch_angle = LaunchAngle::from_decider(decider);
    let exit_speed = Speed::from_decider(decider);

    let launch_error = clamp(((launch_angle.0 - levels::HIT_LAUNCH_ANGLE.average).abs() - 2.0) / 200.0, 0.0..=0.25);
    let exit_speed = Speed(exit_speed.0 * (1.0 - launch_error));

    HitRecord {
        direction,
        launch_angle,
        exit_speed,
        outcome: fielding::simulate_fielding(
            direction,
            launch_angle,
            exit_speed,
            base_state,
            decider,
        ),
    }
}

