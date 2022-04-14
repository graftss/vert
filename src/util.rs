use bevy::prelude::*;

pub fn despawn_all_with<C: Component>(mut commands: Commands, query: Query<Entity, With<C>>) {
    for entity in query.iter() {
        println!("despawning entity with id {:?}", entity);
        commands.entity(entity).despawn();
    }
}
