use bevy::prelude::*;
use state::{add_state_systems, state_hotkey_system, AppState};

use bevy_egui::EguiPlugin;
use bevy_inspector_egui::{
    widgets::InspectorQuery, InspectableRegistry, InspectorPlugin, RegisterInspectable,
    WorldInspectorParams, WorldInspectorPlugin,
};
use bevy_prototype_lyon::prelude::*;
use controller::system::add_controller_systems;
use display::{
    display::{InputDisplay, RootAtomicDisplayMarker},
    present::add_present_systems,
    system::add_display_systems,
    test::{
        clear_display_hotkey, inject_debug_display, inject_debug_display_hotkey,
        save_display_hotkey,
    },
};
use editor::system::add_editor_systems;
use input::input::add_input_systems;

mod controller;
mod display;
mod editor;
mod input;
mod state;
mod util;

pub const VERSION: &'static str = "0.1";

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

    #[cfg(debug_assertions)]
    add_debug_tools(&mut app);

    app.add_startup_system(root_startup_system);
    add_state_systems(&mut app);
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

fn print_display_res(display: Option<Res<InputDisplay>>, kb_input: Res<Input<KeyCode>>) {
    if kb_input.just_pressed(KeyCode::F6) {
        println!("display: {:?}", display);
    }
}

fn add_debug_tools(app: &mut App) {
    // app.insert_resource(WorldInspectorParams {
    //     despawnable_entities: true,
    //     highlight_changes: true,
    //     ..Default::default()
    // });

    app.add_plugin(WorldInspectorPlugin::new());
    // app.register_inspectable::<InputDisplayRes>();
    // add console-based FPS logging
    // app.add_plugin(LogDiagnosticsPlugin::default());
    // app.add_plugin(FrameTimeDiagnosticsPlugin::default());

    // Add some fixed input displays for testing
    app.add_system(save_display_hotkey);
    app.add_system(print_display_res);
    app.add_system(inject_debug_display_hotkey);
    app.add_system(clear_display_hotkey);
}
