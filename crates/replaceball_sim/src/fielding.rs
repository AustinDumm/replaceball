use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use ts_rs::TS;

use crate::{base_running, player::Team, prelude::*};

#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumIter, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum Fielder {
    Catcher,
    Pitcher,
    FirstBase,
    SecondBase,
    ThirdBase,
    Shortstop,
    LeftFielder,
    CenterFielder,
    RightFielder,
}

impl Fielder {
    pub fn starting_location(&self) -> Location {
        match self {
            Fielder::Catcher => Location {
                direction: HitDirection(0.0),
                distance: Distance(0.0),
            },
            Fielder::Pitcher => Location {
                direction: HitDirection(45.0),
                distance: Distance(60.5),
            },
            Fielder::FirstBase => Location {
                direction: HitDirection(83.5),
                distance: Distance(105.0),
            },
            Fielder::SecondBase => Location {
                direction: HitDirection(57.5),
                distance: Distance(125.0),
            },
            Fielder::ThirdBase => Location {
                direction: HitDirection(6.5),
                distance: Distance(105.0),
            },
            Fielder::Shortstop => Location {
                direction: HitDirection(32.5),
                distance: Distance(125.0),
            },
            Fielder::LeftFielder => Location {
                direction: HitDirection(19.5),
                distance: Distance(255.0),
            },
            Fielder::CenterFielder => Location {
                direction: HitDirection(45.0),
                distance: Distance(255.0),
            },
            Fielder::RightFielder => Location {
                direction: HitDirection(70.5),
                distance: Distance(255.0),
            },
        }
    }
}

impl Display for Fielder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Fielder::Catcher => "Catcher".to_owned(),
            Fielder::Pitcher => "Pitcher".to_owned(),
            Fielder::FirstBase => "First Baseman".to_owned(),
            Fielder::SecondBase => "Second Baseman".to_owned(),
            Fielder::ThirdBase => "Third Baseman".to_owned(),
            Fielder::Shortstop => "Shortstop".to_owned(),
            Fielder::LeftFielder => "Left Fielder".to_owned(),
            Fielder::CenterFielder => "Center Fielder".to_owned(),
            Fielder::RightFielder => "Right Fielder".to_owned(),
        };

        write!(f, "{}", name)
    }
}

#[derive(Clone, PartialEq, Debug, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FieldingRecord {
    pub landing: BallLanding,
    pub base_running_record: BaseRunningRecord,
}

#[derive(Clone, PartialEq, Debug, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BallLanding {
    Out(Fielder, Location),
    Landed(Location, FieldingPlay),
}

#[derive(Clone, Copy, PartialEq, Debug, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TravelTime(pub f64);

#[derive(Clone, PartialEq, Debug, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FieldingEvent {
    pub location: Location,

    /// Time since contact was made
    pub travel_time: TravelTime,
}

#[derive(Clone, PartialEq, Debug, TS)]
#[ts(export)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FieldingPlay {
    pub from: Fielder,
    pub from_event: FieldingEvent,
    pub to: Fielder,
    pub to_event: FieldingEvent,
    pub base: usize,
}

