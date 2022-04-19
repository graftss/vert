use crate::{
    controller::layout::{ControllerKey, Ps2Key},
    input::input::*,
};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::{
    analog_stick::AnalogStickParams,
    button::ButtonParams,
    display::{AtomicDisplay, InputDisplay, TaggedAtomicParams},
    frame::FrameParams,
    renderable::Renderable,
    serialization::{CircleDef, RectangleDef, RegularPolygonDef, RegularPolygonFeatureDef},
    system::{RequestDespawnAll, RequestSaveDisplay, RequestSpawnAtom},
};

pub fn debug_frame_data() -> Vec<Box<TaggedAtomicParams>> {
    const left: f32 = -120.0;
    const bottom: f32 = -70.0;

    let frame_params = FrameParams {
        position: Vec2::new(left - 10.0, bottom + 10.0),
        height: bottom * -2.0,
        width: left * -2.0,
        thickness: 3.0,
        color: Color::Rgba {
            red: 0.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        },
    };

    vec![Box::new(TaggedAtomicParams::Frame(frame_params))]
}

// Add some analog stick components for testing
pub fn debug_analog_stick_data() -> Vec<Box<TaggedAtomicParams>> {
    let transform = Transform::from_xyz(-40.0, 0.0, 500.0);

    let stick_shape = CircleDef {
        radius: 9.0,
        ..CircleDef::default()
    };

    let stick_mode = DrawMode::Outlined {
        fill_mode: FillMode::color(Color::BLACK),
        outline_mode: StrokeMode::new(Color::BLACK, 1.0),
    };

    let trigger_mode = DrawMode::Outlined {
        fill_mode: FillMode::color(Color::RED),
        outline_mode: StrokeMode::new(Color::BLACK, 1.0),
    }
    .into();

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
        trigger_mode,
        bg_display: Renderable::Circle(bg_shape),
        bg_mode: bg_mode.into(),
        transform: transform.into(),
        pos_x: ControllerKey::Ps2(Ps2Key::LeftPosX).into(),
        neg_x: ControllerKey::Ps2(Ps2Key::LeftNegX).into(),
        pos_y: ControllerKey::Ps2(Ps2Key::LeftPosY).into(),
        neg_y: ControllerKey::Ps2(Ps2Key::LeftNegY).into(),
        trigger: ControllerKey::Ps2(Ps2Key::L3).into(),
        stick_radius: 20.0,
    };

    let right_stick = AnalogStickParams {
        stick_display: Renderable::Circle(stick_shape),
        stick_mode: stick_mode.into(),
        trigger_mode,
        bg_display: Renderable::Circle(bg_shape),
        bg_mode: bg_mode.into(),
        transform: Transform::from_xyz(
            transform.translation.x + 80.0,
            transform.translation.y,
            transform.translation.z,
        )
        .into(),
        pos_x: ControllerKey::Ps2(Ps2Key::RightPosX).into(),
        neg_x: ControllerKey::Ps2(Ps2Key::RightNegX).into(),
        pos_y: ControllerKey::Ps2(Ps2Key::RightPosY).into(),
        neg_y: ControllerKey::Ps2(Ps2Key::RightNegY).into(),
        trigger: ControllerKey::Ps2(Ps2Key::R3).into(),
        stick_radius: 20.0,
    };

    vec![
        Box::new(TaggedAtomicParams::AnalogStick(left_stick)),
        Box::new(TaggedAtomicParams::AnalogStick(right_stick)),
    ]
}

pub fn debug_button_data() -> Vec<Box<TaggedAtomicParams>> {
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

    for x in (std::ops::Range { start: 10, end: 11 }) {
        let z = (x * 30) as f32;
        let button_key = ControllerKey::Ps2(Ps2Key::Circle);
        result.push(Box::new(TaggedAtomicParams::Button(ButtonParams {
            on_mode: on_mode.into(),
            off_mode: off_mode.into(),
            displayable: Renderable::RegularPolygon(shape),
            transform: Transform::from_xyz(z, z, 0.0).into(),
            button_key: button_key.into(),
        })));
    }

    result
}

pub fn inject_debug_display(
    mut commands: Commands,
    mut event_writer: EventWriter<RequestSpawnAtom>,
) {
    let mut atom_params = vec![];
    atom_params.append(&mut debug_analog_stick_data());
    atom_params.append(&mut debug_button_data());
    atom_params.append(&mut debug_frame_data());

    let mut atoms = vec![];
    for params in atom_params {
        atoms.push(AtomicDisplay {
            params,
            entity: None,
        });
    }

    for atom in atoms {
        event_writer.send(RequestSpawnAtom::Existing(atom));
    }
}

pub fn save_display_hotkey(
    keyboard_input: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<RequestSaveDisplay>,
) {
    if keyboard_input.just_pressed(KeyCode::F6) {
        println!("saving display");
        event_writer.send(RequestSaveDisplay);
    }
}

pub fn clear_display_hotkey(
    keyboard_input: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<RequestDespawnAll>,
) {
    if keyboard_input.just_pressed(KeyCode::F7) {
        println!("clearing display");
        event_writer.send(RequestDespawnAll);
    }
}

pub fn inject_debug_display_hotkey(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut event_writer: EventWriter<RequestSpawnAtom>,
) {
    if keyboard_input.just_pressed(KeyCode::F8) {
        println!("injecting display");

        inject_debug_display(commands, event_writer);
    }
}
