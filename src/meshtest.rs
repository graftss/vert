use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub fn setup_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let material = ColorMaterial {
        color: Color::WHITE,
        texture: Some(asset_server.load("images/vert-fullsize.png")),
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(material),
        ..Default::default()
    });
}
