use std::ops::Mul;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stat {
    pub average: f64,
    pub std_dev: f64,
    pub range: (f64, f64),
}

impl Mul<Skill> for Stat {
    type Output = Self;

    fn mul(self, rhs: Skill) -> Self::Output {
        Self {
            average: self.average * rhs.average_multiplier,
            std_dev: self.std_dev * rhs.std_dev_multiplier,
            range: self.range,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Skill {
    pub average_multiplier: f64,
    pub std_dev_multiplier: f64,
}

impl Default for Skill {
    fn default() -> Self {
        Self { average_multiplier: 1.0, std_dev_multiplier: 1.0 }
    }
}

