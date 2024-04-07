
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{player::Team, prelude::*};

#[derive(Clone, PartialEq, Debug, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BaseMovement {
    pub starting_base: Option<usize>,
    pub bases_moved: MoveType,
}

#[derive(Clone, PartialEq, Debug, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MoveType {
    // Stores the base the player is out at
    Out(usize),

    // Stores the number of bases the player advanced
    Advanced(usize),
}

#[derive(Clone, PartialEq, Debug, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BaseRunningOutcome {
    pub outs_made: u8,
    pub runs_scored: u8,
    pub batter_hit_type: HitType,
    pub ending_base_state: [Option<u8>; 3],
}

#[derive(Clone, PartialEq, Debug, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BaseRunningRecord {
    pub movements: Box<[BaseMovement]>,
    pub outcome: BaseRunningOutcome,
}

pub fn simulate_base_running(
    batter_lineup_index: u8,
    batting_team: &Team,
    fielding_team: &Team,
    ball_landing: &BallLanding,
    base_state: &[Option<u8>; 3],
    decider: &mut impl Decider,
) -> BaseRunningRecord {
    match ball_landing {
        BallLanding::Out(
            fielder,
            location
        ) => post_out_base_running(
            fielder,
            location,
            base_state,
            decider,
        ),
        BallLanding::Landed(
            location,
            fielding_play
        ) => post_throw_base_running(
            location,
            fielding_play,
            batter_lineup_index,
            base_state,
            decider,
        ),
    }
}

fn post_out_base_running(
    _fielder: &Fielder,
    location: &Location,
    base_state: &[Option<u8>; 3],
    decider: &mut impl Decider,
) -> BaseRunningRecord {
    let mut base_movements = Vec::<BaseMovement>::new();
    let mut base_state = base_state.clone();
    let fielder_throw_speed = decider.roll_stat(*levels::THROW_SPEED, Skill::default());

    // Out was already made
    let mut outs_made = 1u8;
    let mut runs_scored = 0u8;
    let mut has_thrown = false;

    if base_state[Consts::THIRD].is_some() && location.distance.0 > 250.0 {
        // Runs home on the sac fly
        base_state[Consts::THIRD] = None;
        let third_runner_speed = decider.roll_stat(*levels::BASERUNNER_SPEED, Skill::default());

        has_thrown = true;
        let run_time = 90.0 / third_runner_speed;
        let throw_distance =
            location.distance(Location::home_plate());
        let throw_time = throw_distance / fielder_throw_speed;

        if run_time > throw_time {
            // Caught at home
            base_movements.push(
                BaseMovement {
                    starting_base: Some(Consts::THIRD),
                    bases_moved: MoveType::Out(Consts::HOME),
                }
            );
            outs_made += 1;
        } else {
            // Scores
            base_movements.push(
                BaseMovement {
                    starting_base: Some(Consts::THIRD),
                    bases_moved: MoveType::Advanced(1),
                }
            );
            runs_scored += 1;
        }
    }

    if base_state[Consts::THIRD].is_none() &&
        base_state[Consts::SECOND].is_some() &&
        location.distance.0 > 275.0 &&
        location.direction.0 > 45.0 {
            // Man on second tags and runs to third
            let runner = base_state[Consts::SECOND];
            base_state[Consts::SECOND] = None;
            let second_runner_speed = decider.roll_stat(*levels::BASERUNNER_SPEED, Skill::default());

            let run_time = 90.0 / second_runner_speed;
            let throw_distance =
                location.distance(Location::third_base());
            let throw_time = throw_distance / fielder_throw_speed;

            if run_time > throw_time && !has_thrown {
                // Caught out at third
                outs_made += 1;
                base_movements.push(BaseMovement {
                    starting_base: Some(Consts::SECOND),
                    bases_moved: MoveType::Out(Consts::THIRD),
                });
            } else {
                base_movements.push(BaseMovement {
                    starting_base: Some(Consts::SECOND),
                    bases_moved: MoveType::Advanced(1),
                });
                base_state[Consts::THIRD] = runner;
            }
    }

    BaseRunningRecord {
        movements: base_movements.into_boxed_slice(),
        outcome: BaseRunningOutcome {
            outs_made,
            runs_scored,
            batter_hit_type: HitType::Out,
            ending_base_state: base_state,
        }
    }
}

