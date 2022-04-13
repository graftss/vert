use bevy::{
    core::FixedTimestep,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_prototype_lyon::prelude::*;
use input::input::{poll_input_sources, resolve_input_sinks_system, InputSource, InputValue};
use input::raw_input_reader::RawInputRes;

mod display;
mod input;

const POLL_RAWINPUT_TIME_STEP: f32 = 1.0 / 30.0;
const TIME_STEP: f32 = 1.0 / 30.0;

fn root_startup_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    let mut app = App::new();

    app.add_startup_system(root_startup_system);
    app.add_startup_system(display::button::test_button_startup_system);

    app.init_non_send_resource::<RawInputRes>()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(resolve_input_sinks_system.after("poll_input")),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(display::button::button_display_system),
        );

    #[cfg(target_os = "windows")]
    app.add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(POLL_RAWINPUT_TIME_STEP as f64))
            .with_system(input::raw_input_reader::poll_rawinput_system.label("poll_input")),
    );

    app.run();
}
