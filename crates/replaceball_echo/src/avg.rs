use std::ops::{Add, AddAssign};

use replaceball_sim::prelude::*;

#[derive(Default, Debug)]
pub struct Avg {
    pub home_wins: u64,
    pub runs: u64,
    pub hits: u64,

    pub double_plays: u64,
    pub triple_plays: u64,

    pub strikeouts: u64,
    pub walks: u64,
    pub singles: u64,
    pub doubles: u64,
    pub triples: u64,
    pub inside_the_park: u64,
    pub home_runs: u64,

    pub balls: u64,
    pub strikes: u64,
    pub fouls: u64,

    pub total_games: u64,
    pub total_at_bats: u64,
}

impl AddAssign for Avg {
    fn add_assign(&mut self, rhs: Self) {
        self.home_wins += rhs.home_wins;
        self.hits += rhs.hits;

        self.double_plays += rhs.double_plays;
        self.triple_plays += rhs.triple_plays;

        self.runs += rhs.runs;
        self.strikeouts += rhs.strikeouts;
        self.walks += rhs.walks;

        self.singles += rhs.singles;
        self.doubles += rhs.doubles;
        self.triples += rhs.triples;
        self.inside_the_park += rhs.inside_the_park;
        self.home_runs += rhs.home_runs;

        self.balls += rhs.balls;
        self.strikes += rhs.strikes;
        self.fouls += rhs.fouls;

        self.total_games += rhs.total_games;
        self.total_at_bats += rhs.total_at_bats;
    }
}

impl Add for Avg {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            home_wins: self.home_wins + rhs.home_wins,
            runs: self.runs + rhs.runs,

            double_plays: self.double_plays + rhs.double_plays,
            triple_plays: self.triple_plays + rhs.triple_plays,

            hits: self.hits + rhs.hits,
            strikeouts: self.strikeouts + rhs.strikeouts,
            walks: self.walks + rhs.walks,
            singles: self.singles + rhs.singles,
            doubles: self.doubles + rhs.doubles,
            triples: self.triples + rhs.triples,
            inside_the_park: self.inside_the_park + rhs.inside_the_park,
            home_runs: self.home_runs + rhs.home_runs,

            balls: self.balls + rhs.balls,
            strikes: self.strikes + rhs.strikes,
            fouls: self.fouls + rhs.fouls,

            total_games: self.total_games + rhs.total_games,
            total_at_bats: self.total_at_bats + rhs.total_at_bats,
        }
    }
}

const AWAY_SKILL_BASE: i8 = -128;
const HOME_SKILL_BASE: i8 = 127;

