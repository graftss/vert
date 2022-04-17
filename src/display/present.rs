use bevy::{pbr::VisiblePointLights, prelude::*};

use crate::{state::AppState, MainCameraMarker};

use super::{
    display::{InputDisplay, TaggedAtomicParams},
    frame::FrameParams,
    system::add_display_systems,
};

pub struct StateBeforePresent {
    window_pos: Rect<f32>,
    camera_frame: Rect<f32>,
}

pub fn enter_present_system(
    mut windows: ResMut<Windows>,
    mut display: Res<InputDisplay>,
    mut query: Query<(&mut OrthographicProjection, &mut Transform), With<MainCameraMarker>>,
) {
    if let Some(window) = windows.get_primary_mut() {
        // Set the window size equal to the frame size
        let tagged_frame = display
            .atoms
            .iter()
            .find(|&atom| matches!(&atom.params, TaggedAtomicParams::Frame(_)));

        for atom in display.atoms.iter() {
            if let TaggedAtomicParams::Frame(fp) = atom.params {
                let FrameParams {
                    width,
                    height,
                    left,
                    bottom,
                    thickness,
                } = fp;

                // Update the window size
                window.set_resolution(width - thickness * 2.0, height - thickness * 2.0);
                window.set_resizable(false);

                let (mut orth_proj, mut transform) = query.single_mut();
                orth_proj.scale = 1.0;
                transform.translation.x = left + width / 2.0;
                transform.translation.y = bottom + height / 2.0;
                break;
            }
        }
    } else {
        println!("Error finding primary window (entering present mode)");
    }
}

pub fn exit_present_system(mut windows: ResMut<Windows>, mut window_desc: Res<WindowDescriptor>) {
    if let Some(window) = windows.get_primary_mut() {
        window.set_resolution(window_desc.width, window_desc.height);
        window.set_resizable(true);
    } else {
        println!("Error finding primary window (exiting present mode)");
    }
}

pub fn add_present_systems(app: &mut App, present_state: AppState) {
    // Enter present state
    app.add_system_set(SystemSet::on_enter(present_state).with_system(enter_present_system));

    // Exit present state
    app.add_system_set(SystemSet::on_exit(present_state).with_system(exit_present_system));
}
