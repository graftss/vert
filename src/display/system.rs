use bevy::{core::FixedTimestep, prelude::*};

use crate::{input::input::RESOLVE_INPUT_LABEL, AppState};

use super::{analog_stick::analog_stick_display_system, button::button_display_system};

const DISPLAY_TIME_STEP: f32 = 1.0 / 60.0;

pub fn add_display_systems(app: &mut App, display_state: AppState) {
    app.add_system_set(
        SystemSet::on_update(display_state)
            .with_system(button_display_system.after(RESOLVE_INPUT_LABEL))
            .with_system(analog_stick_display_system.after(RESOLVE_INPUT_LABEL)),
    );
}