pub fn sim_for_averages_biased(sim_count: u64, decider: &mut impl Decider) -> Avg {
    let away_team = Team {
        fielders: [
            Player {
                name: Some("Carl Atcher".to_string()),
                jersey_number: "12".to_string(),
                pitch_height_bias: AWAY_SKILL_BASE,
                pitch_width_bias: AWAY_SKILL_BASE,
                pitch_strike_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                pitcher_hit_direction_bias: AWAY_SKILL_BASE,
                pitcher_launch_angle_bias: AWAY_SKILL_BASE,
                pitcher_hit_speed_bias: AWAY_SKILL_BASE,
                hitter_swing_on_ball_bias: AWAY_SKILL_BASE,
                hitter_contact_on_ball_bias: AWAY_SKILL_BASE,
                hitter_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                hitter_swing_on_strike_bias: AWAY_SKILL_BASE,
                hitter_contact_on_strike_bias: AWAY_SKILL_BASE,
                hitter_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                hitter_hit_direction_bias: AWAY_SKILL_BASE,
                hitter_launch_angle_bias: AWAY_SKILL_BASE,
                hitter_hit_speed_bias: AWAY_SKILL_BASE,
                fielder_run_speed_bias: AWAY_SKILL_BASE,
                fielder_reaction_time_bias: AWAY_SKILL_BASE,
                fielder_throw_speed_bias: AWAY_SKILL_BASE,
                fielder_transfer_time_bias: AWAY_SKILL_BASE,
                baserunner_run_speed_bias: AWAY_SKILL_BASE,
                baserunner_box_exit_time_bias: AWAY_SKILL_BASE,
                baserunner_takeoff_delay_bias: AWAY_SKILL_BASE,
            },
            Player {
                name: Some("Pete Itcher".to_string()),
                jersey_number: "1".to_string(),
                pitch_height_bias: AWAY_SKILL_BASE,
                pitch_width_bias: AWAY_SKILL_BASE,
                pitch_strike_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                pitcher_hit_direction_bias: AWAY_SKILL_BASE,
                pitcher_launch_angle_bias: AWAY_SKILL_BASE,
                pitcher_hit_speed_bias: AWAY_SKILL_BASE,
                hitter_swing_on_ball_bias: AWAY_SKILL_BASE,
                hitter_contact_on_ball_bias: AWAY_SKILL_BASE,
                hitter_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                hitter_swing_on_strike_bias: AWAY_SKILL_BASE,
                hitter_contact_on_strike_bias: AWAY_SKILL_BASE,
                hitter_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                hitter_hit_direction_bias: AWAY_SKILL_BASE,
                hitter_launch_angle_bias: AWAY_SKILL_BASE,
                hitter_hit_speed_bias: AWAY_SKILL_BASE,
                fielder_run_speed_bias: AWAY_SKILL_BASE,
                fielder_reaction_time_bias: AWAY_SKILL_BASE,
                fielder_throw_speed_bias: AWAY_SKILL_BASE,
                fielder_transfer_time_bias: AWAY_SKILL_BASE,
                baserunner_run_speed_bias: AWAY_SKILL_BASE,
                baserunner_box_exit_time_bias: AWAY_SKILL_BASE,
                baserunner_takeoff_delay_bias: AWAY_SKILL_BASE,
            },
            Player {
                name: Some("Frank Base".to_string()),
                jersey_number: "77".to_string(),
                pitch_height_bias: AWAY_SKILL_BASE,
                pitch_width_bias: AWAY_SKILL_BASE,
                pitch_strike_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                pitcher_hit_direction_bias: AWAY_SKILL_BASE,
                pitcher_launch_angle_bias: AWAY_SKILL_BASE,
                pitcher_hit_speed_bias: AWAY_SKILL_BASE,
                hitter_swing_on_ball_bias: AWAY_SKILL_BASE,
                hitter_contact_on_ball_bias: AWAY_SKILL_BASE,
                hitter_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                hitter_swing_on_strike_bias: AWAY_SKILL_BASE,
                hitter_contact_on_strike_bias: AWAY_SKILL_BASE,
                hitter_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                hitter_hit_direction_bias: AWAY_SKILL_BASE,
                hitter_launch_angle_bias: AWAY_SKILL_BASE,
                hitter_hit_speed_bias: AWAY_SKILL_BASE,
                fielder_run_speed_bias: AWAY_SKILL_BASE,
                fielder_reaction_time_bias: AWAY_SKILL_BASE,
                fielder_throw_speed_bias: AWAY_SKILL_BASE,
                fielder_transfer_time_bias: AWAY_SKILL_BASE,
                baserunner_run_speed_bias: AWAY_SKILL_BASE,
                baserunner_box_exit_time_bias: AWAY_SKILL_BASE,
                baserunner_takeoff_delay_bias: AWAY_SKILL_BASE,
            },
            Player {
                name: Some("Steve Base".to_string()),
                jersey_number: "12".to_string(),
                pitch_height_bias: AWAY_SKILL_BASE,
                pitch_width_bias: AWAY_SKILL_BASE,
                pitch_strike_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                pitcher_hit_direction_bias: AWAY_SKILL_BASE,
                pitcher_launch_angle_bias: AWAY_SKILL_BASE,
                pitcher_hit_speed_bias: AWAY_SKILL_BASE,
                hitter_swing_on_ball_bias: AWAY_SKILL_BASE,
                hitter_contact_on_ball_bias: AWAY_SKILL_BASE,
                hitter_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                hitter_swing_on_strike_bias: AWAY_SKILL_BASE,
                hitter_contact_on_strike_bias: AWAY_SKILL_BASE,
                hitter_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                hitter_hit_direction_bias: AWAY_SKILL_BASE,
                hitter_launch_angle_bias: AWAY_SKILL_BASE,
                hitter_hit_speed_bias: AWAY_SKILL_BASE,
                fielder_run_speed_bias: AWAY_SKILL_BASE,
                fielder_reaction_time_bias: AWAY_SKILL_BASE,
                fielder_throw_speed_bias: AWAY_SKILL_BASE,
                fielder_transfer_time_bias: AWAY_SKILL_BASE,
                baserunner_run_speed_bias: AWAY_SKILL_BASE,
                baserunner_box_exit_time_bias: AWAY_SKILL_BASE,
                baserunner_takeoff_delay_bias: AWAY_SKILL_BASE,
            },
            Player {
                name: Some("Tom Base".to_string()),
                jersey_number: "32".to_string(),
                pitch_height_bias: AWAY_SKILL_BASE,
                pitch_width_bias: AWAY_SKILL_BASE,
                pitch_strike_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                pitcher_hit_direction_bias: AWAY_SKILL_BASE,
                pitcher_launch_angle_bias: AWAY_SKILL_BASE,
                pitcher_hit_speed_bias: AWAY_SKILL_BASE,
                hitter_swing_on_ball_bias: AWAY_SKILL_BASE,
                hitter_contact_on_ball_bias: AWAY_SKILL_BASE,
                hitter_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                hitter_swing_on_strike_bias: AWAY_SKILL_BASE,
                hitter_contact_on_strike_bias: AWAY_SKILL_BASE,
                hitter_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                hitter_hit_direction_bias: AWAY_SKILL_BASE,
                hitter_launch_angle_bias: AWAY_SKILL_BASE,
                hitter_hit_speed_bias: AWAY_SKILL_BASE,
                fielder_run_speed_bias: AWAY_SKILL_BASE,
                fielder_reaction_time_bias: AWAY_SKILL_BASE,
                fielder_throw_speed_bias: AWAY_SKILL_BASE,
                fielder_transfer_time_bias: AWAY_SKILL_BASE,
                baserunner_run_speed_bias: AWAY_SKILL_BASE,
                baserunner_box_exit_time_bias: AWAY_SKILL_BASE,
                baserunner_takeoff_delay_bias: AWAY_SKILL_BASE,
            },
            Player {
                name: Some("Sonny Stop".to_string()),
                jersey_number: "41".to_string(),
                pitch_height_bias: AWAY_SKILL_BASE,
                pitch_width_bias: AWAY_SKILL_BASE,
                pitch_strike_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                pitcher_hit_direction_bias: AWAY_SKILL_BASE,
                pitcher_launch_angle_bias: AWAY_SKILL_BASE,
                pitcher_hit_speed_bias: AWAY_SKILL_BASE,
                hitter_swing_on_ball_bias: AWAY_SKILL_BASE,
                hitter_contact_on_ball_bias: AWAY_SKILL_BASE,
                hitter_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                hitter_swing_on_strike_bias: AWAY_SKILL_BASE,
                hitter_contact_on_strike_bias: AWAY_SKILL_BASE,
                hitter_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                hitter_hit_direction_bias: AWAY_SKILL_BASE,
                hitter_launch_angle_bias: AWAY_SKILL_BASE,
                hitter_hit_speed_bias: AWAY_SKILL_BASE,
                fielder_run_speed_bias: AWAY_SKILL_BASE,
                fielder_reaction_time_bias: AWAY_SKILL_BASE,
                fielder_throw_speed_bias: AWAY_SKILL_BASE,
                fielder_transfer_time_bias: AWAY_SKILL_BASE,
                baserunner_run_speed_bias: AWAY_SKILL_BASE,
                baserunner_box_exit_time_bias: AWAY_SKILL_BASE,
                baserunner_takeoff_delay_bias: AWAY_SKILL_BASE,
            },
            Player {
                name: Some("Larry Field".to_string()),
                jersey_number: "12".to_string(),
                pitch_height_bias: AWAY_SKILL_BASE,
                pitch_width_bias: AWAY_SKILL_BASE,
                pitch_strike_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                pitcher_hit_direction_bias: AWAY_SKILL_BASE,
                pitcher_launch_angle_bias: AWAY_SKILL_BASE,
                pitcher_hit_speed_bias: AWAY_SKILL_BASE,
                hitter_swing_on_ball_bias: AWAY_SKILL_BASE,
                hitter_contact_on_ball_bias: AWAY_SKILL_BASE,
                hitter_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                hitter_swing_on_strike_bias: AWAY_SKILL_BASE,
                hitter_contact_on_strike_bias: AWAY_SKILL_BASE,
                hitter_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                hitter_hit_direction_bias: AWAY_SKILL_BASE,
                hitter_launch_angle_bias: AWAY_SKILL_BASE,
                hitter_hit_speed_bias: AWAY_SKILL_BASE,
                fielder_run_speed_bias: AWAY_SKILL_BASE,
                fielder_reaction_time_bias: AWAY_SKILL_BASE,
                fielder_throw_speed_bias: AWAY_SKILL_BASE,
                fielder_transfer_time_bias: AWAY_SKILL_BASE,
                baserunner_run_speed_bias: AWAY_SKILL_BASE,
                baserunner_box_exit_time_bias: AWAY_SKILL_BASE,
                baserunner_takeoff_delay_bias: AWAY_SKILL_BASE,
            },
            Player {
                name: Some("Charlie Field".to_string()),
                jersey_number: "12".to_string(),
                pitch_height_bias: AWAY_SKILL_BASE,
                pitch_width_bias: AWAY_SKILL_BASE,
                pitch_strike_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                pitcher_hit_direction_bias: AWAY_SKILL_BASE,
                pitcher_launch_angle_bias: AWAY_SKILL_BASE,
                pitcher_hit_speed_bias: AWAY_SKILL_BASE,
                hitter_swing_on_ball_bias: AWAY_SKILL_BASE,
                hitter_contact_on_ball_bias: AWAY_SKILL_BASE,
                hitter_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                hitter_swing_on_strike_bias: AWAY_SKILL_BASE,
                hitter_contact_on_strike_bias: AWAY_SKILL_BASE,
                hitter_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                hitter_hit_direction_bias: AWAY_SKILL_BASE,
                hitter_launch_angle_bias: AWAY_SKILL_BASE,
                hitter_hit_speed_bias: AWAY_SKILL_BASE,
                fielder_run_speed_bias: AWAY_SKILL_BASE,
                fielder_reaction_time_bias: AWAY_SKILL_BASE,
                fielder_throw_speed_bias: AWAY_SKILL_BASE,
                fielder_transfer_time_bias: AWAY_SKILL_BASE,
                baserunner_run_speed_bias: AWAY_SKILL_BASE,
                baserunner_box_exit_time_bias: AWAY_SKILL_BASE,
                baserunner_takeoff_delay_bias: AWAY_SKILL_BASE,
            },
            Player {
                name: Some("Rick Field".to_string()),
                jersey_number: "12".to_string(),
                pitch_height_bias: AWAY_SKILL_BASE,
                pitch_width_bias: AWAY_SKILL_BASE,
                pitch_strike_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                pitcher_hit_direction_bias: AWAY_SKILL_BASE,
                pitcher_launch_angle_bias: AWAY_SKILL_BASE,
                pitcher_hit_speed_bias: AWAY_SKILL_BASE,
                hitter_swing_on_ball_bias: AWAY_SKILL_BASE,
                hitter_contact_on_ball_bias: AWAY_SKILL_BASE,
                hitter_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                hitter_swing_on_strike_bias: AWAY_SKILL_BASE,
                hitter_contact_on_strike_bias: AWAY_SKILL_BASE,
                hitter_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                hitter_hit_direction_bias: AWAY_SKILL_BASE,
                hitter_launch_angle_bias: AWAY_SKILL_BASE,
                hitter_hit_speed_bias: AWAY_SKILL_BASE,
                fielder_run_speed_bias: AWAY_SKILL_BASE,
                fielder_reaction_time_bias: AWAY_SKILL_BASE,
                fielder_throw_speed_bias: AWAY_SKILL_BASE,
                fielder_transfer_time_bias: AWAY_SKILL_BASE,
                baserunner_run_speed_bias: AWAY_SKILL_BASE,
                baserunner_box_exit_time_bias: AWAY_SKILL_BASE,
                baserunner_takeoff_delay_bias: AWAY_SKILL_BASE,
            },
        ],
        batting_order: [
            Fielder::Catcher,
            Fielder::Pitcher,
            Fielder::FirstBase,
            Fielder::SecondBase,
            Fielder::ThirdBase,
            Fielder::Shortstop,
            Fielder::LeftFielder,
            Fielder::CenterFielder,
            Fielder::RightFielder,
        ]
    };
    let home_team = Team {
        fielders: [
            Player {
                name: Some("Bob Random".to_string()),
                jersey_number: "12".to_string(),
                pitch_height_bias: HOME_SKILL_BASE,
                pitch_width_bias: HOME_SKILL_BASE,
                pitch_strike_bias: HOME_SKILL_BASE,
                pitcher_swing_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                pitcher_hit_direction_bias: AWAY_SKILL_BASE,
                pitcher_launch_angle_bias: AWAY_SKILL_BASE,
                pitcher_hit_speed_bias: AWAY_SKILL_BASE,
                hitter_swing_on_ball_bias: HOME_SKILL_BASE,
                hitter_contact_on_ball_bias: HOME_SKILL_BASE,
                hitter_foul_on_ball_contact_bias: HOME_SKILL_BASE,
                hitter_swing_on_strike_bias: HOME_SKILL_BASE,
                hitter_contact_on_strike_bias: HOME_SKILL_BASE,
                hitter_foul_on_strike_contact_bias: HOME_SKILL_BASE,
                hitter_hit_direction_bias: HOME_SKILL_BASE,
                hitter_launch_angle_bias: HOME_SKILL_BASE,
                hitter_hit_speed_bias: HOME_SKILL_BASE,
                fielder_run_speed_bias: HOME_SKILL_BASE,
                fielder_reaction_time_bias: HOME_SKILL_BASE,
                fielder_throw_speed_bias: HOME_SKILL_BASE,
                fielder_transfer_time_bias: HOME_SKILL_BASE,
                baserunner_run_speed_bias: HOME_SKILL_BASE,
                baserunner_box_exit_time_bias: HOME_SKILL_BASE,
                baserunner_takeoff_delay_bias: HOME_SKILL_BASE,
            },
            Player {
                name: Some("Joe Lazy".to_string()),
                jersey_number: "1".to_string(),
                pitch_height_bias: HOME_SKILL_BASE,
                pitch_width_bias: HOME_SKILL_BASE,
                pitch_strike_bias: HOME_SKILL_BASE,
                pitcher_swing_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                pitcher_hit_direction_bias: AWAY_SKILL_BASE,
                pitcher_launch_angle_bias: AWAY_SKILL_BASE,
                pitcher_hit_speed_bias: AWAY_SKILL_BASE,
                hitter_swing_on_ball_bias: HOME_SKILL_BASE,
                hitter_contact_on_ball_bias: HOME_SKILL_BASE,
                hitter_foul_on_ball_contact_bias: HOME_SKILL_BASE,
                hitter_swing_on_strike_bias: HOME_SKILL_BASE,
                hitter_contact_on_strike_bias: HOME_SKILL_BASE,
                hitter_foul_on_strike_contact_bias: HOME_SKILL_BASE,
                hitter_hit_direction_bias: HOME_SKILL_BASE,
                hitter_launch_angle_bias: HOME_SKILL_BASE,
                hitter_hit_speed_bias: HOME_SKILL_BASE,
                fielder_run_speed_bias: HOME_SKILL_BASE,
                fielder_reaction_time_bias: HOME_SKILL_BASE,
                fielder_throw_speed_bias: HOME_SKILL_BASE,
                fielder_transfer_time_bias: HOME_SKILL_BASE,
                baserunner_run_speed_bias: HOME_SKILL_BASE,
                baserunner_box_exit_time_bias: HOME_SKILL_BASE,
                baserunner_takeoff_delay_bias: HOME_SKILL_BASE,
            },
            Player {
                name: Some("No Name".to_string()),
                jersey_number: "77".to_string(),
                pitch_height_bias: HOME_SKILL_BASE,
                pitch_width_bias: HOME_SKILL_BASE,
                pitch_strike_bias: HOME_SKILL_BASE,
                pitcher_swing_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                pitcher_hit_direction_bias: AWAY_SKILL_BASE,
                pitcher_launch_angle_bias: AWAY_SKILL_BASE,
                pitcher_hit_speed_bias: AWAY_SKILL_BASE,
                hitter_swing_on_ball_bias: HOME_SKILL_BASE,
                hitter_contact_on_ball_bias: HOME_SKILL_BASE,
                hitter_foul_on_ball_contact_bias: HOME_SKILL_BASE,
                hitter_swing_on_strike_bias: HOME_SKILL_BASE,
                hitter_contact_on_strike_bias: HOME_SKILL_BASE,
                hitter_foul_on_strike_contact_bias: HOME_SKILL_BASE,
                hitter_hit_direction_bias: HOME_SKILL_BASE,
                hitter_launch_angle_bias: HOME_SKILL_BASE,
                hitter_hit_speed_bias: HOME_SKILL_BASE,
                fielder_run_speed_bias: HOME_SKILL_BASE,
                fielder_reaction_time_bias: HOME_SKILL_BASE,
                fielder_throw_speed_bias: HOME_SKILL_BASE,
                fielder_transfer_time_bias: HOME_SKILL_BASE,
                baserunner_run_speed_bias: HOME_SKILL_BASE,
                baserunner_box_exit_time_bias: HOME_SKILL_BASE,
                baserunner_takeoff_delay_bias: HOME_SKILL_BASE,
            },
            Player {
                name: Some("Not Worthit".to_string()),
                jersey_number: "12".to_string(),
                pitch_height_bias: HOME_SKILL_BASE,
                pitch_width_bias: HOME_SKILL_BASE,
                pitch_strike_bias: HOME_SKILL_BASE,
                pitcher_swing_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                pitcher_hit_direction_bias: AWAY_SKILL_BASE,
                pitcher_launch_angle_bias: AWAY_SKILL_BASE,
                pitcher_hit_speed_bias: AWAY_SKILL_BASE,
                hitter_swing_on_ball_bias: HOME_SKILL_BASE,
                hitter_contact_on_ball_bias: HOME_SKILL_BASE,
                hitter_foul_on_ball_contact_bias: HOME_SKILL_BASE,
                hitter_swing_on_strike_bias: HOME_SKILL_BASE,
                hitter_contact_on_strike_bias: HOME_SKILL_BASE,
                hitter_foul_on_strike_contact_bias: HOME_SKILL_BASE,
                hitter_hit_direction_bias: HOME_SKILL_BASE,
                hitter_launch_angle_bias: HOME_SKILL_BASE,
                hitter_hit_speed_bias: HOME_SKILL_BASE,
                fielder_run_speed_bias: HOME_SKILL_BASE,
                fielder_reaction_time_bias: HOME_SKILL_BASE,
                fielder_throw_speed_bias: HOME_SKILL_BASE,
                fielder_transfer_time_bias: HOME_SKILL_BASE,
                baserunner_run_speed_bias: HOME_SKILL_BASE,
                baserunner_box_exit_time_bias: HOME_SKILL_BASE,
                baserunner_takeoff_delay_bias: HOME_SKILL_BASE,
            },
            Player {
                name: Some("Badat Thinkingofnames".to_string()),
                jersey_number: "32".to_string(),
                pitch_height_bias: HOME_SKILL_BASE,
                pitch_width_bias: HOME_SKILL_BASE,
                pitch_strike_bias: HOME_SKILL_BASE,
                pitcher_swing_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                pitcher_hit_direction_bias: AWAY_SKILL_BASE,
                pitcher_launch_angle_bias: AWAY_SKILL_BASE,
                pitcher_hit_speed_bias: AWAY_SKILL_BASE,
                hitter_swing_on_ball_bias: HOME_SKILL_BASE,
                hitter_contact_on_ball_bias: HOME_SKILL_BASE,
                hitter_foul_on_ball_contact_bias: HOME_SKILL_BASE,
                hitter_swing_on_strike_bias: HOME_SKILL_BASE,
                hitter_contact_on_strike_bias: HOME_SKILL_BASE,
                hitter_foul_on_strike_contact_bias: HOME_SKILL_BASE,
                hitter_hit_direction_bias: HOME_SKILL_BASE,
                hitter_launch_angle_bias: HOME_SKILL_BASE,
                hitter_hit_speed_bias: HOME_SKILL_BASE,
                fielder_run_speed_bias: HOME_SKILL_BASE,
                fielder_reaction_time_bias: HOME_SKILL_BASE,
                fielder_throw_speed_bias: HOME_SKILL_BASE,
                fielder_transfer_time_bias: HOME_SKILL_BASE,
                baserunner_run_speed_bias: HOME_SKILL_BASE,
                baserunner_box_exit_time_bias: HOME_SKILL_BASE,
                baserunner_takeoff_delay_bias: HOME_SKILL_BASE,
            },
            Player {
                name: Some("These Dontmatter".to_string()),
                jersey_number: "41".to_string(),
                pitch_height_bias: HOME_SKILL_BASE,
                pitch_width_bias: HOME_SKILL_BASE,
                pitch_strike_bias: HOME_SKILL_BASE,
                pitcher_swing_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                pitcher_hit_direction_bias: AWAY_SKILL_BASE,
                pitcher_launch_angle_bias: AWAY_SKILL_BASE,
                pitcher_hit_speed_bias: AWAY_SKILL_BASE,
                hitter_swing_on_ball_bias: HOME_SKILL_BASE,
                hitter_contact_on_ball_bias: HOME_SKILL_BASE,
                hitter_foul_on_ball_contact_bias: HOME_SKILL_BASE,
                hitter_swing_on_strike_bias: HOME_SKILL_BASE,
                hitter_contact_on_strike_bias: HOME_SKILL_BASE,
                hitter_foul_on_strike_contact_bias: HOME_SKILL_BASE,
                hitter_hit_direction_bias: HOME_SKILL_BASE,
                hitter_launch_angle_bias: HOME_SKILL_BASE,
                hitter_hit_speed_bias: HOME_SKILL_BASE,
                fielder_run_speed_bias: HOME_SKILL_BASE,
                fielder_reaction_time_bias: HOME_SKILL_BASE,
                fielder_throw_speed_bias: HOME_SKILL_BASE,
                fielder_transfer_time_bias: HOME_SKILL_BASE,
                baserunner_run_speed_bias: HOME_SKILL_BASE,
                baserunner_box_exit_time_bias: HOME_SKILL_BASE,
                baserunner_takeoff_delay_bias: HOME_SKILL_BASE,
            },
            Player {
                name: Some("Person Generic".to_string()),
                jersey_number: "12".to_string(),
                pitch_height_bias: HOME_SKILL_BASE,
                pitch_width_bias: HOME_SKILL_BASE,
                pitch_strike_bias: HOME_SKILL_BASE,
                pitcher_swing_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                pitcher_hit_direction_bias: AWAY_SKILL_BASE,
                pitcher_launch_angle_bias: AWAY_SKILL_BASE,
                pitcher_hit_speed_bias: AWAY_SKILL_BASE,
                hitter_swing_on_ball_bias: HOME_SKILL_BASE,
                hitter_contact_on_ball_bias: HOME_SKILL_BASE,
                hitter_foul_on_ball_contact_bias: HOME_SKILL_BASE,
                hitter_swing_on_strike_bias: HOME_SKILL_BASE,
                hitter_contact_on_strike_bias: HOME_SKILL_BASE,
                hitter_foul_on_strike_contact_bias: HOME_SKILL_BASE,
                hitter_hit_direction_bias: HOME_SKILL_BASE,
                hitter_launch_angle_bias: HOME_SKILL_BASE,
                hitter_hit_speed_bias: HOME_SKILL_BASE,
                fielder_run_speed_bias: HOME_SKILL_BASE,
                fielder_reaction_time_bias: HOME_SKILL_BASE,
                fielder_throw_speed_bias: HOME_SKILL_BASE,
                fielder_transfer_time_bias: HOME_SKILL_BASE,
                baserunner_run_speed_bias: HOME_SKILL_BASE,
                baserunner_box_exit_time_bias: HOME_SKILL_BASE,
                baserunner_takeoff_delay_bias: HOME_SKILL_BASE,
            },
            Player {
                name: Some("Generic Person".to_string()),
                jersey_number: "12".to_string(),
                pitch_height_bias: HOME_SKILL_BASE,
                pitch_width_bias: HOME_SKILL_BASE,
                pitch_strike_bias: HOME_SKILL_BASE,
                pitcher_swing_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                pitcher_hit_direction_bias: AWAY_SKILL_BASE,
                pitcher_launch_angle_bias: AWAY_SKILL_BASE,
                pitcher_hit_speed_bias: AWAY_SKILL_BASE,
                hitter_swing_on_ball_bias: HOME_SKILL_BASE,
                hitter_contact_on_ball_bias: HOME_SKILL_BASE,
                hitter_foul_on_ball_contact_bias: HOME_SKILL_BASE,
                hitter_swing_on_strike_bias: HOME_SKILL_BASE,
                hitter_contact_on_strike_bias: HOME_SKILL_BASE,
                hitter_foul_on_strike_contact_bias: HOME_SKILL_BASE,
                hitter_hit_direction_bias: HOME_SKILL_BASE,
                hitter_launch_angle_bias: HOME_SKILL_BASE,
                hitter_hit_speed_bias: HOME_SKILL_BASE,
                fielder_run_speed_bias: HOME_SKILL_BASE,
                fielder_reaction_time_bias: HOME_SKILL_BASE,
                fielder_throw_speed_bias: HOME_SKILL_BASE,
                fielder_transfer_time_bias: HOME_SKILL_BASE,
                baserunner_run_speed_bias: HOME_SKILL_BASE,
                baserunner_box_exit_time_bias: HOME_SKILL_BASE,
                baserunner_takeoff_delay_bias: HOME_SKILL_BASE,
            },
            Player {
                name: Some("Last Player".to_string()),
                jersey_number: "12".to_string(),
                pitch_height_bias: HOME_SKILL_BASE,
                pitch_width_bias: HOME_SKILL_BASE,
                pitch_strike_bias: HOME_SKILL_BASE,
                pitcher_swing_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_ball_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_ball_contact_bias: AWAY_SKILL_BASE,
                pitcher_swing_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_contact_on_strike_bias: AWAY_SKILL_BASE,
                pitcher_foul_on_strike_contact_bias: AWAY_SKILL_BASE,
                pitcher_hit_direction_bias: AWAY_SKILL_BASE,
                pitcher_launch_angle_bias: AWAY_SKILL_BASE,
                pitcher_hit_speed_bias: AWAY_SKILL_BASE,
                hitter_swing_on_ball_bias: HOME_SKILL_BASE,
                hitter_contact_on_ball_bias: HOME_SKILL_BASE,
                hitter_foul_on_ball_contact_bias: HOME_SKILL_BASE,
                hitter_swing_on_strike_bias: HOME_SKILL_BASE,
                hitter_contact_on_strike_bias: HOME_SKILL_BASE,
                hitter_foul_on_strike_contact_bias: HOME_SKILL_BASE,
                hitter_hit_direction_bias: HOME_SKILL_BASE,
                hitter_launch_angle_bias: HOME_SKILL_BASE,
                hitter_hit_speed_bias: HOME_SKILL_BASE,
                fielder_run_speed_bias: HOME_SKILL_BASE,
                fielder_reaction_time_bias: HOME_SKILL_BASE,
                fielder_throw_speed_bias: HOME_SKILL_BASE,
                fielder_transfer_time_bias: HOME_SKILL_BASE,
                baserunner_run_speed_bias: HOME_SKILL_BASE,
                baserunner_box_exit_time_bias: HOME_SKILL_BASE,
                baserunner_takeoff_delay_bias: HOME_SKILL_BASE,
            },
        ],
        batting_order: [
            Fielder::Catcher,
            Fielder::Pitcher,
            Fielder::FirstBase,
            Fielder::SecondBase,
            Fielder::ThirdBase,
            Fielder::Shortstop,
            Fielder::LeftFielder,
            Fielder::CenterFielder,
            Fielder::RightFielder,
        ]
    };

    let mut running_totals = Avg::default();
    let progress_divisor = 100;
    let progress_mod = sim_count / progress_divisor;
    let mut no_hitters = 0;
    for count in 0..sim_count {
        let game = replaceball_sim::simulate_game_with_teams(decider, &home_team, &away_team);
        running_totals += count_totals(&game);
        if count % progress_mod == 0 {
            println!("Completed: {:3}%", count / progress_mod);
        }

        if game.outcome.home_hits == 0 || game.outcome.away_hits == 0 {
            no_hitters += 1;
        }
    }

    println!("=== NO HITTERS: {} ===", no_hitters);

    running_totals
}

