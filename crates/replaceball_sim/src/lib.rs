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

pub use {
    game::simulate_game,
    stat::*,
};

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

#[cfg(test)]
mod tests {
}