pub fn simulate_fielding(
    direction: HitDirection,
    launch_angle: LaunchAngle,
    exit_speed: Speed,
    batter_lineup_index: u8,
    batting_team: &Team,
    fielding_team: &Team,
    base_state: &[Option<u8>; 3],
    decider: &mut impl Decider,
) -> HitOutcome {
    let hit_locations = ball_path(direction, launch_angle, exit_speed);

    if (hit_locations.catchable_path.location.direction.0 < 30.0
        && hit_locations.catchable_path.location.distance.0 > 370.0)
        || (30.0 <= hit_locations.catchable_path.location.direction.0
            && hit_locations.catchable_path.location.direction.0 < 60.0
            && hit_locations.catchable_path.location.distance.0 > 400.0)
        || (60.0 <= hit_locations.catchable_path.location.direction.0
            && hit_locations.catchable_path.location.distance.0 > 370.0)
    {
        return HitOutcome::HomeRun;
    }

    let mut eligible_fielders: Vec<_> = Fielder::iter()
        .map(|fielder| {
            (
                fielder.clone(),
                player_distance_to_catch_hit(&fielder, &hit_locations),
            )
        })
        .filter(|(fielder, distance)| {
            (distance.1 .0
                / Speed::from_decider(
                    decider,
                    *levels::FIELDER_SPEED,
                    fielding_team
                        .player_at_position(fielder)
                        .fielder_run_speed_bias,
                )
                .0)
                < hit_locations.catchable_path.travel_time.0
        })
        .collect();

    eligible_fielders.sort_by(|(_, (_, lh_distance)), (_, (_, rh_distance))| {
        lh_distance.0.partial_cmp(&rh_distance.0).unwrap()
    });

    if let Some((fielder, (location, _))) = eligible_fielders.first() {
        let landing = BallLanding::Out(*fielder, location.clone());
        let base_running_record = base_running::simulate_base_running(
            batter_lineup_index,
            batting_team,
            fielding_team,
            &landing,
            base_state,
            decider,
        );

        HitOutcome::InPlay(FieldingRecord {
            landing,
            base_running_record,
        })
    } else {
        HitOutcome::InPlay(hit(
            batter_lineup_index,
            batting_team,
            fielding_team,
            hit_locations.landed_path,
            base_state,
            decider,
        ))
    }
}

fn hit(
    batter_lineup_index: u8,
    batting_team: &Team,
    fielding_team: &Team,
    landed_at: FieldingEvent,
    base_state: &[Option<u8>; 3],
    decider: &mut impl Decider,
) -> FieldingRecord {
    let force_play_index = force_play(base_state);
    let (closest_fielder, fielded_at, distance_to_fielding, time_fielding) =
        closest_fielder(&landed_at, fielding_team, decider);
    let fielding_play = match force_play_index {
        0 => force_at_first(
            closest_fielder,
            distance_to_fielding,
            fielded_at,
            time_fielding,
            fielding_team,
            decider,
        ),
        // Force at second
        1 => force_at_second(
            closest_fielder,
            distance_to_fielding,
            fielded_at,
            time_fielding,
            fielding_team,
            decider,
        ),
        // Force at third
        2 => force_at_third(
            closest_fielder,
            distance_to_fielding,
            fielded_at,
            time_fielding,
            fielding_team,
            decider,
        ),
        // Force at home
        3 => force_at_home(
            closest_fielder,
            distance_to_fielding,
            fielded_at,
            time_fielding,
            fielding_team,
            decider,
        ),
        _ => unreachable!(),
    };

    let base_running_record = base_running::simulate_base_running(
        batter_lineup_index,
        batting_team,
        fielding_team,
        &BallLanding::Landed(fielded_at, fielding_play.clone()),
        base_state,
        decider,
    );

    FieldingRecord {
        landing: BallLanding::Landed(landed_at.location, fielding_play),
        base_running_record,
    }
}

fn throw_to_force(
    throw_start_time: TravelTime,
    from_fielder: Fielder,
    to_location: Location,
    to_fielder: Fielder,
    fielder: Fielder,
    _distance: Distance,
    fielded_at: Location,
    base_index: usize,
    fielding_team: &Team,
    decider: &mut impl Decider,
) -> FieldingPlay {
    let player = fielding_team.player_at_position(&from_fielder);
    let throw_time = TravelTime(
        (fielded_at.distance(to_location)
            / decider.roll_std_dev_skill_stat(*levels::THROW_SPEED, player.fielder_throw_speed_bias))
            + throw_start_time.0,
    );

    FieldingPlay {
        from: fielder,
        from_event: FieldingEvent {
            location: fielded_at.clone(),
            travel_time: throw_start_time,
        },
        to: to_fielder,
        to_event: FieldingEvent {
            location: to_location,
            travel_time: throw_time,
        },
        base: base_index,
    }
}

