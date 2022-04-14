use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    app_state::AppState,
    input::input::{InputSink, InputSource, InputValue},
    util::despawn_all_with,
};

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

pub fn test_button_startup_system(mut commands: Commands) {
    println!("startup button ");
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

    for x in (std::ops::Range { start: 10, end: 15 }) {
        let z = (x * 5) as f32;
        let input_source = InputSource::Key(KeyCode::W);
        add_button_display(
            &mut commands,
            ButtonDisplayData {
                on_mode,
                off_mode,
                displayable: Displayable::RegularPolygon(shape),
                transform: Transform::from_xyz(z, z, 0.0),
                input_source,
            },
        );
    }

    // for x in (std::ops::Range { start: -40, end: 0 }) {
    //     let z = (x * 10) as f32;
    //     let input_source = InputSource::Key(KeyCode::W);
    //     add_button_display(
    //         &mut commands,
    //         ButtonDisplayData {
    //             on_mode,
    //             off_mode,
    //             displayable: Displayable::RegularPolygon(shape),
    //             transform: Transform::from_xyz(z, z, 0.0),
    //             input_source,
    //         },
    //     );
    // }
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
