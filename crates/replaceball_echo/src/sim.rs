use std::{thread, time::Duration};
use itertools::Itertools;

use inflector::string::pluralize;
use replaceball_sim::prelude::*;

macro_rules! println_wait {
    () => {
        {
            println!(); // Handle the no-arguments case like println!
            wait(); // Call the "wait" function
        }
    };
    ($($arg:tt)*) => {
        {
            println!($($arg)*); // Use the println! macro for other cases
            wait(); // Call the "wait" function
        }
    };
}

pub fn display_game(record: &GameRecord) {
    println_wait!("==== Away vs Home ====");

    let mut last_progress: Option<GameProgress> = None;
    for (index, (record, progress)) in record.innings.iter().enumerate() {
        let (away, home) = if let Some(progress) = &last_progress {
            (progress.away_score, progress.home_score)
        } else {
            (0, 0)
        };

        let inning = index + 1;

        display_inning(record, inning, away, home);
        display_game_progress(progress, inning);
        
        last_progress = Some(progress.clone());
    }
}

fn display_game_progress(progress: &GameProgress, inning: usize) {
    println_wait!("End of {}", inning);
    println_wait!("Away: {}\nHome: {}", progress.away_score, progress.home_score);
}

fn display_inning(record: &InningRecord, inning: usize, home: u16, away: u16) {
    println_wait!("Top {}", inning);
    display_half_inning(&record.away);
    display_half_inning_outcome(&record.outcome.away);

    println_wait!("\nBottom {}", inning);
    println_wait!("Away: {}\nHome: {}", away + record.outcome.away.runs_scored, home);
    display_half_inning(&record.home);
    display_half_inning_outcome(&record.outcome.home);
}

fn display_half_inning(record: &HalfInningRecord) {
    display_half_inning_progress(&HalfInningProgress::default());
    for (at_bat, progress) in record.at_bats.iter() {
        println_wait!();
        display_at_bat(at_bat);
        println_wait!();
        display_half_inning_progress(progress);
        println_wait!();
    }
}

fn display_half_inning_progress(progress: &HalfInningProgress) {
    let out_description = "Out";
    print!(
        "{} {}.",
        progress.outs,
        if progress.outs == 1 { out_description.to_owned() } else { pluralize::to_plural(out_description) }
    );
    let run_description = "run";
    println_wait!(
        " {} {} in.",
        progress.score_change,
        if progress.score_change == 1 { run_description.to_owned() } else { pluralize::to_plural(run_description) }
    );

    let base_description = if progress.bases.iter().all(|on| *on) {
        "Bases loaded".to_owned()
    } else if progress.bases.iter().all(|on| !on) {
        "Nobody on".to_owned()
    } else {
        format!(
            "Man on {}",
            progress
                .bases
                .iter()
                .enumerate()
                .map(|(index, on)| (match index {
                    0 => "First",
                    1 => "Second",
                    2 => "Third",
                    _ => unreachable!(),
                }, *on))
                .filter(|(_, on)| *on)
                .map(|(base, _)| base)
                .collect::<Vec<&str>>()
                .join(" and ")
        )
    };
    println_wait!("{}", base_description);
}

fn display_half_inning_outcome(outcome: &HalfInningOutcome) {
    let run_description = "Run";
    let hit_description = "Hit";

    println_wait!(
        "{} {} on {} {}",
        outcome.runs_scored,
        if outcome.runs_scored == 1 { run_description.to_owned() } else { pluralize::to_plural(run_description) },
        outcome.total_hits,
        if outcome.total_hits == 1 { hit_description.to_owned() } else { pluralize::to_plural(hit_description) },
    );
}

fn display_at_bat(at_bat: &AtBatRecord) {
    println_wait!("Now Batting: #{}", at_bat.batter_index + 1);

    display_at_bat_progress(&AtBatProgress::default());

    let mut peek = at_bat.pitches.iter().peekable();
    while let Some((pitch, progress)) = peek.next() {
        display_pitch(pitch);
        println_wait!();

        if peek.peek().is_some() {
            display_at_bat_progress(progress);
        }
    }

    match at_bat.outcome.outcome_type {
        AtBatOutcomeType::Hit(ref hit_record) => display_hit(&hit_record),
        AtBatOutcomeType::Out => println_wait!("Strikeout"),
        AtBatOutcomeType::Walk => println_wait!("Walk"),
    }
}

