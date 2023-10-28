use crate::{
    prelude::*,
    half_inning::{ HalfInningRecord, simulate_half_inning },
};

#[derive(Clone, Debug)]
pub struct InningRecord {
    pub away: HalfInningRecord,
    pub home: HalfInningRecord,
    pub outcome: InningOutcome,
}

#[derive(Clone, Debug)]
pub struct InningOutcome {
    pub away: HalfInningOutcome,
    pub home: HalfInningOutcome,
}

pub fn simulate_inning(
    away_batting_index: u8,
    home_batting_index: u8,
    decider: &mut impl Decider
) -> InningRecord {
    let away = simulate_half_inning(
        away_batting_index,
        decider,
    );
    let home = simulate_half_inning(
        home_batting_index,
        decider,
    );
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

