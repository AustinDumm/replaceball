use std::{
    io::Write,
    time::Duration, sync::mpsc::Sender,
};
use rand::prelude::*;
use rand_distr::Normal;

use crossterm::event::{self, Event, KeyCode};
use replaceball_sim::prelude::*;

use display::{DisplayModel, AtBat, Score, EventRecord};
use ring_buffer::RingBuffer;

mod display;
mod ring_buffer;
mod event_description;

fn main() -> std::io::Result<()> {
    let (mut sender, join_handle) = display::Display::start(std::io::stdout());

    loop {
        let game = replaceball_sim::simulate_game(&mut RandomDecider::new());
        if let Err(e) = sim_game(
            &game,
            &mut sender
        ) {
            write!(std::io::stderr(), "Error while displaying game: {}", e)?;
        }

        if event::poll(Duration::from_secs(600))? {
            match event::read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Esc => break,
                    _ => (),
                },
                _ => (),
            }
        }
    }

    let close_send = sender.send(None);
    let display_result = join_handle.join().expect("Failed to join to display thread");

    match (close_send, display_result) {
        (Ok(_), Ok(_)) => Ok(()),
        (Err(e), Ok(_)) => panic!("Failed to send closing message to display thread: {}", e),
        (_, Err(e)) => Err(e),
    }
}

macro_rules! escapable_wait {
    ($wait_time:expr) => {
        if poll_esc($wait_time)? {
            return Ok(())
        }
    }
}

fn sim_game(record: &GameRecord, display_sender: &mut Sender<Option<DisplayModel>>) -> std::io::Result<()> {
    let mut away_score = 0u16;
    let mut home_score = 0u16;
    let mut game_display_model = DisplayModel {
        is_top: true,
        inning_index: 0,
        score: Score {
            home: 0,
            away: 0,
        },
        at_bat: AtBat {
            strikes: 0,
            balls: 0,
            outs: 0,
            base_state: [false, false, false],
        },
        event_record: EventRecord {
            event_list: RingBuffer::new(20),
        },
    };
    for (game_record, _game_outcome) in record.innings.iter() {
        escapable_wait!(Duration::from_secs(2));

        for (at_bat_record, inning_progress) in game_record.away.at_bats.iter() {
            update_event(
                &mut game_display_model,
                display_sender,
                event_description::at_bat_start(&at_bat_record),
                Duration::from_secs(2)
            )?;
            
            for (pitch_record, at_bat_progress) in at_bat_record.pitches.iter() {
                game_display_model.at_bat.strikes = at_bat_progress.strikes;
                game_display_model.at_bat.balls = at_bat_progress.balls;

                game_display_model.event_record.event_list.push_back(
                    event_description::pitch(&pitch_record)
                );
                display_sender.send(Some(game_display_model.clone()))
                    .expect("Failed to send model for pitch");
                escapable_wait!(Duration::from_secs(2));
            }

            update_event(
                &mut game_display_model,
                display_sender,
                event_description::at_bat_outcome(
                    &at_bat_record.outcome.outcome_type
                ),
                Duration::from_secs(2),
            )?;

            game_display_model.at_bat.outs = inning_progress.outs;
            game_display_model.at_bat.base_state = inning_progress.bases;
            game_display_model.score.away = away_score + inning_progress.score_change;

            match &at_bat_record.outcome.outcome_type {
                AtBatOutcomeType::Hit(hit_record) => {
                    match &hit_record.outcome {
                        HitOutcome::HomeRun => {
                            display_sender.send(Some(game_display_model.clone()))
                                .expect("Failed to send model for between batters");
                            escapable_wait!(Duration::from_secs(2));
                                    },
                        HitOutcome::InPlay(fielding_record) => {
                            update_event(
                                &mut game_display_model,
                                display_sender,
                                event_description::play_result(&fielding_record),
                                Duration::from_secs(2),
                            )?;
                        }
                    }
                },
                _ => {
                    display_sender.send(Some(game_display_model.clone()))
                        .expect("Failed to send model for between batters");
                    escapable_wait!(Duration::from_secs(2));
                }
            };

            game_display_model.at_bat.balls = 0;
            game_display_model.at_bat.strikes = 0;

        }

        away_score = game_display_model.score.away;
        display_sender.send(Some(game_display_model.clone()))
            .expect("Failed to send model for between innings");
        escapable_wait!(Duration::from_secs(3));
        game_display_model.is_top = !game_display_model.is_top;

        game_display_model.at_bat.strikes = 0;
        game_display_model.at_bat.balls = 0;
        game_display_model.at_bat.outs = 0;
        game_display_model.at_bat.base_state = [false, false, false];
        for (at_bat_record, inning_progress) in game_record.home.at_bats.iter() {
            update_event(
                &mut game_display_model,
                display_sender,
                event_description::at_bat_start(&at_bat_record),
                Duration::from_secs(2)
            )?;

            for (pitch_record, at_bat_progress) in at_bat_record.pitches.iter() {
                game_display_model.at_bat.strikes = at_bat_progress.strikes;
                game_display_model.at_bat.balls = at_bat_progress.balls;

                game_display_model.event_record.event_list.push_back(
                    event_description::pitch(&pitch_record)
                );

                display_sender.send(Some(game_display_model.clone()))
                    .expect("Failed to send model for pitch");
                escapable_wait!(Duration::from_secs(2));
            }

            update_event(
                &mut game_display_model,
                display_sender,
                event_description::at_bat_outcome(
                    &at_bat_record.outcome.outcome_type
                ),
                Duration::from_secs(2),
            )?;

            game_display_model.at_bat.outs = inning_progress.outs;
            game_display_model.at_bat.base_state = inning_progress.bases;
            game_display_model.score.home = home_score + inning_progress.score_change;

            match &at_bat_record.outcome.outcome_type {
                AtBatOutcomeType::Hit(hit_record) => {
                    match &hit_record.outcome {
                        HitOutcome::HomeRun => {
                            display_sender.send(Some(game_display_model.clone()))
                                .expect("Failed to send model for between batters");
                            escapable_wait!(Duration::from_secs(2));
                                    },
                        HitOutcome::InPlay(fielding_record) => {
                            update_event(
                                &mut game_display_model,
                                display_sender,
                                event_description::play_result(&fielding_record),
                                Duration::from_secs(2),
                            )?;
                        }
                    }
                },
                _ => {
                    display_sender.send(Some(game_display_model.clone()))
                        .expect("Failed to send model for between batters");
                    escapable_wait!(Duration::from_secs(2));
                }
            };

            game_display_model.at_bat.balls = 0;
            game_display_model.at_bat.strikes = 0;
        }

        home_score = game_display_model.score.home;
        display_sender.send(Some(game_display_model.clone()))
            .expect("Failed to send model for between innings");
        escapable_wait!(Duration::from_secs(2));
        game_display_model.is_top = !game_display_model.is_top;
        game_display_model.inning_index += 1;
    }

    Ok(())
}

fn poll_esc(duration: Duration) -> std::io::Result<bool> {
    if event::poll(duration)? {
        let event = event::read()?;

        if event == Event::Key(KeyCode::Esc.into()) {
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
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

fn update_event(
    display_model: &mut DisplayModel,
    display_sender: &mut Sender<Option<DisplayModel>>,
    event: String,
    delay: Duration
) -> std::io::Result<()> {
    _ = display_model.event_record.event_list.push_back(event);
    display_sender.send(Some(display_model.clone())).expect("Failed to send model.");
    escapable_wait!(delay);

    Ok(())
}