fn display_at_bat_progress(progress: &AtBatProgress) {
    let ball_description = "ball";
    let strike_description = "strike";
    println_wait!(
        "{} {} and {} {}",
        progress.balls,
        if progress.balls == 1 { ball_description.to_owned() } else { pluralize::to_plural(ball_description) },
        progress.strikes,
        if progress.strikes == 1 { strike_description.to_owned() } else { pluralize::to_plural(strike_description) },
    );
}

fn display_pitch(pitch: &PitchRecord) {
    let location = match (&pitch.location.height, &pitch.location.width) {
        (PitchHeight::High, PitchWidth::Left) => "high and outside",
        (PitchHeight::High, PitchWidth::Center) => "high",
        (PitchHeight::High, PitchWidth::Right) => "high and inside",
        (PitchHeight::Middle, PitchWidth::Left) => "outside",
        (PitchHeight::Middle, PitchWidth::Center) => "down the middle",
        (PitchHeight::Middle, PitchWidth::Right) => "middle in",
        (PitchHeight::Low, PitchWidth::Left) => "low and away",
        (PitchHeight::Low, PitchWidth::Center) => "low",
        (PitchHeight::Low, PitchWidth::Right) => "low and in",
    };
    print!("Pitch is {}.", location);

    let result = match pitch.outcome {
        PitchOutcome::Strike(false) => "Called strike.",
        PitchOutcome::Strike(true) => "Swings and misses.",
        PitchOutcome::Ball => "Ball.",
        PitchOutcome::Foul => "Fouled off.",
        PitchOutcome::Hit(_) => "Swings.",
    };
    println_wait!(" {}", result);
}

fn display_hit(record: &HitRecord) {
    let velocity_comment = if record.launch_angle.0 < 25.0 {
        if record.exit_speed.0 < 70.0 {
            " soft"
        } else if 110.0 < record.exit_speed.0 {
            " hard"
        } else {
            ""
        }
    } else {
        if record.exit_speed.0 < 70.0 {
            " looping"
        } else if 110.0 < record.exit_speed.0 {
            " deep"
        } else {
            ""
        }
    };

    let angle_comment = if record.launch_angle.0 < -10.0 {
        "chopper"
    } else if record.launch_angle.0 < 10.0 {
        "ground ball"
    } else if record.launch_angle.0 < 25.0 {
        "line drive"
    } else {
        "fly ball"
    };

    let hit_type = record.outcome.hit_type();
    let direction_comment = if record.launch_angle.0 < 10.0 {
        // Groundball
        if hit_type == HitType::Out {
            match record.direction.0 {
                angle if angle < 1.0 * (90.0 / 7.0) => "to third base",
                angle if angle < 2.0 * (90.0 / 7.0) => "to the left side of the infield",
                angle if angle < 3.0 * (90.0 / 7.0) => "to shortstop",
                angle if angle < 4.0 * (90.0 / 7.0) => "to the pitcher",
                angle if angle < 5.0 * (90.0 / 7.0) => "to second base",
                angle if angle < 6.0 * (90.0 / 7.0) => "to the right side of the infield",
                angle if angle < 7.0 * (90.0 / 7.0) => "to first base",
                _ => unreachable!(),
            }
        } else {
            match record.direction.0 {
                angle if angle < 1.0 * (90.0 / 7.0) => "past third base",
                angle if angle < 2.0 * (90.0 / 7.0) => "through the left side of the infield",
                angle if angle < 3.0 * (90.0 / 7.0) => "past shortstop",
                angle if angle < 4.0 * (90.0 / 7.0) => "past the pitcher",
                angle if angle < 5.0 * (90.0 / 7.0) => "past second base",
                angle if angle < 6.0 * (90.0 / 7.0) => "through the right side of the infield",
                angle if angle < 7.0 * (90.0 / 7.0) => "past first base",
                _ => unreachable!(),
            }
        }
    } else if record.launch_angle.0 < 15.0 {
        // Line Drive
        if hit_type == HitType::Out {
            match record.direction.0 {
                angle if angle < 1.0 * (90.0 / 7.0) => "to the third baseman",
                angle if angle < 2.0 * (90.0 / 7.0) => "into left field",
                angle if angle < 3.0 * (90.0 / 7.0) => "to the shortstop",
                angle if angle < 4.0 * (90.0 / 7.0) => "right back up the middle",
                angle if angle < 5.0 * (90.0 / 7.0) => "to the second baseman",
                angle if angle < 6.0 * (90.0 / 7.0) => "into right field",
                angle if angle < 7.0 * (90.0 / 7.0) => "to the first baseman",
                _ => unreachable!(),
            }
        } else {
            match record.direction.0 {
                angle if angle < 1.0 * (90.0 / 7.0) => "into the left field corner",
                angle if angle < 2.0 * (90.0 / 7.0) => "into left field",
                angle if angle < 3.0 * (90.0 / 7.0) => "into left center",
                angle if angle < 4.0 * (90.0 / 7.0) => "right back up the middle",
                angle if angle < 5.0 * (90.0 / 7.0) => "into right center",
                angle if angle < 6.0 * (90.0 / 7.0) => "into right field",
                angle if angle < 7.0 * (90.0 / 7.0) => "into the right field corner",
                _ => unreachable!(),
            }
        }
     } else {
        // Fly ball
        match record.direction.0 {
            angle if angle < 1.0 * (90.0 / 7.0) => "into the left field corner",
            angle if angle < 2.0 * (90.0 / 7.0) => "towards the left fielder",
            angle if angle < 3.0 * (90.0 / 7.0) => "into left center",
            angle if angle < 4.0 * (90.0 / 7.0) => "towards the center fielder",
            angle if angle < 5.0 * (90.0 / 7.0) => "into right center",
            angle if angle < 6.0 * (90.0 / 7.0) => "towards the right fielder",
            angle if angle < 7.0 * (90.0 / 7.0) => "into the right field corner",
            _ => unreachable!(),
        }
    };


    println_wait!(
        "That's a{} {} {}.",
        velocity_comment,
        angle_comment,
        direction_comment,
    );

    match &record.outcome {
        HitOutcome::HomeRun => println_wait!("Home Run"),
        HitOutcome::InPlay(fielding_record) => {
            match &fielding_record.landing {
                BallLanding::Out(fielder, _) => {
                    display_catch(fielder);
                },
                BallLanding::Landed(_, fielding_play) => {
                    display_fielding(fielding_play);
                },
            };

            display_baserunning(&fielding_record.base_running_record);
        },
    };
}