fn force_at_first(
    fielder: Fielder,
    distance: Distance,
    fielded_at: Location,
    fielded_at_time: TravelTime,
    fielding_team: &Team,
    decider: &mut impl Decider,
) -> FieldingPlay {
    throw_to_force(
        fielded_at_time,
        fielder,
        Location::first_base(),
        Fielder::FirstBase,
        fielder,
        distance,
        fielded_at,
        Consts::FIRST,
        fielding_team,
        decider,
    )
}

fn force_at_second(
    fielder: Fielder,
    distance: Distance,
    fielded_at: Location,
    fielded_at_time: TravelTime,
    fielding_team: &Team,
    decider: &mut impl Decider,
) -> FieldingPlay {
    let to_fielder = match fielder {
        Fielder::FirstBase | Fielder::SecondBase | Fielder::Pitcher | Fielder::RightFielder => {
            Fielder::Shortstop
        }
        Fielder::Shortstop | Fielder::ThirdBase | Fielder::Catcher | Fielder::LeftFielder => {
            Fielder::SecondBase
        }
        Fielder::CenterFielder => {
            if fielded_at.direction.0 < 45.0 {
                Fielder::SecondBase
            } else {
                Fielder::Shortstop
            }
        }
    };
    let to_location = Location::second_base();

    throw_to_force(
        fielded_at_time,
        fielder,
        to_location,
        to_fielder,
        fielder,
        distance,
        fielded_at,
        Consts::SECOND,
        fielding_team,
        decider,
    )
}

fn force_at_third(
    fielder: Fielder,
    distance: Distance,
    fielded_at: Location,
    fielded_at_time: TravelTime,
    fielding_team: &Team,
    decider: &mut impl Decider,
) -> FieldingPlay {
    throw_to_force(
        fielded_at_time,
        fielder,
        Location::third_base(),
        Fielder::ThirdBase,
        fielder,
        distance,
        fielded_at,
        Consts::THIRD,
        fielding_team,
        decider,
    )
}

fn force_at_home(
    fielder: Fielder,
    distance: Distance,
    fielded_at: Location,
    fielded_at_time: TravelTime,
    fielding_team: &Team,
    decider: &mut impl Decider,
) -> FieldingPlay {
    throw_to_force(
        fielded_at_time,
        fielder,
        Location::home_plate(),
        Fielder::Catcher,
        fielder,
        distance,
        fielded_at,
        Consts::HOME,
        fielding_team,
        decider,
    )
}

fn force_play(base_state: &[Option<u8>; 3]) -> usize {
    if base_state[0].is_some() {
        if base_state[1].is_some() {
            if base_state[2].is_some() {
                3
            } else {
                2
            }
        } else {
            1
        }
    } else {
        0
    }
}

