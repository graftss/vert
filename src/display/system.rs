use bevy::{core::FixedTimestep, prelude::*};

use crate::input::input::RESOLVE_INPUT_LABEL;

use super::{analog_stick::analog_stick_display_system, button::button_display_system};

const DISPLAY_TIME_STEP: f32 = 1.0 / 60.0;

pub fn add_display_systems(app: &mut App) {
    app.add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(DISPLAY_TIME_STEP as f64))
            .with_system(button_display_system.after(RESOLVE_INPUT_LABEL))
            .with_system(analog_stick_display_system.after(RESOLVE_INPUT_LABEL)),
    );
}
