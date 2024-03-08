use std::ops::{RangeInclusive, Div, Mul, Add, Sub};

use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Distance(pub f64);

#[derive(Clone, Copy, Debug, PartialEq)]
/// Angle in degrees from left foul line to right foul line that the ball was hit.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HitDirection(pub f64);

impl HitDirection {
    const MAX_ANGLE: f64 = 90.0;

    pub fn from_decider(decider: &mut impl Decider) -> Self {
        Self(decider.roll_uniform(0.0..Self::MAX_ANGLE))
    }
}


#[derive(Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Location {
    pub direction: HitDirection,
    pub distance: Distance,
}

impl Location {
    pub fn first_base() -> Self {
        Self {
            direction: HitDirection(89.0),
            distance: Distance(90.0),
        }
    }

    pub fn second_base() -> Self {
        Self {
            direction: HitDirection(45.0),
            distance: Distance(127.25),
        }
    }

    pub fn third_base() -> Self {
        Self {
            direction: HitDirection(1.0),
            distance: Distance(90.0),
        }
    }

    pub fn home_plate() -> Self {
        Self {
            direction: HitDirection(45.0),
            distance: Distance(1.0),
        }
    }

    pub fn square_distance(
        self,
        rhs: Self,
    ) -> f64 {
        Cartesian::from(self).square_distance(rhs.into())
    }

    pub fn distance(
        self,
        rhs: Self,
    ) -> f64 {
        Cartesian::from(self).distance(rhs.into())
    }
}

impl From<Cartesian> for Location {
    fn from(value: Cartesian) -> Self {
        Self {
            direction: HitDirection(value.1.atan2(value.0).to_degrees()),
            distance: Distance((value.0.powf(2.0) + value.1.powf(2.0)).sqrt()),
        }
    }
}

pub fn clamp(value: f64, between: RangeInclusive<f64>) -> f64 {
    if value < *between.start() {
        *between.start()
    } else if *between.end() < value {
        *between.end()
    } else {
        value
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Cartesian(pub f64, pub f64);

impl Div<f64> for Cartesian {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl Cartesian {
    pub fn square_distance(
        self,
        rhs: Self,
    ) -> f64 {
        (self.0 - rhs.0).powf(2.0) + (self.1 - rhs.1).powf(2.0)
    }

    pub fn distance(
        self,
        rhs: Self,
    ) -> f64 {
        self.square_distance(rhs).sqrt()
    }

    pub fn dot(
        self,
        rhs: Self,
    ) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1
    }

    pub fn sub(
        self,
        rhs: Self,
    ) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }

    pub fn add(
        self,
        rhs: Self,
    ) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }

    pub fn magnitude(&self) -> f64 {
        (self.0.powf(2.0) + self.1.powf(2.0)).sqrt()
    }

    pub fn normalized(self) -> Self {
        let magnitude = self.magnitude();
        Self(self.0 / magnitude, self.1 / magnitude)
    }

    pub fn fielding_location(
        self,
        speed: f64,
        ball_location: Self,
        ball_velocity: Self,
    ) -> Option<Cartesian> {
        let ball_angle = ball_velocity.1.atan2(ball_velocity.0);
        let self_ball_vector = ball_location - self;
        let self_ball_angle = self_ball_vector.1.atan2(self_ball_vector.0);
        let ball_speed = ball_velocity.magnitude();

        let a = 1.0;
        let b = 2.0 * ball_speed * (ball_angle - self_ball_angle).cos();
        let c = ball_speed.powf(2.0) - speed.powf(2.0);

        let discriminant = b.powf(2.0) - 4.0 * a * c;
        if discriminant < 0.0 {
            return None
        }

        let first_result = (-b - discriminant.sqrt()) / 2.0 * a;
        let first_sin = ((ball_speed * ball_angle.sin()) + first_result * self_ball_angle.sin()) / speed;
        let first_angle = if first_sin < -1.0 { -1.0 } else if 1.0 < first_sin { 1.0 } else { first_sin }
            .asin();
        let first_time = (self.1 - ball_location.1) / (ball_speed * (ball_angle.sin() - first_angle.sin()));

        let second_result = (-b + discriminant.sqrt()) / 2.0 * a;
        let second_sin = ((ball_speed * ball_angle.sin()) + second_result * self_ball_angle.sin()) / speed;
        let second_angle = if second_sin < -1.0 { -1.0 } else if 1.0 < second_sin { 1.0 } else { second_sin }
            .asin();
        let second_time = (self.1 - ball_location.1) / (ball_speed * (ball_angle.sin() - second_angle.sin()));

        let (faster_angle, faster_time) = if first_time < second_time {
            (first_angle, first_time)
        } else {
            (second_angle, second_time)
        };
        if faster_angle.is_nan() || faster_time.is_nan() {
            panic!("{}, {}", faster_angle, faster_time)
        }
        let player_move_vector = Cartesian(faster_angle.cos(), faster_angle.sin()) * speed * faster_time;
        let fielding_vector = self + player_move_vector;
        
        
        Some(fielding_vector.into())
    }
}

impl From<Location> for Cartesian {
    fn from(value: Location) -> Self {
        Cartesian(
            value.direction.0.to_radians().cos() * value.distance.0,
            value.direction.0.to_radians().sin()
        )
    }
}

impl Mul<f64> for Cartesian {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self(self.0 * scalar, self.0 * scalar)
    }
}

impl Add<Self> for Cartesian {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Self> for Cartesian {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}
