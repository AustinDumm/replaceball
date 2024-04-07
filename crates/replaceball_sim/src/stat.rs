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
    pub average_shift: f64,
    pub std_dev_multiplier: f64,
}

impl Default for Skill {
    fn default() -> Self {
        Self {
            average_multiplier: 1.0,
            average_shift: 0.0,
            std_dev_multiplier: 1.0,
        }
    }
}

impl Skill {
    pub fn std_dev_bias_skill(bias: i8, stat: Stat) -> Self {
        let bias_percent = bias as f64 / -std::i8::MIN as f64;

        Self {
            average_multiplier: 1.0,
            average_shift: bias_percent * stat.std_dev,
            std_dev_multiplier: 1.0,
        }
    }
}
