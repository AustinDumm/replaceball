#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    inning::simulate_inning, player::Team, prelude::*
};

#[derive(Debug, Clone, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GameRecord {
    pub innings: Box<[(InningRecord, GameProgress)]>,
    pub outcome: GameOutcome,
}

#[derive(Debug, Clone, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GameOutcome {
    pub home_score: Score,
    pub home_hits: u16,
    pub away_score: Score,
    pub away_hits: u16,
}

pub type GameProgress = GameOutcome;


pub fn simulate_game(
    decider: &mut impl Decider
) -> GameRecord {
    simulate_game_with_teams(
        decider,
        &Default::default(),
        &Default::default(),
    )
}

pub fn simulate_game_with_teams(
    decider: &mut impl Decider,
    home_team: &Team,
    away_team: &Team,
) -> GameRecord {
    let mut away_score: Score = 0;
    let mut away_hits: u16 = 0;
    let mut away_batting_index: u8 = 0;

    let mut home_score: Score = 0;
    let mut home_hits: u16 = 0;
    let mut home_batting_index: u8 = 0;

    let mut running_innings = Vec::<(InningRecord, GameProgress)>::new();

    for _ in 0..Consts::COUNT_INNINGS {
        let inning = simulate_inning(
            away_batting_index,
            &away_team,
            home_batting_index,
            &home_team,
            decider,
        );

        away_score += inning.outcome.away.runs_scored;
        home_score += inning.outcome.home.runs_scored;
        away_hits += inning.outcome.away.total_hits as u16;
        home_hits += inning.outcome.home.total_hits as u16;

        away_batting_index = (away_batting_index +
            inning.away.at_bats.len() as u8) %
            Consts::PLAYERS_PER_LINEUP;
        home_batting_index = (home_batting_index +
            inning.home.at_bats.len() as u8) %
            Consts::PLAYERS_PER_LINEUP;

        let progress = GameProgress {
            away_score,
            away_hits,
            home_score,
            home_hits,
        };
        running_innings.push((inning, progress));
    }

    while away_score == home_score {
        let inning = simulate_inning(
            away_batting_index,
            &away_team,
            home_batting_index,
            &home_team,
            decider,
        );
        away_score += inning.outcome.away.runs_scored;
        home_score += inning.outcome.home.runs_scored;
        away_hits += inning.outcome.away.total_hits as u16;
        home_hits += inning.outcome.home.total_hits as u16;

        away_batting_index = (away_batting_index +
            inning.away.at_bats.len() as u8) %
            Consts::PLAYERS_PER_LINEUP;
        home_batting_index = (home_batting_index +
            inning.home.at_bats.len() as u8) %
            Consts::PLAYERS_PER_LINEUP;
        let progress = GameProgress {
            away_score,
            away_hits,
            home_score,
            home_hits,
        };
        running_innings.push((inning, progress));
    }
    
    GameRecord {
        innings: running_innings.into_boxed_slice(),
        outcome: GameOutcome {
            home_score,
            home_hits,
            away_score,
            away_hits,
        },
    }
}

