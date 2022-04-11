use bevy:: {
    core::FixedTimestep,
    prelude::*,
};

mod input;


const TIME_STEP: f32 = 1.0 / 60.0;

fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
) {
    println!("hi {:?}", keyboard_input.pressed(KeyCode::W));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(input_system)
        )
.run();
}