fn display_catch(fielder: &Fielder) {
    println_wait!("Caught by the {}", fielder);
}

fn display_fielding(play: &FieldingPlay) {
    println_wait!("Fielded by the {}.", play.from);
    println_wait!("Over to the {}.", play.to);
}

fn display_baserunning(record: &BaseRunningRecord) {
    for (_, out_at) in record.movements.iter()
        .map(|r| match r.bases_moved {
            MoveType::Out(at_base) => (Some(r.starting_base), at_base),
            MoveType::Advanced(_) => (None, 0),
        })
        .filter(|(starting, _)| starting.is_some())
        .sorted_by(|(_, l_at), (_, r_at)| r_at.cmp(l_at)) {
            let out_at_name = match out_at {
                Consts::FIRST => "First",
                Consts::SECOND => "Second",
                Consts::THIRD => "Third",
                Consts::HOME => "Home",
                _ => unreachable!(),
            };

            println_wait!("Out at {}", out_at_name);
        }

    for (from, advances) in record.movements.iter()
        .filter_map(|r| match r.bases_moved {
            MoveType::Out(_) => None,
            MoveType::Advanced(moved) => Some((r.starting_base, moved)),
        })
        .sorted_by(|(l_from, _), (r_from, _)| l_from.cmp(r_from)) {
            let (from_name, to_base) = match from {
                None => {
                    ("The batter", match advances {
                        1 => "first base",
                        2 => "second base",
                        3 => "third base",
                        _ => "home",
                    })
                },
                Some(Consts::FIRST) => {
                    ("Runner from first", match advances {
                        1 => "second base",
                        2 => "third base",
                        _ => "home",
                    })
                },
                Some(Consts::SECOND) => {
                    ("Runner from second", match advances {
                        1 => "third base",
                        _ => "home",
                    })
                },
                Some(Consts::THIRD) => {
                    ("Runner from third", "home")
                },
                _ => unreachable!(),
            };

            println_wait!("{} is safe at {}", from_name, to_base);
        }
}

fn wait() {
    thread::sleep(Duration::from_secs(1))
}

