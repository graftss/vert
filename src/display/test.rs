use crate::input::input::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::{
    analog_stick::AnalogStickDisplayData,
    button::ButtonDisplayData,
    display::{AtomicDisplay, Display, Renderable},
};

// Add some analog stick components for testing
pub fn debug_analog_stick_data() -> Vec<AtomicDisplay> {
    let transform = Transform::from_xyz(-40.0, 0.0, 500.0);

    let stick_shape = shapes::Circle {
        radius: 9.0,
        ..shapes::Circle::default()
    };

    let stick_mode = DrawMode::Outlined {
        fill_mode: FillMode::color(Color::BLACK),
        outline_mode: StrokeMode::new(Color::BLACK, 1.0),
    };

    let bg_shape = shapes::Circle {
        radius: 30.0,
        ..shapes::Circle::default()
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

    let left_stick = AnalogStickDisplayData {
        stick_display: Renderable::Circle(stick_shape),
        stick_mode,
        bg_display: Renderable::Circle(bg_shape),
        bg_mode,
        transform,
        pos_x: InputSource::HidAxis(0, HidAxisId::X, AxisSign::Plus),
        neg_x: InputSource::HidAxis(0, HidAxisId::X, AxisSign::Minus),
        pos_y: InputSource::HidAxis(0, HidAxisId::Y, AxisSign::Plus),
        neg_y: InputSource::HidAxis(0, HidAxisId::Y, AxisSign::Minus),
        trigger: Some(InputSource::HidButton(0, 10)),
        stick_radius: 20.0,
    };

    let right_stick = AnalogStickDisplayData {
        stick_display: Renderable::Circle(stick_shape),
        stick_mode,
        bg_display: Renderable::Circle(bg_shape),
        bg_mode,
        transform: Transform::from_xyz(
            transform.translation.x + 80.0,
            transform.translation.y,
            transform.translation.z,
        ),
        pos_x: InputSource::HidAxis(0, HidAxisId::RZ, AxisSign::Plus),
        neg_x: InputSource::HidAxis(0, HidAxisId::RZ, AxisSign::Minus),
        pos_y: InputSource::HidAxis(0, HidAxisId::Z, AxisSign::Plus),
        neg_y: InputSource::HidAxis(0, HidAxisId::Z, AxisSign::Minus),
        trigger: Some(InputSource::HidButton(0, 11)),
        stick_radius: 20.0,
    };

    vec![
        AtomicDisplay::AnalogStick(left_stick),
        AtomicDisplay::AnalogStick(right_stick),
    ]
}

pub fn debug_button_data() -> Vec<AtomicDisplay> {
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
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
        let z = (x * 5) as f32;
        let input_source = InputSource::Key(KeyCode::W);
        result.push(AtomicDisplay::Button(ButtonDisplayData {
            on_mode,
            off_mode,
            displayable: Renderable::RegularPolygon(shape),
            transform: Transform::from_xyz(z, z, 0.0),
            input_source,
        }));
    }

    result
}

pub fn inject_debug_display(mut commands: Commands) {
    let mut atoms = debug_analog_stick_data();
    atoms.append(&mut debug_button_data());

    commands.insert_resource(Display { atoms });
}
