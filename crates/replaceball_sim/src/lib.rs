#![feature(variant_count)]

mod at_bat;
mod base_running;
mod consts;
mod fielding;
mod game;
mod half_inning;
mod hit;
mod inning;
pub mod levels;
pub mod location;
mod pitch;
mod player;
mod stat;

pub mod prelude {
    pub use crate::{
        at_bat::{AtBatOutcome, AtBatOutcomeType, AtBatProgress, AtBatRecord},
        base_running::{BaseMovement, BaseRunningOutcome, BaseRunningRecord, MoveType},
        consts::Consts,
        fielding::{BallLanding, Fielder, FieldingPlay, FieldingRecord},
        game::{GameOutcome, GameProgress, GameRecord},
        half_inning::{HalfInningOutcome, HalfInningProgress, HalfInningRecord},
        hit::{HitOutcome, HitRecord, HitType, LaunchAngle, Speed},
        inning::{InningOutcome, InningRecord},
        levels,
        location::{self, *},
        pitch::{PitchHeight, PitchLocation, PitchOutcome, PitchRecord, PitchWidth},
        player::{Player, Team},
        stat::{Skill, Stat},
        Decider, Score,
    };
}

use std::ops::Range;

pub use {game::simulate_game, game::simulate_game_with_teams, stat::*};

use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

pub struct ExternalDecider {}

impl ExternalDecider {
    pub fn new() -> Self {
        Self {}
    }
}

#[wasm_bindgen(module = "$lib/replaceball_deps")]
extern "C" {
    pub fn gen_range(start: u64, end: u64) -> u64;
    pub fn gen_usize_range(start: usize, end: usize) -> usize;
    pub fn gen_float_range(start: f64, end: f64) -> f64;
    pub fn gen_normal(avg: f64, std_dist: f64) -> f64;
}

use crate::prelude::*;
impl Decider for ExternalDecider {
    fn roll(&mut self, check: u64, count: u64, adjust: u64) -> bool {
        let probability = check + adjust;
        let roll = gen_range(0, count);

        roll < probability
    }

    fn roll_pitch_location(
        &mut self,
        pitch_height_bias: i8,
        pitch_width_bias: i8,
    ) -> PitchLocation {
        let zone_count = 3;
        let zone_size = std::i8::MAX as i16;
        let full_range = zone_count * zone_size;

        let width = match (gen_range(0, full_range as u64)
            .saturating_add_signed(pitch_width_bias as i64) as f32
            / zone_size as f32) as u8
        {
            0 => PitchWidth::Left,
            1 => PitchWidth::Center,
            2 => PitchWidth::Right,
            _ => unreachable!(),
        };

        let height = match (gen_range(0, full_range as u64)
            .saturating_add_signed(pitch_height_bias as i64) as f32
            / zone_size as f32) as u8
        {
            0 => PitchHeight::High,
            1 => PitchHeight::Middle,
            2 => PitchHeight::Low,
            _ => unreachable!(),
        };

        PitchLocation { width, height }
    }

    fn roll_index(&mut self, range: std::ops::Range<usize>) -> usize {
        gen_usize_range(range.start, range.end)
    }

    fn roll_stat(&mut self, stat: Stat, skill: Skill) -> f64 {
        let sample = gen_normal(
            stat.average * skill.average_multiplier + skill.average_shift,
            stat.std_dev * skill.std_dev_multiplier,
        );

        if sample < stat.range.0 {
            stat.range.0
        } else if stat.range.1 < sample {
            stat.range.1
        } else {
            sample
        }
    }

    fn flip(&mut self, probability: f64, bias: i8) -> bool {
        (gen_float_range(0.0, 1.0) + (bias as f64 / std::i8::MAX as f64)) < probability
    }

    fn roll_uniform(&mut self, range: std::ops::Range<f64>) -> f64 {
        gen_float_range(range.start, range.end)
    }
}

#[derive(Serialize, Deserialize)]
struct Test {
    x: i32,
    y: f32,
    name: String,
}

#[wasm_bindgen]
pub fn wasm_simulate_game() -> JsValue {
    let game = simulate_game(&mut ExternalDecider::new());

    serde_wasm_bindgen::to_value(&game).unwrap()
}

pub type Score = u16;
pub trait Decider {
    fn roll(&mut self, check: u64, count: u64, adjust: u64) -> bool;
    fn roll_pitch_location(&mut self, height_bias: i8, width_bias: i8) -> pitch::PitchLocation;
    fn roll_index(&mut self, range: Range<usize>) -> usize;

    fn flip(&mut self, probability: f64, bias: i8) -> bool;

    fn roll_uniform(&mut self, range: Range<f64>) -> f64;

    fn roll_stat(&mut self, stat: Stat, skill: Skill) -> f64;
    fn roll_std_dev_skill_stat(&mut self, stat: Stat, bias: i8) -> f64 {
        self.roll_stat(stat, Skill::std_dev_bias_skill(bias, stat))
    }
}

#[cfg(test)]
mod tests {}
