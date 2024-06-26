#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PitchOutcome {
    /// Bool is true if it was a swinging strike
    Strike(bool),
    Ball,
    Foul,

    /// Bool defines if the hit was on a pitched ball or not
    Hit(bool),
}

#[derive(PartialEq, Eq, Clone, Debug, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum PitchHeight {
    High,
    Middle,
    Low,
}

#[derive(PartialEq, Eq, Clone, Debug, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum PitchWidth {
    Left,
    Center,
    Right,
}

#[derive(Clone, Debug, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PitchLocation {
    pub height: PitchHeight,
    pub width: PitchWidth,
}

#[derive(Clone, Debug, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PitchRecord {
    pub location: PitchLocation,
    pub outcome: PitchOutcome,
}

pub fn simulate_pitch(
    decider: &mut impl Decider,
    batter: &Player,
    pitcher: &Player,
) -> PitchRecord {
    let location = decider.roll_pitch_location(pitcher.pitch_height_bias, pitcher.pitch_width_bias);

    if decider.flip(*levels::BALLS_PER_PITCH, pitcher.pitch_strike_bias) {
        // Is Ball
        let location =
            if location.height == PitchHeight::Middle && location.width == PitchWidth::Center {
                // A ball can't be down the middle
                PitchLocation {
                    height: PitchHeight::Low,
                    width: PitchWidth::Left,
                }
            } else {
                location
            };

        if decider.flip(*levels::SWINGS_PER_BALL, batter.hitter_swing_on_ball_bias.saturating_sub(pitcher.pitcher_swing_on_ball_bias)) {
            // Swings anyways

            if decider.flip(
                *levels::CONTACTS_PER_BALL_SWING,
                batter.hitter_contact_on_ball_bias.saturating_sub(pitcher.pitcher_contact_on_ball_bias),
            ) {
                // Makes contact

                if decider.flip(
                    *levels::FOULS_PER_BALL_CONTACT,
                    batter.hitter_foul_on_ball_contact_bias.saturating_sub(pitcher.pitcher_foul_on_ball_contact_bias),
                ) {
                    // Fouls it off
                    PitchRecord {
                        location,
                        outcome: PitchOutcome::Foul,
                    }
                } else {
                    // In play
                    PitchRecord {
                        location,
                        outcome: PitchOutcome::Hit(true),
                    }
                }
            } else {
                // Swinging Strike
                PitchRecord {
                    location,
                    outcome: PitchOutcome::Strike(true),
                }
            }
        } else {
            // Holds off
            PitchRecord {
                location,
                outcome: PitchOutcome::Ball,
            }
        }
    } else {
        // Is Strike

        if decider.flip(
            *levels::SWINGS_PER_STRIKE,
            batter.hitter_swing_on_strike_bias.saturating_sub(pitcher.pitcher_swing_on_strike_bias),
        ) {
            // Swung at the strike

            if decider.flip(
                *levels::CONTACTS_PER_STRIKE_SWING,
                batter.hitter_contact_on_strike_bias.saturating_sub(pitcher.pitcher_contact_on_strike_bias),
            ) {
                // Made contact

                if decider.flip(
                    *levels::FOULS_PER_STRIKE_CONTACT,
                    batter.hitter_foul_on_strike_contact_bias.saturating_sub(pitcher.pitcher_foul_on_strike_contact_bias),
                ) {
                    // Foul ball
                    PitchRecord {
                        location,
                        outcome: PitchOutcome::Foul,
                    }
                } else {
                    // Hit
                    PitchRecord {
                        location,
                        outcome: PitchOutcome::Hit(false),
                    }
                }
            } else {
                // Swing and a miss
                PitchRecord {
                    location,
                    outcome: PitchOutcome::Strike(true),
                }
            }
        } else {
            // Stood there
            PitchRecord {
                location,
                outcome: PitchOutcome::Strike(false),
            }
        }
    }
}
