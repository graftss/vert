use bevy::{core::FixedTimestep, prelude::*};
use bevy_prototype_lyon::prelude::*;

mod input;

const POLL_RAWINPUT_TIME_STEP: f32 = 1.0 / 30.0;
const TIME_STEP: f32 = 1.0 / 5.0;

fn main() {
    let mut app = App::new();

    app.init_non_send_resource::<input::RawInputRes>()
        .add_plugins(DefaultPlugins)
        // .add_system_set(
        //     SystemSet::new()
        //         .with_run_criteria(FixedTimestep::step(POLL_RAWINPUT_TIME_STEP as f64))
        //         .with_system(input::poll_rawinput_system),
        // )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(input::test_gamepad_system),
        );

    app.run();
}
