use std::{
    io::Write,
    time::Duration, sync::mpsc::Sender,
};

use crossterm::event::{self, Event, KeyCode};
use display::{DisplayModel, AtBat, Score};
use replaceball_sim::prelude::*;

mod display;

fn main() -> std::io::Result<()> {
    let mut stderr = std::io::stderr();
    let (sender, join_handle) = display::Display::start(std::io::stdout());

    loop {
        match poll_esc(Duration::from_secs(1)) {
            Ok(true) => break,
            Ok(false) => (),
            Err(e) => {
                write!(stderr, "Error polling: {}", e)?;
                break;
            },
        }

        if let Err(e) = sender.send(Some(DisplayModel {
            score: Score {
                away: 0,
                home: 0,
            },
            at_bat: AtBat {
                strikes: 0,
                balls: 0,
                outs: 0,
                base_state: [false, false, false],
            },
        })) {
            write!(stderr, "Failed to send model: {}", e)?;
            break;
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

fn sim_game(record: &GameRecord, display_sender: Sender<Option<DisplayModel>>) -> std::io::Result<()> {
    let mut away_score = 0u16;
    let mut home_score = 0u16;
    for (game_record, _game_outcome) in record.innings.iter() {
        escapable_wait!(Duration::from_secs(1));

        let mut game_display_model = DisplayModel {
            score: Score {
                home: 0,
                away: 0,
            },
            at_bat: AtBat {
                strikes: 0,
                balls: 0,
                outs: 0,
                base_state: [false, false, false],
            }
        };
        for (at_bat_record, inning_progress) in game_record.away.at_bats.iter() {
            for (_pitch_record, at_bat_progress) in at_bat_record.pitches.iter() {
                game_display_model.at_bat.strikes = at_bat_progress.strikes;
                game_display_model.at_bat.balls = at_bat_progress.balls;

                display_sender.send(Some(game_display_model.clone()))
                    .expect("Failed to send model for pitch");
                escapable_wait!(Duration::from_secs(1));
            }

            game_display_model.at_bat.outs = inning_progress.outs;
            game_display_model.at_bat.base_state = inning_progress.bases;
            game_display_model.score.away = away_score + inning_progress.score_change;

            display_sender.send(Some(game_display_model.clone()))
                .expect("Failed to send model for between batters");
            escapable_wait!(Duration::from_secs(1));
        }

        away_score = game_display_model.score.away;
        display_sender.send(Some(game_display_model.clone()))
            .expect("Failed to send model for between innings");
        escapable_wait!(Duration::from_secs(1));

        for (at_bat_record, inning_progress) in game_record.home.at_bats.iter() {
            for (_pitch_record, at_bat_progress) in at_bat_record.pitches.iter() {
                game_display_model.at_bat.strikes = at_bat_progress.strikes;
                game_display_model.at_bat.balls = at_bat_progress.balls;

                display_sender.send(Some(game_display_model.clone()))
                    .expect("Failed to send model for pitch");
                escapable_wait!(Duration::from_secs(1));
            }

            game_display_model.at_bat.outs = inning_progress.outs;
            game_display_model.at_bat.base_state = inning_progress.bases;
            game_display_model.score.home = home_score + inning_progress.score_change;

            display_sender.send(Some(game_display_model.clone()))
                .expect("Failed to send model for between batters");
            escapable_wait!(Duration::from_secs(1));
        }

        home_score = game_display_model.score.home;
        display_sender.send(Some(game_display_model.clone()))
            .expect("Failed to send model for between innings");
        escapable_wait!(Duration::from_secs(1));
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
