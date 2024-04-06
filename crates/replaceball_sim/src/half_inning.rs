#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{at_bat::simulate_at_bat, player::Team, prelude::*};

#[derive(Clone, Debug, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HalfInningRecord {
    pub at_bats: Box<[(AtBatRecord, HalfInningProgress)]>,
    pub outcome: HalfInningOutcome,
}

#[derive(Default, Clone, Debug, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HalfInningProgress {
    pub bases: [bool; 3],
    pub score_change: Score,
    pub outs: u8,
}

#[derive(Clone, Debug, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HalfInningOutcome {
    pub runs_scored: Score,
    pub total_hits: u8,
}

pub fn simulate_half_inning(
    starting_index: u8,
    batting_team: &Team,
    fielding_team: &Team,
    decider: &mut impl Decider,
) -> HalfInningRecord {
    let mut state = HalfInningState::new();
    let mut at_bats = Vec::<(AtBatRecord, HalfInningProgress)>::new();

    let mut batting_index = starting_index;
    while state.outs_remaining > 0 {
        let at_bat_record = simulate_at_bat(
            batting_index,
            batting_team,
            fielding_team,
            decider,
            &state.bases,
        );

        match at_bat_record.outcome.outcome_type {
            AtBatOutcomeType::Hit(ref hit_record) => state.hit(&hit_record),
            AtBatOutcomeType::Walk => state.walk(batting_index),
            AtBatOutcomeType::Out => state.out(),
        };

        let progress = HalfInningProgress {
            bases: state.bases.map(|base| base.is_some()),
            score_change: state.runs_scored,
            outs: state.number_of_outs(),
        };

        at_bats.push((at_bat_record, progress));
        batting_index = (batting_index + 1) % Consts::PLAYERS_PER_LINEUP;
    }

    let outcome = HalfInningOutcome {
        runs_scored: state.runs_scored,
        total_hits: state.total_hits,
    };

    HalfInningRecord {
        at_bats: at_bats.into_boxed_slice(),
        outcome,
    }
}

struct HalfInningState {
    bases: [Option<u8>; 3],
    outs_remaining: u8,
    runs_scored: Score,
    total_hits: u8,
}

impl HalfInningState {
    fn new() -> Self {
        Self {
            bases: [None, None, None],
            outs_remaining: Consts::OUTS_PER_HALF_INNING,
            runs_scored: 0,
            total_hits: 0,
        }
    }

    fn number_of_outs(&self) -> u8 {
        Consts::OUTS_PER_HALF_INNING - self.outs_remaining
    }

    fn out(&mut self) {
        self.outs_remaining -= 1;
    }

    fn hit(&mut self, hit_record: &HitRecord) {
        match &hit_record.outcome {
            HitOutcome::HomeRun => self.home_run(),
            HitOutcome::InPlay(fielding) => {
                let fielding_record = &fielding;
                self.total_hits += match fielding_record.landing {
                    BallLanding::Out(_, _) => 0,
                    BallLanding::Landed(_, _) => 1,
                };

                let play_outcome = &fielding.base_running_record.outcome;
                self.outs_remaining = self.outs_remaining.saturating_sub(play_outcome.outs_made);
                self.runs_scored += play_outcome.runs_scored as u16;
                self.bases = play_outcome.ending_base_state;
            }
        }
    }

    fn walk(&mut self, batter_lineup_index: u8) {
        self.runs_scored += if self.bases.iter().all(|on| on.is_some()) {
            1
        } else {
            0
        };
        self.bases[Consts::THIRD] = if self.bases.iter().take(2).all(|on| on.is_some()) {
            self.bases[Consts::SECOND]
        } else {
            self.bases[Consts::THIRD]
        };
        self.bases[Consts::SECOND] =
            if self.bases.iter().take(1).all(|on| on.is_some()) { self.bases[Consts::FIRST] } else { self.bases[Consts::SECOND] };

        self.bases[Consts::FIRST] = Some(batter_lineup_index);
    }

    fn home_run(&mut self) {
        self.total_hits += 1;
        self.runs_scored += self.bases.iter().filter(|b| b.is_some()).count() as u16 + 1;
        self.bases = [None, None, None];
    }
}
