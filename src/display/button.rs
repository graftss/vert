use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::input::input::{InputSink, InputSource, InputValue};

use super::display::Displayable;

#[derive(Component)]
pub struct ButtonDisplayMarker {
    pub pressed: bool,
}

pub struct ButtonDisplayData {
    displayable: Displayable,
    on_mode: DrawMode,
    off_mode: DrawMode,
    transform: Transform,
    input_source: InputSource,
}

pub fn add_button_display(commands: &mut Commands, display_data: ButtonDisplayData) {
    let ButtonDisplayData {
        displayable,
        on_mode,
        off_mode,
        transform,
        input_source,
    } = display_data;
    let shape = displayable.to_geometry();

    let mut on_bundle = GeometryBuilder::build_as(shape, on_mode, transform);
    on_bundle.visibility = Visibility { is_visible: false };

    let off_bundle = GeometryBuilder::build_as(shape, off_mode, transform);

    commands
        .spawn_bundle(on_bundle)
        .insert(ButtonDisplayMarker { pressed: true })
        .insert(InputSink::new(input_source));

    commands
        .spawn_bundle(off_bundle)
        .insert(ButtonDisplayMarker { pressed: false })
        .insert(InputSink::new(input_source));
}

pub fn test_button_startup_system(mut commands: Commands) {
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

    let w_button = ButtonDisplayData {
        on_mode,
        off_mode,
        displayable: Displayable::RegularPolygon(shape),
        transform: Transform::from_xyz(100.0, 100.0, 100.0),
        input_source: InputSource::Key(KeyCode::W),
    };

    let d_button = ButtonDisplayData {
        on_mode,
        off_mode,
        displayable: Displayable::RegularPolygon(shape),
        transform: Transform::from_xyz(200.0, 200.0, 200.0),
        input_source: InputSource::Key(KeyCode::D),
    };

    add_button_display(&mut commands, w_button);
    add_button_display(&mut commands, d_button);
}

pub fn button_display_system(
    mut query: Query<(&InputSink, &ButtonDisplayMarker, &mut Visibility)>,
) {
    for (sink, marker, mut vis) in query.iter_mut() {
        match sink.value {
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