fn post_throw_base_running(
    _location: &Location,
    fielding_play: &FieldingPlay,
    batter_lineup_index: u8,
    base_state: &[Option<u8>; 3],
    decider: &mut impl Decider,
) -> BaseRunningRecord {
    let mut base_movements = Vec::<BaseMovement>::new();
    let mut runs_scored: u8 = 0;
    let mut outs_made: u8 = 0;
    let mut base_state = base_state.clone();

    let fielder_throw_speed = decider.roll_stat(*levels::THROW_SPEED, Skill::default());
    let batter_runner_speed =
        decider.roll_stat(*levels::BASERUNNER_SPEED, Skill::default());
    let batter_box_exit_time =
        decider.roll_stat(*levels::BOX_EXIT_TIME, Skill::default());

    let batter_hit_type: HitType;
    match fielding_play.base {
        Consts::FIRST => {
            let run_time = 90.0 / batter_runner_speed + batter_box_exit_time;
            let minimum_advance: u8;
            if run_time > fielding_play.to_event.travel_time.0 {
                // Player is out at first
                batter_hit_type = HitType::Out;
                outs_made += 1;
                base_movements.push(BaseMovement {
                    starting_base: None,
                    bases_moved: MoveType::Out(Consts::FIRST),
                });
                minimum_advance = 0;
            } else {
                minimum_advance = batter_advanced(
                    fielding_play.to_event.travel_time.0,
                    batter_runner_speed,
                    batter_box_exit_time,
                );
                match minimum_advance {
                    1 => batter_hit_type = HitType::Single,
                    2 => batter_hit_type = HitType::Double,
                    3 => batter_hit_type = HitType::Triple,
                    _ => batter_hit_type = HitType::HomeRun,
                }
            }

            if base_state[Consts::THIRD].is_some() {
                runs_scored += 1;
                base_state[Consts::THIRD] = None;
                base_movements.push(BaseMovement {
                    starting_base: Some(Consts::THIRD),
                    bases_moved: MoveType::Advanced(1),
                });
            }

            if base_state[Consts::SECOND].is_some() {
                let runner = base_state[Consts::SECOND];
                base_state[Consts::SECOND] = None;
                let second_runner_speed = decider.roll_stat(*levels::BASERUNNER_SPEED, Skill::default());

                let time_to_home = (90.0 * 2.0) / second_runner_speed;
                let time_to_throw_home = fielding_play.from_event.location.distance.0 / fielder_throw_speed;
                if time_to_home > time_to_throw_home && minimum_advance < 3 {
                    // Stays on third
                    base_state[Consts::THIRD] = runner;
                    base_movements.push(BaseMovement {
                        starting_base: Some(Consts::SECOND),
                        bases_moved: MoveType::Advanced(1),
                    });
                } else {
                    // Runner stretches for home
                    runs_scored += 1;
                    base_movements.push(BaseMovement {
                        starting_base: Some(Consts::SECOND),
                        bases_moved: MoveType::Advanced(2),
                    });
                }
            }

            if base_state[Consts::FIRST].is_some() {
                let runner = base_state[Consts::FIRST];
                base_state[Consts::FIRST] = None;

                match minimum_advance {
                    0 | 1 => {
                        base_state[Consts::SECOND] = runner;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::FIRST),
                            bases_moved: MoveType::Advanced(1),
                        });
                    },
                    2 | 3 | _ => {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::FIRST),
                            bases_moved: MoveType::Advanced(3),
                        });
                    }
                }
            }

            match minimum_advance {
                1 => {
                    base_state[Consts::FIRST] = Some(batter_lineup_index);
                    base_movements.push(BaseMovement {
                        starting_base: None,
                        bases_moved: MoveType::Advanced(1),
                    });
                },
                2 => {
                    base_state[Consts::SECOND] = Some(batter_lineup_index);
                    base_movements.push(BaseMovement {
                        starting_base: None,
                        bases_moved: MoveType::Advanced(2),
                    });
                },
                3 => {
                    base_state[Consts::THIRD] = Some(batter_lineup_index);
                    base_movements.push(BaseMovement {
                        starting_base: None,
                        bases_moved: MoveType::Advanced(3),
                    });
                },
                4 => {
                    base_movements.push(BaseMovement {
                        starting_base: None,
                        bases_moved: MoveType::Advanced(4),
                    });
                    runs_scored += 1;
                },
                _ => (),
            }
        },

        Consts::SECOND => {
            let batter_advanced = batter_advanced(
                fielding_play.to_event.travel_time.0,
                batter_runner_speed,
                batter_box_exit_time,
            );

            let mut new_base_state = [None; 3];
            match batter_advanced {
                0 => {
                    if base_state[Consts::FIRST].is_some() {
                        // Force at second
                        let first_batter_speed = decider.roll_stat(*levels::BASERUNNER_SPEED, Skill::default());
                        let first_takeoff_delay = decider.roll_stat(*levels::BASE_TAKEOFF_DELAY, Skill::default());
                        let first_to_second_run_time = 90.0 / first_batter_speed + first_takeoff_delay;
                        if first_to_second_run_time > fielding_play.to_event.travel_time.0 {
                            // Player is out at second
                            outs_made += 1;
                            base_movements.push(BaseMovement {
                                starting_base: Some(Consts::FIRST),
                                bases_moved: MoveType::Out(Consts::SECOND),
                            });

                            let second_fielder_throw_speed = decider.roll_stat(*levels::THROW_SPEED, Skill::default());
                            let batter_run_time = 90.0 / batter_runner_speed + batter_box_exit_time;
                            let double_play_throw_time = 90.0 / second_fielder_throw_speed;
                            if batter_run_time > fielding_play.to_event.travel_time.0 + double_play_throw_time {
                                // Turns the double play
                                batter_hit_type = HitType::Out;

                                outs_made += 1;
                                base_movements.push(BaseMovement {
                                    starting_base: None,
                                    bases_moved: MoveType::Out(Consts::FIRST),
                                });
                            } else {
                                // Fails to turn the double play
                                batter_hit_type = HitType::FieldersChoice;

                                new_base_state[Consts::FIRST] = Some(batter_lineup_index);
                                base_movements.push(BaseMovement {
                                    starting_base: None,
                                    bases_moved: MoveType::Advanced(1),
                                });
                            }
                        } else {
                            // Player is safe at second
                            batter_hit_type = HitType::Single;

                            new_base_state[Consts::SECOND] = Some(batter_lineup_index);
                            base_movements.push(BaseMovement {
                                starting_base: Some(Consts::FIRST),
                                bases_moved: MoveType::Advanced(1),
                            });
                        }
                    } else {
                        // No force at second
                        batter_hit_type = HitType::Single;

                        new_base_state[Consts::FIRST] = Some(batter_lineup_index);
                        base_movements.push(BaseMovement {
                            starting_base: None,
                            bases_moved: MoveType::Advanced(1),
                        });

                        // Runner on second stays put
                        new_base_state[Consts::SECOND] = base_state[Consts::SECOND];
                        new_base_state[Consts::THIRD] = base_state[Consts::THIRD];
                    }

                    if base_state.iter().all(|b| b.is_some()) {
                        // Forces everywhere, player runs home
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::THIRD),
                            bases_moved: MoveType::Advanced(1),
                        });
                    }

                    if base_state[Consts::SECOND].is_some() && base_state[Consts::FIRST].is_some() {
                        let runner_from_second = base_state[Consts::SECOND];

                        // Force at third, player runs to third
                        new_base_state[Consts::THIRD] = runner_from_second;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::SECOND),
                            bases_moved: MoveType::Advanced(1),
                        });
                    }
                },
                1 => {
                    // Batter has time to get to first for sure
                    new_base_state[Consts::FIRST] = Some(batter_lineup_index);
                    base_movements.push(BaseMovement {
                        starting_base: None,
                        bases_moved: MoveType::Advanced(1),
                    });

                    let first_batter_speed = decider.roll_stat(*levels::BASERUNNER_SPEED, Skill::default());
                    let first_takeoff_delay = decider.roll_stat(*levels::BASE_TAKEOFF_DELAY, Skill::default());
                    let first_to_second_run_time = 90.0 / first_batter_speed + first_takeoff_delay;
                    if first_to_second_run_time > fielding_play.to_event.travel_time.0 && base_state[Consts::FIRST].is_some() {
                        // Force at second, player is out at second
                        batter_hit_type = HitType::FieldersChoice;
                        outs_made += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::FIRST),
                            bases_moved: MoveType::Out(Consts::SECOND),
                        });
                    } else {
                        let runner_from_first = base_state[Consts::FIRST];

                        // Force at second, player is safe at second
                        batter_hit_type = HitType::Single;
                        new_base_state[Consts::SECOND] = runner_from_first;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::FIRST),
                            bases_moved: MoveType::Advanced(1),
                        });
                    }

                    if base_state[Consts::SECOND].is_some() && base_state[Consts::FIRST].is_some() {
                        let runner_from_second = base_state[Consts::SECOND];

                        // Force at third
                        new_base_state[Consts::THIRD] = runner_from_second;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::SECOND),
                            bases_moved: MoveType::Advanced(1),
                        });
                    }

                    if base_state.iter().all(|b| b.is_some()) {
                        // Force at home
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::THIRD),
                            bases_moved: MoveType::Advanced(1),
                        });
                    }
                },
                2 => {
                    // Batter stretches for a double
                    batter_hit_type = HitType::Double;
                    if base_state[Consts::FIRST].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::FIRST),
                            bases_moved: MoveType::Advanced(3),
                        });
                    }

                    if base_state[Consts::SECOND].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::SECOND),
                            bases_moved: MoveType::Advanced(2),
                        });
                    }

                    if base_state[Consts::THIRD].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::THIRD),
                            bases_moved: MoveType::Advanced(1),
                        });
                    }

                    new_base_state[Consts::SECOND] = Some(batter_lineup_index);
                    base_movements.push(BaseMovement {
                        starting_base: None,
                        bases_moved: MoveType::Advanced(2),
                    });
                },
                3 => {
                    // Batter hits a triple
                    batter_hit_type = HitType::Triple;
                    if base_state[Consts::FIRST].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::FIRST),
                            bases_moved: MoveType::Advanced(3),
                        });
                    }

                    if base_state[Consts::SECOND].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::SECOND),
                            bases_moved: MoveType::Advanced(2),
                        });
                    }

                    if base_state[Consts::THIRD].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::THIRD),
                            bases_moved: MoveType::Advanced(1),
                        });
                    }

                    new_base_state[Consts::THIRD] = Some(batter_lineup_index);
                    base_movements.push(BaseMovement {
                        starting_base: None,
                        bases_moved: MoveType::Advanced(3),
                    });
                },
                _ => {
                    // Batter hits an inside the park home run
                    batter_hit_type = HitType::HomeRun;
                    if base_state[Consts::FIRST].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::FIRST),
                            bases_moved: MoveType::Advanced(3),
                        });
                    }

                    if base_state[Consts::SECOND].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::SECOND),
                            bases_moved: MoveType::Advanced(2),
                        });
                    }

                    if base_state[Consts::THIRD].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::THIRD),
                            bases_moved: MoveType::Advanced(1),
                        });
                    }

                    runs_scored += 1;
                    base_movements.push(BaseMovement {
                        starting_base: None,
                        bases_moved: MoveType::Advanced(4),
                    });
                }
            }
            base_state = new_base_state;
        },
        Consts::THIRD => {
            let batter_advanced = batter_advanced(
                fielding_play.to_event.travel_time.0,
                batter_runner_speed,
                batter_box_exit_time,
            );

            let mut new_base_state = [None; 3];
            match batter_advanced {
                0 => {
                    if base_state[Consts::FIRST].is_some() && base_state[Consts::SECOND].is_some() {
                        let runner_from_first = base_state[Consts::FIRST];
                        let runner_from_second = base_state[Consts::SECOND];

                        // Force at third
                        let second_runner_speed = decider.roll_stat(*levels::BASERUNNER_SPEED, Skill::default());
                        let second_runner_takeoff_delay = decider.roll_stat(*levels::BASE_TAKEOFF_DELAY, Skill::default());
                        let second_to_third_run_time = 90.0 / second_runner_speed + second_runner_takeoff_delay;
                        if second_to_third_run_time > fielding_play.to_event.travel_time.0 {
                            // Player is out at third
                            outs_made += 1;
                            base_movements.push(BaseMovement {
                                starting_base: Some(Consts::SECOND),
                                bases_moved: MoveType::Out(Consts::THIRD),
                            });

                            let third_baseman_throw_speed = decider.roll_stat(
                                *levels::THROW_SPEED, Skill::default()
                            );
                            let third_baseman_transfer_time = decider.roll_stat(
                                *levels::FIELDER_TRANSFER_TIME, Skill::default()
                            );
                            let third_to_second_base_throw_time = 90.0 / third_baseman_throw_speed +
                                third_baseman_transfer_time;
                            if third_to_second_base_throw_time >
                                fielding_play.to_event.travel_time.0 + third_to_second_base_throw_time {
                                // Double Play at second
                                outs_made += 1;
                                base_movements.push(BaseMovement {
                                    starting_base: Some(Consts::FIRST),
                                    bases_moved: MoveType::Out(Consts::SECOND),
                                });
                                let second_baseman_throw_speed = decider.roll_stat(
                                    *levels::THROW_SPEED, Skill::default()
                                );
                                let second_baseman_transfer_time = decider.roll_stat(
                                    *levels::FIELDER_TRANSFER_TIME, Skill::default()
                                );
                                let second_to_first_base_throw_time = 90.0 / second_baseman_throw_speed +
                                    second_baseman_transfer_time;
                                let batter_run_time = 90.0 / batter_runner_speed + batter_box_exit_time;
                                if batter_run_time >
                                    fielding_play.to_event.travel_time.0 +
                                        third_to_second_base_throw_time +
                                        second_to_first_base_throw_time {
                                    // Around the horn triple play
                                    batter_hit_type = HitType::Out;
                                    outs_made += 1;
                                    base_movements.push(BaseMovement {
                                        starting_base: None,
                                        bases_moved: MoveType::Out(Consts::FIRST),
                                    });
                                } else {
                                    // Only gets two
                                    batter_hit_type = HitType::FieldersChoice;
                                    base_movements.push(BaseMovement {
                                        starting_base: None,
                                        bases_moved: MoveType::Advanced(1),
                                    });
                                }
                            } else {
                                // No double Play at second

                                new_base_state[Consts::SECOND] = runner_from_first;
                                base_movements.push(BaseMovement {
                                    starting_base: Some(Consts::FIRST),
                                    bases_moved: MoveType::Advanced(1),
                                });

                                let third_baseman_throw_speed = decider.roll_stat(
                                    *levels::THROW_SPEED, Skill::default()
                                );
                                let third_baseman_transfer_time = decider.roll_stat(
                                    *levels::FIELDER_TRANSFER_TIME, Skill::default()
                                );
                                let third_to_first_distance = 2.0 * 90.0 / 2.0f64.sqrt();
                                let third_first_throw_time = third_to_first_distance /
                                    third_baseman_throw_speed + third_baseman_transfer_time;

                                let batter_run_time = 90.0 / batter_runner_speed + batter_box_exit_time;
                                if batter_run_time > third_first_throw_time {
                                    // Third to First double play
                                    batter_hit_type = HitType::Out;
                                    outs_made += 1;
                                    base_movements.push(BaseMovement {
                                        starting_base: None,
                                        bases_moved: MoveType::Out(Consts::FIRST),
                                    });
                                } else {
                                    // No double play
                                    batter_hit_type = HitType::FieldersChoice;
                                    new_base_state[Consts::FIRST] = runner_from_first;
                                    base_movements.push(BaseMovement {
                                        starting_base: None,
                                        bases_moved: MoveType::Advanced(1),
                                    });
                                }
                            }
                        } else {
                            // Player is safe at third
                            batter_hit_type = HitType::Single;
                            new_base_state[Consts::THIRD] = runner_from_second;
                            base_movements.push(BaseMovement {
                                starting_base: Some(Consts::SECOND),
                                bases_moved: MoveType::Advanced(1),
                            });
                        }
                    } else {
                        // No runner going from Second to Third
                        batter_hit_type = HitType::Single;
                        new_base_state[Consts::FIRST] = Some(batter_lineup_index);
                        base_movements.push(BaseMovement {
                            starting_base: None,
                            bases_moved: MoveType::Advanced(1),
                        });

                        if base_state[Consts::FIRST].is_some() {
                            new_base_state[Consts::SECOND] = base_state[Consts::FIRST];
                            base_movements.push(BaseMovement {
                                starting_base: Some(Consts::FIRST),
                                bases_moved: MoveType::Advanced(1),
                            });
                        }

                        new_base_state[Consts::THIRD] = base_state[Consts::THIRD];
                    }

                    if base_state.iter().all(|b| b.is_some()) {
                        // Forces everywhere, player runs home
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::THIRD),
                            bases_moved: MoveType::Advanced(1),
                        });
                    }
                },
                1 => {
                    // Batter has time to get to first for sure
                    new_base_state[Consts::FIRST] = Some(batter_lineup_index);
                    base_movements.push(BaseMovement {
                        starting_base: None,
                        bases_moved: MoveType::Advanced(1),
                    });
                    new_base_state[Consts::SECOND] = base_state[Consts::FIRST];
                    base_movements.push(BaseMovement {
                        starting_base: Some(Consts::FIRST),
                        bases_moved: MoveType::Advanced(1),
                    });

                    let second_runner_speed = decider.roll_stat(*levels::BASERUNNER_SPEED, Skill::default());
                    let second_runner_takeoff_delay = decider.roll_stat(*levels::BASE_TAKEOFF_DELAY, Skill::default());
                    let second_to_third_run_time = 90.0 / second_runner_speed + second_runner_takeoff_delay;
                    if second_to_third_run_time > fielding_play.to_event.travel_time.0 &&
                            base_state[Consts::SECOND].is_some() &&
                            base_state[Consts::THIRD].is_some() {
                        // Force at third, player is out at third
                        batter_hit_type = HitType::FieldersChoice;
                        outs_made += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::SECOND),
                            bases_moved: MoveType::Out(Consts::THIRD),
                        });
                    } else {
                        // Force at third, player is safe at third
                        batter_hit_type = HitType::Single;
                        new_base_state[Consts::THIRD] = base_state[Consts::SECOND];
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::SECOND),
                            bases_moved: MoveType::Advanced(1),
                        });
                    }

                    if base_state.iter().all(|b| b.is_some()) {
                        // Force at home
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::THIRD),
                            bases_moved: MoveType::Advanced(1),
                        });
                    }
                },
                2 => {
                    // Batter stretches for a double
                    batter_hit_type = HitType::Double;
                    if base_state[Consts::FIRST].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::FIRST),
                            bases_moved: MoveType::Advanced(3),
                        });
                    }

                    if base_state[Consts::SECOND].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::SECOND),
                            bases_moved: MoveType::Advanced(2),
                        });
                    }

                    if base_state[Consts::THIRD].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::THIRD),
                            bases_moved: MoveType::Advanced(1),
                        });
                    }

                    new_base_state[Consts::SECOND] = Some(batter_lineup_index);
                    base_movements.push(BaseMovement {
                        starting_base: None,
                        bases_moved: MoveType::Advanced(2),
                    });
                },
                3 => {
                    // Batter hits a triple
                    batter_hit_type = HitType::Triple;
                    if base_state[Consts::FIRST].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::FIRST),
                            bases_moved: MoveType::Advanced(3),
                        });
                    }

                    if base_state[Consts::SECOND].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::SECOND),
                            bases_moved: MoveType::Advanced(2),
                        });
                    }

                    if base_state[Consts::THIRD].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::THIRD),
                            bases_moved: MoveType::Advanced(1),
                        });
                    }

                    new_base_state[Consts::THIRD] = Some(batter_lineup_index);
                    base_movements.push(BaseMovement {
                        starting_base: None,
                        bases_moved: MoveType::Advanced(3),
                    });
                },
                _ => {
                    // Batter hits an inside the park home run
                    batter_hit_type = HitType::HomeRun;
                    if base_state[Consts::FIRST].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::FIRST),
                            bases_moved: MoveType::Advanced(3),
                        });
                    }

                    if base_state[Consts::SECOND].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::SECOND),
                            bases_moved: MoveType::Advanced(2),
                        });
                    }

                    if base_state[Consts::THIRD].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::THIRD),
                            bases_moved: MoveType::Advanced(1),
                        });
                    }

                    runs_scored += 1;
                    base_movements.push(BaseMovement {
                        starting_base: None,
                        bases_moved: MoveType::Advanced(4),
                    });
                }
            }
            base_state = new_base_state;
        }
        Consts::HOME => {
            let batter_advanced = batter_advanced(
                fielding_play.to_event.travel_time.0,
                batter_runner_speed,
                batter_box_exit_time,
            );

            let mut new_base_state = [None; 3];
            match batter_advanced {
                0 => {
                    if base_state.iter().all(|b| b.is_some()) {
                        // Force at home
                        let third_to_home_speed = decider.roll_stat(*levels::BASERUNNER_SPEED, Skill::default());
                        let third_to_home_takeoff = decider.roll_stat(*levels::BASE_TAKEOFF_DELAY, Skill::default());
                        let third_to_home_run_time = 90.0 / third_to_home_speed + third_to_home_takeoff;
                        if third_to_home_run_time > fielding_play.to_event.travel_time.0 {
                            // Player is out at home
                            outs_made += 1;
                            base_movements.push(BaseMovement {
                                starting_base: Some(Consts::THIRD),
                                bases_moved: MoveType::Out(Consts::HOME),
                            });

                            let home_to_third_throw_time = decider.roll_stat(*levels::THROW_SPEED, Skill::default());
                            let home_transfer_time = decider.roll_stat(*levels::FIELDER_TRANSFER_TIME, Skill::default());
                            let home_to_third_throw_time = 90.0 / home_to_third_throw_time +
                                home_transfer_time;

                            let second_to_third_speed = decider.roll_stat(*levels::BASERUNNER_SPEED, Skill::default());
                            let second_to_third_takeoff = decider.roll_stat(*levels::BASE_TAKEOFF_DELAY, Skill::default());
                            let second_to_third_run_time = 90.0 / second_to_third_speed + second_to_third_takeoff;
                            if second_to_third_run_time > fielding_play.to_event.travel_time.0 + home_to_third_throw_time {
                                // Double Play at third
                                outs_made += 1;
                                base_movements.push(BaseMovement {
                                    starting_base: Some(Consts::SECOND),
                                    bases_moved: MoveType::Out(Consts::THIRD),
                                });

                                let third_to_first_distance = 2.0 * 90.0 / 2.0f64.sqrt();
                                let third_to_first_throw_speed = decider.roll_stat(
                                    *levels::THROW_SPEED, Skill::default()
                                );
                                let third_to_first_transfer = decider.roll_stat(
                                    *levels::FIELDER_TRANSFER_TIME, Skill::default()
                                );
                                let third_to_first_throw_time = third_to_first_distance /
                                    third_to_first_throw_speed + third_to_first_transfer;

                                let batter_run_time = 90.0 / batter_runner_speed + batter_box_exit_time;
                                if batter_run_time > third_to_first_throw_time {
                                    // 2-5-4 Triple Play
                                    batter_hit_type = HitType::FieldersChoice;
                                    outs_made += 1;
                                    base_movements.push(BaseMovement {
                                        starting_base: Some(Consts::FIRST),
                                        bases_moved: MoveType::Out(Consts::SECOND),
                                    });
                                    base_movements.push(BaseMovement {
                                        starting_base: None,
                                        bases_moved: MoveType::Advanced(1),
                                    });
                                } else {
                                    // Double play only
                                    batter_hit_type = HitType::FieldersChoice;
                                    base_movements.push(BaseMovement {
                                        starting_base: None,
                                        bases_moved: MoveType::Advanced(1),
                                    });
                                }
                            } else {
                                // No double Play at third

                                new_base_state[Consts::THIRD] = base_state[Consts::SECOND];
                                base_movements.push(BaseMovement {
                                    starting_base: Some(Consts::SECOND),
                                    bases_moved: MoveType::Advanced(1),
                                });
                                new_base_state[Consts::SECOND] = base_state[Consts::FIRST];
                                base_movements.push(BaseMovement {
                                    starting_base: Some(Consts::FIRST),
                                    bases_moved: MoveType::Advanced(1),
                                });

                                let home_first_throw_speed = decider.roll_stat(
                                    *levels::THROW_SPEED, Skill::default()
                                );
                                let home_first_transfer = decider.roll_stat(
                                    *levels::FIELDER_TRANSFER_TIME, Skill::default()
                                );
                                let home_first_throw_time = 90.0 / home_first_throw_speed + home_first_transfer;
                                let batter_run_time = 90.0 / batter_runner_speed + batter_box_exit_time;

                                if batter_run_time > home_first_throw_time {
                                    // Home to First double play
                                    batter_hit_type = HitType::Out;
                                    outs_made += 1;
                                    base_movements.push(BaseMovement {
                                        starting_base: None,
                                        bases_moved: MoveType::Out(Consts::FIRST),
                                    });
                                } else {
                                    // No double play
                                    batter_hit_type = HitType::FieldersChoice;
                                    new_base_state[Consts::FIRST] = Some(batter_lineup_index);
                                    base_movements.push(BaseMovement {
                                        starting_base: None,
                                        bases_moved: MoveType::Advanced(1),
                                    });
                                }
                            }
                        } else {
                            // Player is safe at Home
                            batter_hit_type = HitType::Single;
                            runs_scored += 1;
                            base_movements.push(BaseMovement {
                                starting_base: Some(Consts::THIRD),
                                bases_moved: MoveType::Advanced(1),
                            });

                            new_base_state[Consts::THIRD] = base_state[Consts::SECOND];
                            base_movements.push(BaseMovement {
                                starting_base: Some(Consts::SECOND),
                                bases_moved: MoveType::Advanced(1),
                            });

                            new_base_state[Consts::SECOND] = base_state[Consts::FIRST];
                            base_movements.push(BaseMovement {
                                starting_base: Some(Consts::FIRST),
                                bases_moved: MoveType::Advanced(1),
                            });

                            new_base_state[Consts::FIRST] = Some(batter_lineup_index);
                            base_movements.push(BaseMovement {
                                starting_base: None,
                                bases_moved: MoveType::Advanced(1),
                            });
                        }
                    } else {
                        // No force at home
                        batter_hit_type = HitType::Single;
                        new_base_state[Consts::FIRST] = Some(batter_lineup_index);
                        base_movements.push(BaseMovement {
                            starting_base: None,
                            bases_moved: MoveType::Advanced(1),
                        });

                        if base_state[Consts::FIRST].is_some() {
                            new_base_state[Consts::SECOND] = base_state[Consts::FIRST];
                            base_movements.push(BaseMovement {
                                starting_base: Some(Consts::FIRST),
                                bases_moved: MoveType::Advanced(1),
                            });
                        }

                        if base_state[Consts::SECOND].is_some() {
                            new_base_state[Consts::THIRD] = base_state[Consts::SECOND];
                            base_movements.push(BaseMovement {
                                starting_base: Some(Consts::SECOND),
                                bases_moved: MoveType::Advanced(1),
                            });
                        }
                    }
                },
                1 => {
                    // Batter has time to get to first for sure
                    batter_hit_type = HitType::Single;
                    new_base_state[Consts::FIRST] = Some(batter_lineup_index);

                    if base_state[Consts::FIRST].is_some() {
                        new_base_state[Consts::SECOND] = base_state[Consts::FIRST];
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::FIRST),
                            bases_moved: MoveType::Advanced(1),
                        });
                    }

                    if base_state[Consts::SECOND].is_some() {
                        let second_run_speed = decider.roll_stat(*levels::BASERUNNER_SPEED, Skill::default());
                        let second_takeoff = decider.roll_stat(*levels::BASE_TAKEOFF_DELAY, Skill::default());
                        let second_home_run_time = (2.0 * 90.0) / second_run_speed + second_takeoff;

                        if second_home_run_time > fielding_play.to_event.travel_time.0 {
                            // Scores from second on a single
                            runs_scored += 1;
                            base_movements.push(BaseMovement {
                                starting_base: Some(Consts::SECOND),
                                bases_moved: MoveType::Advanced(2),
                            });
                        } else {
                            // Stays at third
                            new_base_state[Consts::THIRD] = base_state[Consts::SECOND];
                            base_movements.push(BaseMovement {
                                starting_base: Some(Consts::SECOND),
                                bases_moved: MoveType::Advanced(1),
                            });
                        }
                    }

                    if base_state[Consts::THIRD].is_some() {
                        // Safe at home
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::THIRD),
                            bases_moved: MoveType::Advanced(1),
                        });
                    }
                },
                2 => {
                    // Batter stretches for a double
                    batter_hit_type = HitType::Double;
                    if base_state[Consts::FIRST].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::FIRST),
                            bases_moved: MoveType::Advanced(3),
                        });
                    }

                    if base_state[Consts::SECOND].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::SECOND),
                            bases_moved: MoveType::Advanced(2),
                        });
                    }

                    if base_state[Consts::THIRD].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::THIRD),
                            bases_moved: MoveType::Advanced(1),
                        });
                    }

                    new_base_state[Consts::SECOND] = Some(batter_lineup_index);
                    base_movements.push(BaseMovement {
                        starting_base: None,
                        bases_moved: MoveType::Advanced(2),
                    });
                },
                3 => {
                    // Batter hits a triple
                    batter_hit_type = HitType::Triple;
                    if base_state[Consts::FIRST].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::FIRST),
                            bases_moved: MoveType::Advanced(3),
                        });
                    }

                    if base_state[Consts::SECOND].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::SECOND),
                            bases_moved: MoveType::Advanced(2),
                        });
                    }

                    if base_state[Consts::THIRD].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::THIRD),
                            bases_moved: MoveType::Advanced(1),
                        });
                    }

                    new_base_state[Consts::THIRD] = Some(batter_lineup_index);
                    base_movements.push(BaseMovement {
                        starting_base: None,
                        bases_moved: MoveType::Advanced(3),
                    });
                },
                _ => {
                    // Batter hits an inside the park home run
                    batter_hit_type = HitType::HomeRun;
                    if base_state[Consts::FIRST].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::FIRST),
                            bases_moved: MoveType::Advanced(3),
                        });
                    }

                    if base_state[Consts::SECOND].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::SECOND),
                            bases_moved: MoveType::Advanced(2),
                        });
                    }

                    if base_state[Consts::THIRD].is_some() {
                        runs_scored += 1;
                        base_movements.push(BaseMovement {
                            starting_base: Some(Consts::THIRD),
                            bases_moved: MoveType::Advanced(1),
                        });
                    }

                    runs_scored += 1;
                    base_movements.push(BaseMovement {
                        starting_base: None,
                        bases_moved: MoveType::Advanced(4),
                    });
                }
            }
            base_state = new_base_state;
        }
        _ => unreachable!()
    };

    BaseRunningRecord {
        movements: base_movements.into_boxed_slice(),
        outcome: BaseRunningOutcome {
            outs_made,
            runs_scored,
            batter_hit_type,
            ending_base_state: base_state,
        }
    }
}

fn batter_advanced(throw_time: f64, baserunner_speed: f64, box_exit_time: f64) -> u8 {
    let bases_ran = (throw_time * baserunner_speed + box_exit_time) / 90.0;
    bases_ran.trunc() as u8
}


