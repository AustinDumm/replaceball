use replaceball_sim::prelude::*;


pub fn at_bat_start(record: &AtBatRecord) -> String {
    format!("Now Batting: #{}", record.batter_index + 1)
}

pub fn pitch(record: &PitchRecord) -> String {
    let call_type = match record.outcome {
        PitchOutcome::Strike(true) => "Swinging Strike",
        PitchOutcome::Strike(false) => "Called strike",
        PitchOutcome::Ball => "Ball",
        PitchOutcome::Foul => "Fouled off",

        PitchOutcome::Hit(_) => "Swung on",
    };

    let location = match (&record.location.height, &record.location.width) {
        (PitchHeight::High, PitchWidth::Left) => "high and outside",
        (PitchHeight::High, PitchWidth::Center) => "high",
        (PitchHeight::High, PitchWidth::Right) => "high and inside",
        (PitchHeight::Middle, PitchWidth::Left) => "outside",
        (PitchHeight::Middle, PitchWidth::Center) => if record.outcome == PitchOutcome::Ball { "high and away" } else { "down the middle" },
        (PitchHeight::Middle, PitchWidth::Right) => "inside",
        (PitchHeight::Low, PitchWidth::Left) => "low and away",
        (PitchHeight::Low, PitchWidth::Center) => "low",
        (PitchHeight::Low, PitchWidth::Right) => "low and inside",
    };

    format!(
        "{}, {}",
        call_type,
        location,
    )
}

pub fn at_bat_outcome(outcome: &AtBatOutcomeType) -> String {
    match outcome {
        AtBatOutcomeType::Hit(record) => hit_record(record),
        AtBatOutcomeType::Walk => format!("Walk"),
        AtBatOutcomeType::Out => format!("Strikeout"),
    }
}

fn hit_record(record: &HitRecord) -> String {
    match &record.outcome {
        HitOutcome::HomeRun => 
            format!("Fly ball into {}", field_direction(&record.direction, false)),
        HitOutcome::InPlay(field_record) => landing(
            &record.direction,
            &record.launch_angle,
            &field_record.landing,
        ),
    }
}

pub fn play_result(record: &FieldingRecord) -> String {
    let landing_description = landing_description(&record.landing);
    let baserunning_description = baserunning(&record.base_running_record);

    format!("{}. {}", landing_description, baserunning_description)
}

fn landing_description(landing: &BallLanding) -> String {
    match landing {
        BallLanding::Out(fielder, _) =>
            format!("Caught by the {} for an out", fielder),
        BallLanding::Landed(_, play) => 
            landed_play(play),
    }
}

fn landed_play(fielding: &FieldingPlay) -> String {
    format!(
        "{} over to the {}",
        fielding.from,
        fielding.to,
    )
}

fn baserunning(record: &BaseRunningRecord) -> String {
    let hit_type = match record.outcome.batter_hit_type {
        HitType::Out => format!("Batter is out"),
        HitType::FieldersChoice => format!("Fielders choice"),
        HitType::Single => format!("Single"),
        HitType::Double => format!("Double"),
        HitType::Triple => format!("Triple"),
        HitType::HomeRun => format!("Inside the Park Home Run"),
    };

    format!(
        "{}. {} run(s) score. {} out(s) are made.",
        hit_type,
        record.outcome.runs_scored,
        record.outcome.outs_made,
    )
}

fn field_direction(direction: &HitDirection, is_infield: bool) -> String {
    if is_infield {
        let direction_index = (direction.0 / 18.0) as u8;
        match direction_index {
            0 => format!("third base"),
            1 => format!("shortstop"),
            2 => format!("up the middle"),
            3 => format!("second"),
            _ => format!("first base"),
        }
    } else {
        let direction_index = (direction.0 / 12.85) as u8;
        match direction_index {
            0 => format!("left field corner"),
            1 => format!("left field"),
            2 => format!("left center"),
            3 => format!("center field"),
            4 => format!("right center"),
            5 => format!("right field"),
            _ => format!("right field corner"),
        }
    }
}

fn landing(
    direction: &HitDirection,
    launch: &LaunchAngle,
    landing: &BallLanding
) -> String {
    let landing_location = match landing {
        BallLanding::Out(_, location) => location,
        BallLanding::Landed(location, _) => location,
    };

    let direction = field_direction(
        direction,
        landing_location.distance.0 < 120.0,
    );
    let angle = hit_angle(launch);

    format!("{} to {}", angle, direction)
}

fn hit_angle(launch: &LaunchAngle) -> String {
    if launch.0 < -10.0 {
        format!("Chopper")
    } else if launch.0 < 10.0 {
        format!("Ground ball")
    } else if launch.0 < 25.0 {
        format!("Line drive")
    } else if launch.0 < 40.0 {
        format!("Fly ball")
    } else {
        format!("Pop up")
    }
}

