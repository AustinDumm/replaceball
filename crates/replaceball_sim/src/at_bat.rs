use crate::{
    prelude::*,
    pitch::simulate_pitch,
    hit::simulate_hit,
};


#[derive(Clone, Debug)]
pub struct AtBatRecord {
    pub batter_index: u8,
    pub pitches: Box<[(PitchRecord, AtBatProgress)]>,
    pub outcome: AtBatOutcome,
}

#[derive(Default, Clone, Debug)]
pub struct AtBatProgress {
    pub balls: u8,
    pub strikes: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AtBatOutcomeType {
    Hit(HitRecord),
    Walk,
    Out,
}

#[derive(Clone, Debug)]
pub struct AtBatOutcome {
    pub outcome_type: AtBatOutcomeType,
}

pub fn simulate_at_bat(
    batter_index: u8,
    decider: &mut impl Decider,
    base_state: &[bool; 3],
) -> AtBatRecord {
    let mut state = AtBatState::new();
    let mut pitches = Vec::<(PitchRecord, AtBatProgress)>::new();

    while state.outcome_type().is_none() {
        let pitch_record = simulate_pitch(decider);

        match pitch_record.outcome {
            PitchOutcome::Strike(_) => state.strike(),
            PitchOutcome::Foul => state.foul(),
            PitchOutcome::Ball => state.ball(),
            PitchOutcome::Hit(is_ball) => handle_hit(
                &pitch_record.location,
                is_ball,
                decider,
                &mut state,
                base_state,
            ),
        }

        let progress = AtBatProgress {
            balls: state.current_balls(),
            strikes: state.current_strikes()
        };

        pitches.push((pitch_record, progress));
    }

    let outcome = AtBatOutcome {
        outcome_type: state.outcome_type().expect("At bat is not active but no outcome found.")
    };

    AtBatRecord {
        batter_index,
        pitches: pitches.into_boxed_slice(),
        outcome,
    }
}

fn handle_hit(
    pitch_location: &PitchLocation,
    is_ball: bool,
    decider: &mut impl Decider,
    state: &mut AtBatState,
    base_state: &[bool; 3],
) {
    let hit_record = simulate_hit(
        pitch_location,
        is_ball,
        decider,
        base_state,
    );

    state.hit(hit_record);
}


struct AtBatState {
    balls_remaining: u8,
    strikes_remaining: u8,
    hit_record: Option<HitRecord>,
}

impl AtBatState {
    fn new() -> Self {
        Self {
            balls_remaining: Consts::BALLS_PER_WALK,
            strikes_remaining: Consts::STRIKES_PER_STRIKEOUT,
            hit_record: None,
        }
    }

    fn current_balls(&self) -> u8 {
        Consts::BALLS_PER_WALK - self.balls_remaining
    }

    fn current_strikes(&self) -> u8 {
        Consts::STRIKES_PER_STRIKEOUT - self.strikes_remaining
    }

    fn ball(&mut self) {
        self.balls_remaining -= 1;
    }

    fn strike(&mut self) {
        self.strikes_remaining -= 1;
    }
    
    fn foul(&mut self) {
        if self.strikes_remaining != 1 {
            self.strikes_remaining -= 1;
        }
    }

    fn hit(&mut self, hit_record: HitRecord) {
        self.hit_record = Some(hit_record);
    }

    fn outcome_type(&self) -> Option<AtBatOutcomeType> {
        if let Some(hit_record) = self.hit_record.clone() {
            Some(AtBatOutcomeType::Hit(hit_record))
        } else if self.balls_remaining == 0 {
            Some(AtBatOutcomeType::Walk)
        } else if self.strikes_remaining == 0 {
            Some(AtBatOutcomeType::Out)
        } else {
            None
        }
    }
}

