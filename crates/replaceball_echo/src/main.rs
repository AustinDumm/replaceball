
use std::{thread, time::Duration};

use rand::prelude::*;
use rand_distr::{Normal, Distribution};
use clap::{Parser, ValueEnum};

use replaceball_sim::{
    self,
    prelude::*,
};

mod avg;
mod sim;

#[derive(Clone, Debug, ValueEnum)]
enum Mode {
    Sim,
    Debug,
    Avg,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_enum, default_value_t = Mode::Sim)]
    mode: Mode,
}

fn main() {
    let args = Args::parse();

    let mut decider = RandomDecider::new();
    let game_record = replaceball_sim::simulate_game(
        &mut decider,
    );

    match args.mode {
        Mode::Sim => {
            let mut game_record = game_record;
            loop {
                sim::display_game(&game_record);

                thread::sleep(Duration::from_secs(1800));

                game_record = replaceball_sim::simulate_game(&mut decider);
            }
        },
        Mode::Debug => println!("{:#?}", game_record),
        Mode::Avg => {
            let games_count = 10_000;
            let averages = avg::sim_for_averages(games_count, &mut decider);

            println!("{:#?}", averages);
            println!("Home Win Pct: {}", averages.home_wins as f64 / (games_count as f64));
            println!("Runs / Game: {}", averages.runs as f64 / (games_count as f64 * 2.0));
            println!("Hits / Game: {}", averages.hits as f64 / (games_count as f64 * 2.0));
            println!("Double Play / Game: {}", averages.double_plays as f64 / (games_count as f64 * 2.0));
            println!("Triple Play / Game: {}", averages.triple_plays as f64 / (games_count as f64 * 2.0));
            println!("Strikeouts / Game: {}", averages.strikeouts as f64 / (games_count as f64 * 2.0));
            println!("Walks / Game: {}", averages.walks as f64 / (games_count as f64 * 2.0));
            println!("Singles / Game: {}", averages.singles as f64 / (games_count as f64 * 2.0));
            println!("Doubles / Game: {}", averages.doubles as f64 / (games_count as f64 * 2.0));
            println!("Triples / Game: {}", averages.triples as f64 / (games_count as f64 * 2.0));
            println!("Inside-the-park / Game: {}", averages.inside_the_park as f64 / (games_count as f64 * 2.0));
            println!("Home Runs / Game: {}", averages.home_runs as f64 / (games_count as f64 * 2.0));
            println!("At Bats / Game: {}", averages.total_at_bats as f64 / (games_count as f64 * 2.0));

            println!("Batting Average: {}", averages.hits as f64 / averages.total_at_bats as f64);
            println!("Slugging: {}", (averages.singles + averages.doubles * 2 + averages.triples * 3 + averages.home_runs * 4) as f64 / averages.total_at_bats as f64);

            let pitches = averages.balls + averages.strikes + averages.fouls;
            println!("Balls / Pitch: {}", averages.balls as f64 / pitches as f64);
            println!("Strikes / Pitch: {}", averages.strikes as f64 / pitches as f64);
            println!("Fouls / Pitch: {}", averages.fouls as f64 / pitches as f64);

        },
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

