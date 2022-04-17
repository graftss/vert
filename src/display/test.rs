use crate::{
    controller::layout::{ControllerKey, Ps2Key},
    input::input::*,
};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::{
    analog_stick::AnalogStickParams,
    button::ButtonParams,
    display::{AtomicDisplay, InputDisplay, QueuedInputDisplay, Renderable, TaggedAtomicParams},
    frame::FrameParams,
    serialization::{CircleDef, RectangleDef, RegularPolygonDef, RegularPolygonFeatureDef},
};

pub fn debug_frame_data() -> Vec<TaggedAtomicParams> {
    const left: f32 = -120.0;
    const bottom: f32 = -70.0;

    let frame_params = FrameParams {
        left: left - 10.0,
        bottom: bottom + 10.0,
        height: bottom * -2.0,
        width: left * -2.0,
        thickness: 3.0,
    };

    vec![TaggedAtomicParams::Frame(frame_params)]
}

// Add some analog stick components for testing
pub fn debug_analog_stick_data() -> Vec<TaggedAtomicParams> {
    let transform = Transform::from_xyz(-40.0, 0.0, 500.0);

    let stick_shape = CircleDef {
        radius: 9.0,
        ..CircleDef::default()
    };

    let stick_mode = DrawMode::Outlined {
        fill_mode: FillMode::color(Color::BLACK),
        outline_mode: StrokeMode::new(Color::BLACK, 1.0),
    };

    let bg_shape = CircleDef {
        radius: 30.0,
        ..CircleDef::default()
    };

    let bg_mode = DrawMode::Outlined {
        fill_mode: FillMode::color(Color::Rgba {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 0.0,
        }),
        outline_mode: StrokeMode::new(Color::BLACK, 3.0),
    };

    let left_stick = AnalogStickParams {
        stick_display: Renderable::Circle(stick_shape),
        stick_mode: stick_mode.into(),
        bg_display: Renderable::Circle(bg_shape),
        bg_mode: bg_mode.into(),
        transform: transform.into(),
        pos_x: ControllerKey::Ps2(Ps2Key::LeftPosX),
        neg_x: ControllerKey::Ps2(Ps2Key::LeftNegX),
        pos_y: ControllerKey::Ps2(Ps2Key::LeftPosY),
        neg_y: ControllerKey::Ps2(Ps2Key::LeftNegY),
        trigger: Some(ControllerKey::Ps2(Ps2Key::L3)),
        stick_radius: 20.0,
    };

    let right_stick = AnalogStickParams {
        stick_display: Renderable::Circle(stick_shape),
        stick_mode: stick_mode.into(),
        bg_display: Renderable::Circle(bg_shape),
        bg_mode: bg_mode.into(),
        transform: Transform::from_xyz(
            transform.translation.x + 80.0,
            transform.translation.y,
            transform.translation.z,
        )
        .into(),
        pos_x: ControllerKey::Ps2(Ps2Key::RightPosX),
        neg_x: ControllerKey::Ps2(Ps2Key::RightNegX),
        pos_y: ControllerKey::Ps2(Ps2Key::RightPosY),
        neg_y: ControllerKey::Ps2(Ps2Key::RightNegY),
        trigger: Some(ControllerKey::Ps2(Ps2Key::R3)),
        stick_radius: 20.0,
    };

    vec![
        TaggedAtomicParams::AnalogStick(left_stick),
        TaggedAtomicParams::AnalogStick(right_stick),
    ]
}

pub fn debug_button_data() -> Vec<TaggedAtomicParams> {
    let shape = RegularPolygonDef {
        sides: 6,
        feature: RegularPolygonFeatureDef::Radius(200.0),
        ..RegularPolygonDef::default()
    };

    let on_mode = DrawMode::Outlined {
        fill_mode: FillMode::color(Color::CYAN),
        outline_mode: StrokeMode::new(Color::BLACK, 10.0),
    };

    let off_mode = DrawMode::Outlined {
        fill_mode: FillMode::color(Color::RED),
        outline_mode: StrokeMode::new(Color::GREEN, 6.0),
    };

    let mut result = vec![];

    for x in (std::ops::Range { start: 10, end: 15 }) {
        let z = (x * 30) as f32;
        let button_key = ControllerKey::Ps2(Ps2Key::Circle);
        result.push(TaggedAtomicParams::Button(ButtonParams {
            on_mode: on_mode.into(),
            off_mode: off_mode.into(),
            displayable: Renderable::RegularPolygon(shape),
            transform: Transform::from_xyz(z, z, 0.0).into(),
            button_key,
        }));
    }

    result
}

pub fn inject_debug_display(mut commands: Commands) {
    let mut atom_params = vec![];
    atom_params.append(&mut debug_analog_stick_data());
    atom_params.append(&mut debug_button_data());
    atom_params.append(&mut debug_frame_data());

    let atoms = atom_params
        .iter()
        .map(|params| AtomicDisplay {
            params: params.clone(),
            entity: None,
        })
        .collect();

    let display = InputDisplay { atoms };

    commands.insert_resource(QueuedInputDisplay { display });
}

pub fn reinject_debug_display(mut commands: Commands, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::F8) {
        println!("injecting display");
        inject_debug_display(commands);
    }
}
