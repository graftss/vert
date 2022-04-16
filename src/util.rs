use std::{error::Error, fs::File, io::BufReader};

use bevy::prelude::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub fn despawn_all_with<C: Component>(mut commands: Commands, query: Query<Entity, With<C>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

// Write a struct of type `T` to the file at `path`.
pub fn write_to_file<T>(data: &T, path: &str)
where
    T: Serialize,
{
    match serde_json::to_string_pretty(data) {
        Ok(json) => {
            if let Err(err) = std::fs::write(path, json) {
                println!(
                    "Error writing '{:?}' data to '{:?}': {:?}",
                    std::any::type_name::<T>(),
                    path,
                    err
                );
            }
        }
        Err(err) => {
            println!(
                "Error serializing '{:?}' data: {:?}.",
                std::any::type_name::<T>(),
                err
            );
        }
    }
}

// Read a struct of type `T` from the file at `path`.
pub fn read_from_file<T>(path: &str) -> Result<T, Box<dyn Error>>
where
    T: DeserializeOwned,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader)?;
    Ok(data)
}

pub fn div_vec2(a: &Vec2, b: &Vec2) -> Vec2 {
    Vec2::new(a.x / b.x, a.y / b.y)
}

pub fn screen_to_world(
    transform: &GlobalTransform,
    camera: &Camera,
    window: &Window,
    screen_point: &Vec2,
) -> Vec2 {
    let screen_size = Vec2::from([window.width(), window.height()]);
    let screen_ndc = div_vec2(screen_point, &screen_size) * 2.0 - Vec2::from([1.0, 1.0]);
    let camera_matrix = transform.compute_matrix();
    let ndc_to_world: Mat4 = camera_matrix * camera.projection_matrix.inverse();
    ndc_to_world
        .transform_point3(screen_ndc.extend(1.0))
        .truncate()
}
