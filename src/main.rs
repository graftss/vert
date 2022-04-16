use app_state::{state_hotkey_system, AppState};
use bevy::prelude::*;

use bevy_egui::EguiPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;
use controller::system::add_controller_systems;
use display::{
    present::add_present_systems,
    system::add_display_systems,
    test::{inject_debug_display, reinject_debug_display},
};
use editor::system::add_editor_systems;
use input::input::add_input_systems;

mod app_state;
mod controller;
mod display;
mod editor;
mod input;
mod util;

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "vert".to_string(),
        width: 800.,
        height: 600.,
        ..WindowDescriptor::default()
    });

    // Add plugins.
    app.add_plugins(DefaultPlugins);
    app.add_plugin(ShapePlugin);
    app.add_plugin(EguiPlugin);

    // Set initial value of `AppState`.
    app.add_state(AppState::ConfigureController);

    #[cfg(debug_assertions)]
    add_debug_tools(&mut app);

    app.add_startup_system(root_startup_system);
    app.add_system(state_hotkey_system);
    add_input_systems(&mut app);
    add_display_systems(&mut app);
    add_present_systems(&mut app, AppState::Present);
    add_controller_systems(&mut app, AppState::ConfigureController);
    add_editor_systems(&mut app, AppState::Editor);

    app.run();
}

#[derive(Component)]
pub struct MainCameraMarker;

#[derive(Component)]
pub struct UiCameraMarker;

fn root_startup_system(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCameraMarker);
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiCameraMarker);
}

fn add_debug_tools(app: &mut App) {
    app.add_plugin(WorldInspectorPlugin::new());

    // add console-based FPS logging
    // app.add_plugin(LogDiagnosticsPlugin::default());
    // app.add_plugin(FrameTimeDiagnosticsPlugin::default());

    // Add some fixed input displays for testing
    app.add_system(reinject_debug_display);
}
