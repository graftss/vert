use app_state::{state_hotkey_system, AppState};
use bevy::prelude::*;

use bevy_prototype_lyon::prelude::*;
use display::system::add_display_systems;

use input::input::add_input_systems;

mod app_state;
mod display;
mod input;

fn main() {
    let mut app = App::new();

    // Add plugins.
    app.add_plugins(DefaultPlugins);
    app.add_plugin(ShapePlugin);

    // Set initial value of `AppState`.
    app.add_state(AppState::ConfigureController);

    #[cfg(debug_assertions)]
    add_debug_tools(&mut app);

    app.add_startup_system(root_startup_system);
    app.add_system(state_hotkey_system);
    add_input_systems(&mut app);
    add_display_systems(&mut app, AppState::Display);

    app.insert_resource(WindowDescriptor {
        title: "vert".to_string(),
        width: 800.,
        height: 600.,
        ..WindowDescriptor::default()
    });

    app.run();
}

fn root_startup_system(mut commands: Commands, mut app_state: ResMut<State<AppState>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn add_debug_tools(app: &mut App) {
    // app.add_plugin(WorldInspectorPlugin::new());

    // add console-based FPS logging
    // app.add_plugin(LogDiagnosticsPlugin::default());
    // app.add_plugin(FrameTimeDiagnosticsPlugin::default());

    // Add some fixed button displays for testing
    // app.add_startup_system(display::button::test_button_startup_system);

    // Add some fixed analog stick displays for testing
    app.add_system_set(
        SystemSet::on_enter(AppState::Display)
            .with_system(display::analog_stick::test_analog_stick_startup_system),
    );
}