fn closest_fielder(
    event: &FieldingEvent,
    fielding_team: &Team,
    decider: &mut impl Decider,
) -> (Fielder, Location, Distance, TravelTime) {
    let mut fielder_distances: Vec<_> = Fielder::iter()
        .filter_map(|fielder| {
            let player = fielding_team.player_at_position(&fielder);
            let reaction_time = decider.roll_std_dev_skill_stat(
                *levels::PLAYER_REACTION_TIME,
                player.fielder_reaction_time_bias,
            );

            let player_speed = decider.roll_std_dev_skill_stat(
                *levels::FIELDER_SPEED,
                player.fielder_run_speed_bias,
            );

            let ball_speed = event.location.distance.0 / event.travel_time.0;
            let ball_location_on_reaction = Location {
                direction: event.location.direction,
                distance: Distance(ball_speed * reaction_time),
            };

            let starting_location = Cartesian::from(fielder.starting_location());
            if let Some(fielding_location) = starting_location.fielding_location(
                player_speed,
                ball_location_on_reaction.into(),
                Location {
                    direction: event.location.direction,
                    distance: Distance(ball_speed),
                }
                .into(),
            ) {
                if fielding_location.magnitude() < event.location.distance.0 {
                    // Player can only get to the ball before it landed
                    None
                } else {
                    let fielding_location = Cartesian::from(fielding_location);
                    let travel_distance = fielding_location.sub(starting_location).magnitude();

                    let fielder_transfer =
                        decider.roll_std_dev_skill_stat(*levels::FIELDER_TRANSFER_TIME, player.fielder_transfer_time_bias);
                    Some((
                        fielder,
                        fielding_location.into(),
                        Distance(travel_distance),
                        TravelTime(
                            travel_distance / player_speed + reaction_time + fielder_transfer,
                        ),
                    ))
                }
            } else {
                None
            }
        })
        .collect();

    fielder_distances.sort_by(|(_, _, _, lh_distance), (_, _, _, rh_distance)| {
        lh_distance.0.partial_cmp(&rh_distance.0).unwrap()
    });
    if let Some(found) = fielder_distances.first() {
        *found
    } else {
        // No one could field the ball at top speed. Try fielding after landing at half speed
        let mut fielder_distances: Vec<_> = Fielder::iter()
            .filter_map(|fielder| {
                let player = fielding_team.player_at_position(&fielder);
                let player_speed = decider.roll_std_dev_skill_stat(*levels::FIELDER_SPEED, player.fielder_run_speed_bias);

                let ball_speed = event.location.distance.0 / event.travel_time.0;

                let starting_location = Cartesian::from(fielder.starting_location());
                if let Some(fielding_location) = starting_location.fielding_location(
                    player_speed,
                    event.location.into(),
                    Location {
                        direction: event.location.direction,
                        distance: Distance(ball_speed * 1.5),
                    }
                    .into(),
                ) {
                    if fielding_location.magnitude() < event.location.distance.0 {
                        // Player can only field the ball before it landed, can't get there
                        None
                    } else {
                        let travel_distance = fielding_location.sub(starting_location).magnitude();

                        let fielder_transfer =
                            decider.roll_std_dev_skill_stat(*levels::FIELDER_TRANSFER_TIME, player.fielder_transfer_time_bias);
                        Some((
                            fielder,
                            fielding_location.into(),
                            Distance(travel_distance),
                            TravelTime(
                                travel_distance / player_speed
                                    + event.travel_time.0
                                    + fielder_transfer,
                            ),
                        ))
                    }
                } else {
                    None
                }
            })
            .collect();

        fielder_distances.sort_by(|(_, _, _, lh_distance), (_, _, _, rh_distance)| {
            lh_distance.0.partial_cmp(&rh_distance.0).unwrap()
        });

        if let Some(found) = fielder_distances.first() {
            *found
        } else {
            // Still no one can field the ball. Get a player to the ball at the wall 
            let ball_distance =
                if event.location.direction.0 < 20.0 || 70.0 < event.location.direction.0 {
                    325.0
                } else if event.location.direction.0 < 30.0 || 60.0 < event.location.direction.0 {
                    // Alleys
                    425.0
                } else {
                    400.0
                };
            let ball_location = Location {
                direction: event.location.direction,
                distance: Distance(ball_distance),
            };
            let ball_vector = Cartesian::from(ball_location);

            let mut fielder_distances: Vec<_> = Fielder::iter()
                .map(|fielder| {
                    let player = fielding_team.player_at_position(&fielder);
                    let reaction_time =
                        decider.roll_std_dev_skill_stat(*levels::PLAYER_REACTION_TIME, player.fielder_reaction_time_bias);
                    let ball_speed = event.location.distance.0 / event.travel_time.0;
                    let distance = (ball_vector - fielder.starting_location().into()).magnitude();
                    let fielder_transfer =
                        decider.roll_std_dev_skill_stat(*levels::FIELDER_TRANSFER_TIME, player.fielder_transfer_time_bias);

                    (
                        fielder,
                        ball_location,
                        Distance(distance),
                        TravelTime(
                            ball_location.distance.0 / ball_speed
                                + reaction_time
                                + fielder_transfer,
                        ),
                    )
                })
                .collect();
            fielder_distances.sort_by(|(_, _, _, lh_distance), (_, _, _, rh_distance)| {
                lh_distance.0.partial_cmp(&rh_distance.0).unwrap()
            });

            *fielder_distances
                .first()
                .expect("Failed to find a fielder to field the ball at the wall")
        }
    }
}

