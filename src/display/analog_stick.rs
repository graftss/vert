use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_prototype_lyon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    controller::layout::ControllerKey,
    editor::inspector::BoundControllerKey,
    input::input::{InputSink, InputSource, InputValue},
    state::AppState,
    util::despawn_all_with,
};

use super::{
    display::{AtomicInputDisplay, Renderable, RootAtomicDisplayMarker},
    serialization::{DrawModeDef, TransformDef},
};

// The data parameterizing an analog stick input display.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Component, Inspectable)]
pub struct AnalogStickParams {
    #[inspectable(ignore)]
    pub stick_display: Renderable,
    #[inspectable(ignore)]
    pub stick_mode: DrawModeDef,
    #[inspectable(min = 0.0, suffix = "px")]
    pub stick_radius: f32,
    #[inspectable(ignore)]
    pub bg_display: Renderable,
    #[inspectable(ignore)]
    pub bg_mode: DrawModeDef,
    pub transform: TransformDef,
    pub pos_x: BoundControllerKey,
    pub neg_x: BoundControllerKey,
    pub pos_y: BoundControllerKey,
    pub neg_y: BoundControllerKey,
    pub trigger: BoundControllerKey,
}

impl AnalogStickParams {
    fn root_bundle(self) -> impl Bundle {
        let Self {
            pos_x,
            neg_x,
            pos_y,
            neg_y,
            trigger,
            ..
        } = self;

        // Collect the input sources needed by this display
        let mut sources = vec![pos_x.key, neg_x.key, pos_y.key, neg_y.key, trigger.key];
        let input_sink = InputSink::new(sources);

        (
            GlobalTransform::identity(),
            Into::<Transform>::into(self.transform),
            RootAnalogStickMarker,
            RootAtomicDisplayMarker,
            Name::new("Analog Stick"),
            input_sink,
        )
    }

    fn insert_stick_bundle(self, mut commands: EntityCommands) {
        let Self {
            stick_display,
            stick_mode,
            ..
        } = self;
        commands
            .insert_bundle(stick_display.build_as(stick_mode.into(), Transform::identity()))
            .insert(ChildStickMarker);
    }

    fn insert_bg_bundle(self, mut commands: EntityCommands) {
        let Self {
            bg_display,
            bg_mode,
            ..
        } = self;

        commands
            .insert_bundle(bg_display.build_as(bg_mode.into(), Transform::identity()))
            .insert(ChildBgMarker);
    }
}

// An entity with this marker will have an `InputSink` with a source vector of:
//   - 4 entries, if `use_trigger` is `false`;
//   - 5 entries, if `use_trigger` is `true`.
// These entries will be in the same order as they appear in `AnalogStickDisplayData`.
#[derive(Component)]
pub struct RootAnalogStickMarker;

#[derive(Component)]
pub struct ChildStickMarker;

#[derive(Component)]
pub struct ChildBgMarker;

pub struct AnalogStickAtomicDisplay;

impl AnalogStickAtomicDisplay {
    fn unwrap_axis(axis: Option<InputValue>) -> f32 {
        match axis {
            Some(InputValue::Axis(v)) => v,
            _ => 0.0,
        }
    }

    // Parses an axis magnitude from two (potentially `None`-valued) analog inputs
    // corresponding to the positive and negative directions of that axis.
    fn axis_to_position(pos: Option<InputValue>, neg: Option<InputValue>) -> f32 {
        let pos_value = Self::unwrap_axis(pos);
        let neg_value = Self::unwrap_axis(neg);
        pos_value - neg_value
    }

    // Parses the relative position of the analog stick as a `Vec2` from the analog
    // stick's `InputValue` vector.
    fn axes_to_positions(values: &Vec<Option<InputValue>>) -> Vec2 {
        if let [pos_x, neg_x, pos_y, neg_y] = values[0..4] {
            Vec2::new(
                Self::axis_to_position(pos_x, neg_x),
                Self::axis_to_position(pos_y, neg_y),
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

    fn analog_stick_display_system(
        q_parent: Query<(&InputSink, &Children, &AnalogStickParams)>,
        mut q_child_stick: Query<(&mut Transform, &mut DrawMode), With<ChildStickMarker>>,
    ) {
        for (sink, children, params) in q_parent.iter() {
            for child in children.iter() {
                if let Ok((mut stick_transform, mut draw_mode)) = q_child_stick.get_mut(*child) {
                    // Move the stick child according to the axis input
                    let pos = Self::axes_to_positions(&sink.values);
                    stick_transform.translation.x = pos.x * params.stick_radius;
                    stick_transform.translation.y = pos.y * params.stick_radius;

                    // Handle trigger presses
                    if params.trigger.key.is_some() {
                        if Self::is_trigger_pressed(&sink.values) {
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

    fn regenerate_system(
        mut commands: Commands,
        parent_query: Query<(Entity, &AnalogStickParams, &Children), Changed<AnalogStickParams>>,
        child_stick_query: Query<Entity, With<ChildStickMarker>>,
        child_bg_query: Query<Entity, With<ChildBgMarker>>,
    ) {
        for (root_entity, params, children) in parent_query.iter() {
            // Regenerate the root entity
            commands
                .entity(root_entity)
                .insert_bundle(params.root_bundle());

            // Rengenerate the child entities
            for &child_entity in children.iter() {
                if let Ok(stick_entity) = child_stick_query.get(child_entity) {
                    params.insert_stick_bundle(commands.entity(stick_entity));
                } else if let Ok(bg_entity) = child_bg_query.get(child_entity) {
                    params.insert_bg_bundle(commands.entity(bg_entity));
                }
            }
        }
    }
}

impl AtomicInputDisplay<AnalogStickParams> for AnalogStickAtomicDisplay {
    fn spawn(commands: &mut Commands, params: &AnalogStickParams) -> Entity {
        let mut root = commands.spawn_bundle(params.root_bundle());
        let root_entity = root.id();

        let mut my_params = *params;
        my_params.pos_x.bind(root_entity, 0);
        my_params.neg_x.bind(root_entity, 1);
        my_params.pos_y.bind(root_entity, 2);
        my_params.neg_y.bind(root_entity, 3);
        my_params.trigger.bind(root_entity, 4);

        root.insert(my_params)
            .with_children(|parent| {
                params.insert_stick_bundle(parent.spawn());
                params.insert_bg_bundle(parent.spawn());
            })
            .id()
    }

    fn add_update_systems(app: &mut App) {
        app.add_system(Self::analog_stick_display_system);
        app.add_system(Self::regenerate_system);
        app.register_inspectable::<AnalogStickParams>();
    }
}