pub fn sim_for_averages(sim_count: u64, decider: &mut impl Decider) -> Avg {
    let mut running_totals = Avg::default();
    let progress_divisor = 100;
    let progress_mod = sim_count / progress_divisor;
    for count in 0..sim_count {
        let game = replaceball_sim::simulate_game(decider);
        running_totals += count_totals(&game);
        if count % progress_mod == 0 {
            println!("Completed: {:3}%", count / progress_mod);
        }
    }

    running_totals
}

fn count_totals(game: &GameRecord) -> Avg {
    Avg {
        home_wins: if game.outcome.home_score > game.outcome.away_score {
            1
        } else {
            0
        },
        runs: (game.outcome.home_score + game.outcome.away_score) as u64,
        hits: 0,

        double_plays: 0,
        triple_plays: 0,

        strikeouts: 0,
        walks: 0,
        singles: 0,
        doubles: 0,
        triples: 0,
        inside_the_park: 0,
        home_runs: 0,

        balls: 0,
        strikes: 0,
        fouls: 0,

        total_games: 1,
        total_at_bats: 0,
    } + sum_game_stats(game)
}

fn sum_game_stats(game: &GameRecord) -> Avg {
    game.innings
        .iter()
        .map(|inning| {
            [&inning.0.away, &inning.0.home]
                .iter()
                .map(|half_inning| {
                    half_inning
                        .at_bats
                        .iter()
                        .map(|at_bat| at_bat_stats(&at_bat.0))
                        .fold(Avg::default(), |acc, stat| acc + stat)
                })
                .fold(Avg::default(), |acc, stat| acc + stat)
        })
        .fold(Avg::default(), |acc, stat| acc + stat)
}