struct HitLocations {
    /// FieldingEvent to the point when the ball is catchable
    /// TravelTime and location.distance will both be 0.0 if the ball is always catchable off of the bat
    pub catchable_path: FieldingEvent,

    /// FieldingEvent from the catchable_path point to when it lands
    /// If catchable_path is None, the path from contact to the ground
    pub landed_path: FieldingEvent,
}

fn ball_path(
    direction: HitDirection,
    launch_angle: LaunchAngle,
    exit_speed: Speed,
) -> HitLocations {
    let player_height = 6.0_f64;
    let catch_height = 8.0_f64;

    let catchable_path = if let Some([_, travel_time]) =
        travel_time_for_ball_height(launch_angle, exit_speed, player_height, catch_height)
    {
        if travel_time.0 < 0.0 {
            FieldingEvent {
                location: Location {
                    direction,
                    distance: Distance(0.0),
                },
                travel_time: TravelTime(0.0),
            }
        } else {
            FieldingEvent {
                location: Location {
                    direction,
                    distance: Distance(
                        launch_angle.0.to_radians().cos() * exit_speed.0 * travel_time.0,
                    ),
                },
                travel_time,
            }
        }
    } else {
        FieldingEvent {
            location: Location {
                direction,
                distance: Distance(0.0),
            },
            travel_time: TravelTime(0.0),
        }
    };

    let [_, landed_travel_time] =
        travel_time_for_ball_height(launch_angle, exit_speed, player_height, 0.0)
            .expect("Ball somehow never hits the ground.");
    let landed_distance =
        Distance(launch_angle.0.to_radians().cos() * exit_speed.0 * landed_travel_time.0);

    let landed_path = FieldingEvent {
        location: Location {
            direction,
            distance: landed_distance,
        },
        travel_time: TravelTime(landed_travel_time.0),
    };

    HitLocations {
        catchable_path,
        landed_path,
    }
}

fn travel_time_for_ball_height(
    launch_angle: LaunchAngle,
    exit_speed: Speed,
    initial_height: f64,
    travel_time_height: f64,
) -> Option<[TravelTime; 2]> {
    let gravity = -32.174_f64;
    let launch_angle = launch_angle.0;
    let exit_speed = exit_speed.0;

    let angle_sin = launch_angle.to_radians().sin();
    let determinant = angle_sin.powf(2.0) * exit_speed.powf(2.0)
        - 2.0 * gravity * (initial_height - travel_time_height);

    if determinant < 0.0 {
        None
    } else {
        let first_numerator = -angle_sin * exit_speed - determinant.sqrt();
        let second_numerator = -angle_sin * exit_speed + determinant.sqrt();
        let first = first_numerator / gravity;
        let second = second_numerator / gravity;

        let (lesser, greater) = if first < second {
            (first, second)
        } else {
            (second, first)
        };

        let lesser_travel_time = TravelTime(lesser);
        let greater_travel_time = TravelTime(greater);
        Some([lesser_travel_time, greater_travel_time])
    }
}

fn player_distance_to_catch_hit(
    fielder: &Fielder,
    hit_locations: &HitLocations,
) -> (Location, Distance) {
    let player_point = Cartesian::from(fielder.starting_location());
    let catchable_point = Cartesian::from(hit_locations.catchable_path.location);
    let landed_point = Cartesian::from(hit_locations.landed_path.location);

    let ball_distance_squared = catchable_point.square_distance(landed_point);

    if ball_distance_squared == 0.0 {
        return (
            landed_point.into(),
            Distance(player_point.distance(landed_point)),
        );
    }

    let point_t = location::clamp(
        player_point
            .sub(catchable_point)
            .dot(landed_point.sub(catchable_point))
            / ball_distance_squared,
        0.0..=1.0,
    );

    let projection = (landed_point.sub(catchable_point) * point_t).add(catchable_point);

    (
        projection.into(),
        Distance(player_point.distance(projection)),
    )
}
