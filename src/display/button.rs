use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    app_state::AppState,
    input::input::{InputSink, InputSource, InputValue},
    util::despawn_all_with,
};

use super::display::Renderable;

// The data parameterizing a button input display.
#[derive(Clone, Copy)]
pub struct ButtonDisplayData {
    pub displayable: Renderable,
    pub on_mode: DrawMode,
    pub off_mode: DrawMode,
    pub transform: Transform,
    pub input_source: InputSource,
}

#[derive(Component)]
pub struct ButtonDisplayMarker {
    pub pressed: bool,
}

pub fn spawn_button(commands: &mut Commands, display_data: &ButtonDisplayData) {
    let ButtonDisplayData {
        displayable,
        on_mode,
        off_mode,
        transform,
        input_source,
    } = *display_data;

    let mut on_bundle = displayable.build_as(on_mode, transform);
    on_bundle.visibility = Visibility { is_visible: false };

    let off_bundle = displayable.build_as(off_mode, transform);

    commands
        .spawn_bundle(on_bundle)
        .insert(ButtonDisplayMarker { pressed: true })
        .insert(InputSink::new(vec![input_source]));

    commands
        .spawn_bundle(off_bundle)
        .insert(ButtonDisplayMarker { pressed: false })
        .insert(InputSink::new(vec![input_source]));
}

pub fn add_button_teardown_system(app: &mut App, display_state: AppState) {
    app.add_system_set(
        SystemSet::on_exit(display_state).with_system(despawn_all_with::<ButtonDisplayMarker>),
    );
}

pub fn button_update_system(mut query: Query<(&InputSink, &ButtonDisplayMarker, &mut Visibility)>) {
    for (sink, marker, mut vis) in query.iter_mut() {
        match sink.values[0] {
            Some(InputValue::Button(true)) => {
                vis.is_visible = marker.pressed;
            }
            Some(InputValue::Button(false)) => {
                vis.is_visible = !marker.pressed;
            }
            _ => {
                vis.is_visible = false;
            }
        }
    }
}
