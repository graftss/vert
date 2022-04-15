use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    controller::layout::ControllerKey,
    input::input::{InputSink, InputValue},
    util::despawn_all_with,
};

use super::{
    display::{AtomicInputDisplay, Renderable},
    serialization::{DrawModeDef, TransformDef},
};

// The data parameterizing a button input display.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct ButtonParams {
    pub displayable: Renderable,
    pub on_mode: DrawModeDef,
    pub off_mode: DrawModeDef,
    pub transform: TransformDef,
    pub button_key: ControllerKey,
}

// The marker for the root entity of a button display.
#[derive(Component)]
pub struct RootButtonMarker;

impl RootButtonMarker {
    pub fn build_root(transform: Transform) -> impl Bundle {
        (GlobalTransform::identity(), transform, RootButtonMarker)
    }
}

// The marker for a child entity of a button display.
// The root display has two children whose visibilities are toggled
// according to the button input.
#[derive(Component)]
pub struct ChildButtonMarker {
    // The button press state for which this child entity is visible.
    pub pressed: bool,
}

pub struct ButtonAtomicDisplay;

impl ButtonAtomicDisplay {
    // Update all atomic button displays.
    fn button_update_system(mut query: Query<(&InputSink, &ChildButtonMarker, &mut Visibility)>) {
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
}

impl AtomicInputDisplay<ButtonParams> for ButtonAtomicDisplay {
    fn spawn(commands: &mut Commands, display_data: &ButtonParams) {
        let ButtonParams {
            displayable,
            on_mode,
            off_mode,
            transform,
            button_key,
        } = *display_data;

        let mut on_bundle = displayable.build_as(on_mode.into(), Transform::identity());
        on_bundle.visibility = Visibility { is_visible: false };

        let off_bundle = displayable.build_as(off_mode.into(), Transform::identity());

        commands
            .spawn_bundle(RootButtonMarker::build_root(transform.into()))
            .with_children(|parent| {
                parent
                    .spawn_bundle(on_bundle)
                    .insert(ChildButtonMarker { pressed: true })
                    .insert(InputSink::new(vec![button_key]));

                parent
                    .spawn_bundle(off_bundle)
                    .insert(ChildButtonMarker { pressed: false })
                    .insert(InputSink::new(vec![button_key]));
            });
    }

    fn add_teardown_systems(app: &mut App, display_state: AppState) {
        app.add_system_set(
            SystemSet::on_exit(display_state)
                .with_system(despawn_all_with::<RootButtonMarker>)
                .with_system(despawn_all_with::<ChildButtonMarker>),
        );
    }

    fn add_update_systems(app: &mut App, display_state: AppState) {
        app.add_system_set(
            SystemSet::on_update(display_state).with_system(Self::button_update_system),
        );
    }
}
