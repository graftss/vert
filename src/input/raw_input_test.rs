use bevy::prelude::*;

use super::input::{AxisSign, HidAxisId, InputSink, InputSource, InputValue};

#[derive(Component)]
pub struct AnalogText;

#[derive(Component)]
pub struct ButtonsText;

fn make_text_section(s: String, font: Handle<Font>) -> TextSection {
    TextSection {
        value: s.to_string(),
        style: TextStyle {
            font,
            font_size: 60.0,
            color: Color::RED,
        },
    }
}

pub fn raw_input_test_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // setup ui
    commands.spawn_bundle(UiCameraBundle::default());

    // sink for left stick
    let pos_x = InputSource::HidAxis(0, HidAxisId::X, AxisSign::Plus);
    let neg_x = InputSource::HidAxis(0, HidAxisId::X, AxisSign::Minus);
    let pos_y = InputSource::HidAxis(0, HidAxisId::Y, AxisSign::Plus);
    let neg_y = InputSource::HidAxis(0, HidAxisId::Y, AxisSign::Minus);
    let analog_sink = InputSink::new(vec![pos_x, neg_x, pos_y, neg_y]);

    // sink for buttons
    let mut button_sources = vec![];
    for i in 0..12 {
        button_sources.push(InputSource::HidButton(0, i));
    }
    let buttons_sink = InputSink::new(button_sources);

    // make analog text bundle
    let analog_text_bundle = TextBundle {
        style: Style {
            align_self: AlignSelf::Center,
            ..Style::default()
        },
        text: Text {
            sections: vec![
                make_text_section(
                    "Left stick: ".to_string(),
                    asset_server.load("fonts/FiraSans-Bold.ttf"),
                ),
                make_text_section("".to_string(), asset_server.load("fonts/FiraSans-Bold.ttf")),
            ],
            alignment: TextAlignment::default(),
        },
        ..TextBundle::default()
    };

    // spawn analog text bundle
    commands
        .spawn_bundle(analog_text_bundle)
        .insert(analog_sink)
        .insert(AnalogText);

    // construct buttons text bundle
    let buttons_text_bundle = TextBundle {
        style: Style {
            align_self: AlignSelf::Center,
            ..Style::default()
        },
        text: Text {
            sections: vec![
                make_text_section(
                    "Buttons: ".to_string(),
                    asset_server.load("fonts/FiraSans-Bold.ttf"),
                ),
                make_text_section("".to_string(), asset_server.load("fonts/FiraSans-Bold.ttf")),
            ],
            alignment: TextAlignment::default(),
        },
        ..TextBundle::default()
    };

    // spawn analog text bundle
    commands
        .spawn_bundle(buttons_text_bundle)
        .insert(buttons_sink)
        .insert(ButtonsText);
}

fn ext(input: Option<InputValue>) -> f32 {
    match input {
        Some(InputValue::Axis(v)) => v,
        _ => 0.0,
    }
}

pub fn analog_test_system(mut query: Query<(&mut Text, &InputSink), With<AnalogText>>) {
    for (mut text, sink) in query.iter_mut() {
        if let [a, b, c, d] = sink.values[0..4] {
            text.sections[1].value =
                format!("{:.3} {:.3} {:.3} {:.3}", ext(a), ext(b), ext(c), ext(d));
        }
    }
}

pub fn buttons_test_system(mut query: Query<(&mut Text, &InputSink), With<ButtonsText>>) {
    for (mut text, sink) in query.iter_mut() {
        // Find the pressed buttons by id
        let mut all_pressed = Vec::new();
        for (button_id, pressed) in sink.values.iter().enumerate() {
            if let Some(InputValue::Button(true)) = pressed {
                all_pressed.push(button_id.to_string());
            }
        }

        // Print them to the button text
        text.sections[1].value = format!("{}", all_pressed.join(", "));
    }
}
