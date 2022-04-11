use bevy::{core::FixedTimestep, prelude::*};
use bevy_prototype_lyon::prelude::*;

mod input;

const TIME_STEP: f32 = 1.0 / 60.0;

fn poll_input(keyboard_input: Res<Input<KeyCode>>, mut input_state: ResMut<input::InputState>) {
    for (i, key) in input::KEYBOARD_KEYS.iter().enumerate() {
        input_state.keyboard[i] = keyboard_input.pressed(*key)
    }
}

fn print_input(all_input: Res<input::InputState>) {
    println!("hi {:?}", all_input.keyboard);
}

fn main() {
    App::new()
        .init_resource::<input::InputState>()
        .add_plugins(DefaultPlugins)
        .add_system(poll_input.label("input"))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(print_input.after("input")),
        )
        .run();
}
