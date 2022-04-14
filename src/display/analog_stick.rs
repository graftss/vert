use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    app_state::AppState,
    input::input::{AxisSign, HidAxisId, InputSink, InputSource, InputValue},
    util::despawn_all_with,
};

use super::display::Renderable;

// The data parameterizing an analog stick input display.
pub struct AnalogStickDisplayData {
    pub stick_display: Renderable,
    pub stick_mode: DrawMode,
    pub stick_radius: f32,
    pub bg_display: Renderable,
    pub bg_mode: DrawMode,
    pub transform: Transform,
    pub pos_x: InputSource,
    pub neg_x: InputSource,
    pub pos_y: InputSource,
    pub neg_y: InputSource,
    pub trigger: Option<InputSource>,
}

// An entity with this marker will have an `InputSink` with a source vector of:
//   - 4 entries, if `use_trigger` is `false`;
//   - 5 entries, if `use_trigger` is `true`.
// These entries will be in the same order as they appear in `AnalogStickDisplayData`.
#[derive(Component)]
pub struct AnalogStickDisplayMarker {
    pub stick_radius: f32,
    pub use_trigger: bool,
}

#[derive(Component)]
pub struct ChildStickMarker;

#[derive(Component)]
pub struct ChildBgMarker;

pub fn spawn_analog_stick(commands: &mut Commands, display_data: &AnalogStickDisplayData) {
    let AnalogStickDisplayData {
        stick_display,
        stick_mode,
        bg_display,
        bg_mode,
        transform,
        pos_x,
        neg_x,
        pos_y,
        neg_y,
        stick_radius,
        trigger,
    } = *display_data;

    let stick_bundle = stick_display.build_as(stick_mode, Transform::from_xyz(30.0, 0.0, 0.0));
    let bg_bundle = bg_display.build_as(bg_mode, Transform::identity());

    let mut sources = vec![pos_x, neg_x, pos_y, neg_y];
    let mut use_trigger = false;
    if let Some(source) = trigger {
        sources.push(source);
        use_trigger = true;
    }
    let input_sink = InputSink::new(sources);

    commands
        .spawn()
        .insert(transform)
        .insert(GlobalTransform::identity())
        .insert(AnalogStickDisplayMarker {
            stick_radius,
            use_trigger,
        })
        .insert(input_sink)
        .with_children(|parent| {
            parent.spawn_bundle(stick_bundle).insert(ChildStickMarker);

            parent.spawn_bundle(bg_bundle).insert(ChildBgMarker);
        });
}

fn unwrap_axis(axis: Option<InputValue>) -> f32 {
    match axis {
        Some(InputValue::Axis(v)) => v,
        _ => 0.0,
    }
}

// Parses an axis magnitude from two (potentially `None`-valued) analog inputs
// corresponding to the positive and negative directions of that axis.
fn axis_to_position(pos: Option<InputValue>, neg: Option<InputValue>) -> f32 {
    let pos_value = unwrap_axis(pos);
    let neg_value = unwrap_axis(neg);
    pos_value - neg_value
}

// Parses the relative position of the analog stick as a `Vec2` from the analog
// stick's `InputValue` vector.
fn axes_to_positions(values: &Vec<Option<InputValue>>) -> Vec2 {
    if let [pos_x, neg_x, pos_y, neg_y] = values[0..4] {
        Vec2::new(
            axis_to_position(pos_x, neg_x),
            axis_to_position(pos_y, neg_y),
        )
    } else {
        Vec2::new(0.0, 0.0)
    }
}

// Returns `true` if the analog stick's `InputValue` vector indicates that the
// trigger button is being pressed.
fn is_trigger_pressed(values: &Vec<Option<InputValue>>) -> bool {
    match values.get(4) {
        Some(&Some(InputValue::Button(v))) => v,
        _ => false,
    }
}

pub fn add_analog_stick_teardown_system(app: &mut App, display_state: AppState) {
    app.add_system_set(
        SystemSet::on_exit(display_state)
            .with_system(despawn_all_with::<AnalogStickDisplayMarker>)
            .with_system(despawn_all_with::<ChildStickMarker>)
            .with_system(despawn_all_with::<ChildBgMarker>),
    );
}

pub fn analog_stick_display_system(
    q_parent: Query<(&InputSink, &Children, &AnalogStickDisplayMarker)>,
    mut q_child_stick: Query<(&mut Transform, &mut DrawMode), With<ChildStickMarker>>,
) {
    for (sink, children, asdm) in q_parent.iter() {
        for child in children.iter() {
            if let Ok((mut stick_transform, mut draw_mode)) = q_child_stick.get_mut(*child) {
                // Move the stick child according to the axis input
                let pos = axes_to_positions(&sink.values);
                stick_transform.translation.x = pos.x * asdm.stick_radius;
                stick_transform.translation.y = pos.y * asdm.stick_radius;

                // Handle trigger presses
                if asdm.use_trigger {
                    if is_trigger_pressed(&sink.values) {
                        if let DrawMode::Outlined {
                            ref mut fill_mode, ..
                        } = *draw_mode
                        {
                            fill_mode.color = Color::RED;
                        }
                    } else {
                        if let DrawMode::Outlined {
                            ref mut fill_mode, ..
                        } = *draw_mode
                        {
                            fill_mode.color = Color::BLACK;
                        }
                    }
                }
            }
        }
    }
}
