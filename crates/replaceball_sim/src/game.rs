use crate::{
    prelude::*,
    inning::simulate_inning,
};

#[derive(Debug, Clone)]
pub struct GameRecord {
    pub innings: Box<[(InningRecord, GameProgress)]>,
    pub outcome: GameOutcome,
}

#[derive(Debug, Clone)]
pub struct GameOutcome {
    pub home_score: Score,
    pub away_score: Score,
}

pub type GameProgress = GameOutcome;


pub fn simulate_game(
    decider: &mut impl Decider
) -> GameRecord {
    let mut away_score: Score = 0;
    let mut away_batting_index: u8 = 0;

    let mut home_score: Score = 0;
    let mut home_batting_index: u8 = 0;

    let mut running_innings = Vec::<(InningRecord, GameProgress)>::new();

    for _ in 0..Consts::COUNT_INNINGS {
        let inning = simulate_inning(
            away_batting_index,
            home_batting_index,
            decider,
        );

        away_score += inning.outcome.away.runs_scored;
        home_score += inning.outcome.home.runs_scored;

        away_batting_index = (away_batting_index +
            inning.away.at_bats.len() as u8) %
            Consts::PLAYERS_PER_LINEUP;
        home_batting_index = (home_batting_index +
            inning.home.at_bats.len() as u8) %
            Consts::PLAYERS_PER_LINEUP;

        let progress = GameProgress { away_score, home_score };
        running_innings.push((inning, progress));
    }

    while away_score == home_score {
        let inning = simulate_inning(
            away_batting_index,
            home_batting_index,
            decider,
        );
        away_score += inning.outcome.away.runs_scored;
        home_score += inning.outcome.home.runs_scored;

        away_batting_index = (away_batting_index +
            inning.away.at_bats.len() as u8) %
            Consts::PLAYERS_PER_LINEUP;
        home_batting_index = (home_batting_index +
            inning.home.at_bats.len() as u8) %
            Consts::PLAYERS_PER_LINEUP;
        let progress = GameProgress { away_score, home_score };
        running_innings.push((inning, progress));
    }
    
    GameRecord {
        innings: running_innings.into_boxed_slice(),
        outcome: GameOutcome {
            home_score,
            away_score,
        },
    }
}

