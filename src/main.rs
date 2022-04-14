use bevy::{
    prelude::*,
};

use bevy_prototype_lyon::prelude::*;
use display::system::add_display_systems;

use input::{input::add_input_systems};

mod display;
mod input;

fn root_startup_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn add_debug_tools(app: &mut App) {
    // app.add_plugin(WorldInspectorPlugin::new());

    // add console-based FPS logging
    // app.add_plugin(LogDiagnosticsPlugin::default());
    // app.add_plugin(FrameTimeDiagnosticsPlugin::default());

    // Add some fixed button displays for testing
    // app.add_startup_system(display::button::test_button_startup_system);

    // Add some fixed analog stick displays for testing
    app.add_startup_system(display::analog_stick::test_analog_stick_startup_system);
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugin(ShapePlugin);

    #[cfg(debug_assertions)]
    add_debug_tools(&mut app);

    app.add_startup_system(root_startup_system);
    add_input_systems(&mut app);
    add_display_systems(&mut app);

    app.insert_resource(WindowDescriptor {
        title: "vert".to_string(),
        width: 800.,
        height: 600.,
        ..WindowDescriptor::default()
    });

    app.run();
}
