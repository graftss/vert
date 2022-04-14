use bevy::{core::FixedTimestep, prelude::*};

use crate::{input::input::RESOLVE_INPUT_LABEL, util::despawn_all_with, AppState};

use super::{
    analog_stick::{
        add_analog_stick_teardown_system, analog_stick_display_system, AnalogStickDisplayMarker,
    },
    button::{add_button_teardown_system, button_update_system, ButtonDisplayMarker},
};

const DISPLAY_TIME_STEP: f32 = 1.0 / 60.0;

pub fn add_display_systems(app: &mut App, display_state: AppState) {
    app.add_system_set(
        SystemSet::on_update(display_state)
            .with_system(button_update_system.after(RESOLVE_INPUT_LABEL))
            .with_system(analog_stick_display_system.after(RESOLVE_INPUT_LABEL)),
    );

    add_button_teardown_system(app, display_state);
    add_analog_stick_teardown_system(app, display_state);
}
