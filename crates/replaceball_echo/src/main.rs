use std::{thread, time::Duration};

use clap::{Parser, ValueEnum};
use rand::prelude::*;
use rand_distr::{Distribution, Normal};

use replaceball_sim::{self, prelude::*};

use crate::avg::Avg;

mod avg;
mod sim;

#[derive(Clone, Debug, ValueEnum)]
enum Mode {
    Sim,
    Debug,
    Avg,
    AvgTeams,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_enum, default_value_t = Mode::AvgTeams)]
    mode: Mode,
}

fn main() {
    let args = Args::parse();

    let mut decider = RandomDecider::new();
    let game_record = replaceball_sim::simulate_game(&mut decider);

    match args.mode {
        Mode::Sim => {
            let mut game_record = game_record;
            loop {
                sim::display_game(&game_record);

                thread::sleep(Duration::from_secs(1800));

                game_record = replaceball_sim::simulate_game(&mut decider);
            }
        }
        Mode::Debug => println!("{:#?}", game_record),
        Mode::Avg => {
            let games_count = 10_000;
            let averages = avg::sim_for_averages(games_count, &mut decider);

            print_averages(averages, games_count);
        },
        Mode::AvgTeams => {
            let games_count = 10_000;
            let averages = avg::sim_for_averages_biased(games_count, &mut decider);

            print_averages(averages, games_count);
        },
    }

    fn print_averages(averages: Avg, games_count: u64) {
        println!("{:#?}", averages);
            println!(
                "Home Win Pct: {}",
                averages.home_wins as f64 / (games_count as f64)
            );
            println!(
                "Runs / Game: {}",
                averages.runs as f64 / (games_count as f64 * 2.0)
            );
            println!(
                "Hits / Game: {}",
                averages.hits as f64 / (games_count as f64 * 2.0)
            );
            println!(
                "Double Play / Game: {}",
                averages.double_plays as f64 / (games_count as f64 * 2.0)
            );
            println!(
                "Triple Play / Game: {}",
                averages.triple_plays as f64 / (games_count as f64 * 2.0)
            );
            println!(
                "Strikeouts / Game: {}",
                averages.strikeouts as f64 / (games_count as f64 * 2.0)
            );
            println!(
                "Walks / Game: {}",
                averages.walks as f64 / (games_count as f64 * 2.0)
            );
            println!(
                "Singles / Game: {}",
                averages.singles as f64 / (games_count as f64 * 2.0)
            );
            println!(
                "Doubles / Game: {}",
                averages.doubles as f64 / (games_count as f64 * 2.0)
            );
            println!(
                "Triples / Game: {}",
                averages.triples as f64 / (games_count as f64 * 2.0)
            );
            println!(
                "Inside-the-park / Game: {}",
                averages.inside_the_park as f64 / (games_count as f64 * 2.0)
            );
            println!(
                "Home Runs / Game: {}",
                averages.home_runs as f64 / (games_count as f64 * 2.0)
            );
            println!(
                "At Bats / Game: {}",
                averages.total_at_bats as f64 / (games_count as f64 * 2.0)
            );

            println!(
                "Batting Average: {}",
                averages.hits as f64 / averages.total_at_bats as f64
            );
            println!(
                "Slugging: {}",
                (averages.singles
                    + averages.doubles * 2
                    + averages.triples * 3
                    + averages.home_runs * 4) as f64
                    / averages.total_at_bats as f64
            );

            let pitches = averages.balls + averages.strikes + averages.fouls;
            println!("Balls / Pitch: {}", averages.balls as f64 / pitches as f64);
            println!(
                "Strikes / Pitch: {}",
                averages.strikes as f64 / pitches as f64
            );
            println!("Fouls / Pitch: {}", averages.fouls as f64 / pitches as f64);
    }
}

struct RandomDecider {
    rand: ThreadRng,
}

impl RandomDecider {
    fn new() -> Self {
        Self { rand: thread_rng() }
    }
}

impl Decider for RandomDecider {
    fn roll(&mut self, check: u64, count: u64, adjust: u64) -> bool {
        let probability = check + adjust;
        let roll = self.rand.gen_range(0..count);

        roll < probability
    }

    fn roll_pitch_location(
        &mut self,
        pitch_height_bias: i8,
        pitch_width_bias: i8,
    ) -> PitchLocation {
        let zone_count = 3;
        let zone_size = -(std::i8::MIN as i16);
        let full_range = zone_count * zone_size;

        let width = match (self.rand.gen_range(0..full_range as u64)
            .saturating_add_signed(pitch_width_bias as i64) as f32
            / zone_size as f32) as u8
        {
            0 => PitchWidth::Left,
            1 => PitchWidth::Center,
            2 | 3 => PitchWidth::Right,
            i => unreachable!("Value: {}", i),
        };

        let height = match (self.rand.gen_range(0..full_range as u64)
            .saturating_add_signed(pitch_height_bias as i64) as f32
            / zone_size as f32) as u8
        {
            0 => PitchHeight::High,
            1 => PitchHeight::Middle,
            2 | 3 => PitchHeight::Low,
            i => unreachable!("Value: {}", i),
        };

        PitchLocation { width, height }
    }

    fn roll_index(&mut self, range: std::ops::Range<usize>) -> usize {
        self.rand.gen_range(range)
    }

    fn roll_stat(&mut self, stat: Stat, skill: Skill) -> f64 {
        let distr = Normal::new(
            stat.average * skill.average_multiplier + skill.average_shift,
            stat.std_dev * skill.std_dev_multiplier,
        )
        .expect("Failed to create normal distribution");

        let sample = distr.sample(&mut self.rand);

        if sample < stat.range.0 {
            stat.range.0
        } else if stat.range.1 < sample {
            stat.range.1
        } else {
            sample
        }
    }

    fn flip(&mut self, probability: f64, bias: i8) -> bool {
        (self.rand.gen_range(0.0..1.0) + (bias as f64 / std::i8::MAX as f64 / 10.0)) < probability
    }

    fn roll_uniform(&mut self, range: std::ops::Range<f64>) -> f64 {
        self.rand.gen_range(range)
    }
}
