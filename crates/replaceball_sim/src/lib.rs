#![feature(variant_count)]

mod game;
mod inning;
mod half_inning;
mod at_bat;
mod pitch;
mod hit;
mod fielding;
mod base_running;
mod consts;
mod stat;
pub mod levels;
pub mod location;

pub mod prelude {
    pub use
        crate::{
            game::{ GameRecord, GameProgress, GameOutcome, },
            inning::{ InningRecord, InningOutcome, },
            half_inning::{ HalfInningRecord, HalfInningProgress, HalfInningOutcome, },
            at_bat::{ AtBatRecord, AtBatProgress, AtBatOutcome, AtBatOutcomeType, },
            pitch::{ PitchRecord, PitchOutcome, PitchLocation, PitchHeight, PitchWidth, },
            hit::{ HitRecord, HitOutcome, LaunchAngle, Speed, HitType, },
            fielding::{ FieldingRecord, BallLanding, Fielder, FieldingPlay, },
            base_running::{ BaseRunningRecord, BaseRunningOutcome, BaseMovement, MoveType, },
            location::{ self, * },
            stat::{ Stat, Skill, },
            Score,
            Decider,
            consts::Consts,
            levels,
        };
}


use std::ops::Range;
use rand::prelude::*;
use rand_distr::Normal;

pub use {
    game::simulate_game,
    stat::*,
};

#[cfg(feature = "wasm")]
mod wasm {
    use super::*;

    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    pub fn simulate_game() -> JsValue {
        let game = simulate_game(&mut RandomDecider::new());

        serde_wasm_bindgen::to_value(&game).unwrap()
    }
}


pub type Score = u16;
pub trait Decider {
    fn roll(
        &mut self,
        check: u64,
        count: u64,
        adjust: u64,
    ) -> bool;
    fn roll_pitch_location(
        &mut self,
    ) -> pitch::PitchLocation;
    fn roll_index(
        &mut self,
        range: Range<usize>,
    ) -> usize;

    fn flip(
        &mut self,
        probability: f64,
    )-> bool;

    fn roll_uniform(
        &mut self,
        range: Range<f64>,
    ) -> f64;

    fn roll_stat(
        &mut self,
        stat: Stat,
        skill: Skill,
    ) -> f64;
}

pub struct RandomDecider {
    rand: ThreadRng,
}

impl RandomDecider {
    pub fn new() -> Self {
        Self { rand: thread_rng() }
    }
}

use crate::prelude::*;
impl Decider for RandomDecider {
    fn roll(
        &mut self,
        check: u64,
        count: u64,
        adjust: u64,
    ) -> bool {
        let probability = check + adjust;
        let roll = self.rand.gen_range(0..count);

        roll < probability
    }

    fn roll_pitch_location(&mut self) -> PitchLocation {
        let width = match self.rand.gen_range(0..3) {
            0 => PitchWidth::Left,
            1 => PitchWidth::Center,
            2 => PitchWidth::Right,
            _ => unreachable!(),
        };

        let height = match self.rand.gen_range(0..3) {
            0 => PitchHeight::High,
            1 => PitchHeight::Middle,
            2 => PitchHeight::Low,
            _ => unreachable!(),
        };

        PitchLocation { width, height, }
    }

    fn roll_index(
        &mut self,
        range: std::ops::Range<usize>,
    ) -> usize {
        self.rand.gen_range(range)
    }

    fn roll_stat(
        &mut self,
        stat: Stat,
        skill: Skill
    ) -> f64 {
        let distr = Normal::new(
            stat.average * skill.average_multiplier,
            stat.std_dev * skill.std_dev_multiplier,
        ).expect("Failed to create normal distribution");

        let sample = distr.sample(&mut self.rand);

        if sample < stat.range.0 {
            stat.range.0
        } else if stat.range.1 < sample {
            stat.range.1
        } else {
            sample
        }
    }

    fn flip(
        &mut self,
        probability: f64,
    )-> bool {
        self.rand.gen_range(0.0..1.0) < probability
    }

    fn roll_uniform(
        &mut self,
        range: std::ops::Range<f64>,
    ) -> f64 {
        self.rand.gen_range(range)
    }
}

#[cfg(test)]
mod tests {
}