fn at_bat_stats(at_bat: &AtBatRecord) -> Avg {
    let (is_out_of_park, hit_type, hit_record) = match &at_bat.outcome.outcome_type {
        AtBatOutcomeType::Out | AtBatOutcomeType::Walk => (false, None, None),
        AtBatOutcomeType::Hit(hit_record) => match hit_record.outcome.hit_type() {
            HitType::Out => (false, None, Some(hit_record)),
            hit => (
                hit_record.outcome == HitOutcome::HomeRun,
                Some(hit),
                Some(hit_record),
            ),
        },
    };

    let outs_made = match hit_record.map(|r| &r.outcome) {
        Some(HitOutcome::InPlay(fielding_record)) => {
            fielding_record.base_running_record.outcome.outs_made
        }
        Some(HitOutcome::HomeRun) | None => 0,
    };

    Avg {
        home_wins: 0,
        runs: 0,
        hits: match hit_type {
            None => 0,
            Some(_) => 1,
        },
        double_plays: if outs_made == 2 { 1 } else { 0 },
        triple_plays: if outs_made == 3 { 1 } else { 0 },
        strikeouts: if at_bat.pitches.last().unwrap().1.strikes == 3 {
            1
        } else {
            0
        },
        walks: if at_bat.pitches.last().unwrap().1.balls == 4 {
            1
        } else {
            0
        },
        singles: if hit_type == Some(HitType::Single) {
            1
        } else {
            0
        },
        doubles: if hit_type == Some(HitType::Double) {
            1
        } else {
            0
        },
        triples: if hit_type == Some(HitType::Triple) {
            1
        } else {
            0
        },
        inside_the_park: if hit_type == Some(HitType::HomeRun) && !is_out_of_park {
            1
        } else {
            0
        },
        home_runs: if hit_type == Some(HitType::HomeRun) && is_out_of_park {
            1
        } else {
            0
        },
        balls: 0,
        strikes: 0,
        fouls: 0,
        total_games: 0,
        total_at_bats: 1,
    } + at_bat
        .pitches
        .iter()
        .map(|pitch| pitch_stats(&pitch.0))
        .fold(Avg::default(), |acc, stats| acc + stats)
}

fn pitch_stats(pitch: &PitchRecord) -> Avg {
    Avg {
        home_wins: 0,
        runs: 0,
        hits: 0,
        double_plays: 0,
        triple_plays: 0,
        strikeouts: 0,
        walks: 0,
        singles: 0,
        doubles: 0,
        triples: 0,
        inside_the_park: 0,
        home_runs: 0,
        balls: if pitch.outcome == PitchOutcome::Ball {
            1
        } else {
            0
        },
        strikes: match pitch.outcome {
            PitchOutcome::Strike(_) => 1,
            _ => 0,
        },
        fouls: if pitch.outcome == PitchOutcome::Foul {
            1
        } else {
            0
        },
        total_games: 0,
        total_at_bats: 0,
    }
}
