#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{half_inning::simulate_half_inning, prelude::*};

#[derive(Clone, Debug, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InningRecord {
    pub away: HalfInningRecord,
    pub home: HalfInningRecord,
    pub outcome: InningOutcome,
}

#[derive(Clone, Debug, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InningOutcome {
    pub away: HalfInningOutcome,
    pub home: HalfInningOutcome,
}

pub fn simulate_inning(
    away_batting_index: u8,
    away_team: &Team,
    home_batting_index: u8,
    home_team: &Team,
    decider: &mut impl Decider,
) -> InningRecord {
    let away = simulate_half_inning(away_batting_index, away_team, home_team, decider);
    let home = simulate_half_inning(home_batting_index, home_team, away_team, decider);
    let outcome = InningOutcome {
        away: away.outcome.clone(),
        home: home.outcome.clone(),
    };

    InningRecord {
        away,
        home,
        outcome,
    }
}
