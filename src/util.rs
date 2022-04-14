use bevy::prelude::*;

pub fn despawn_all_with<C: Component>(mut commands: Commands, query: Query<Entity, With<C>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
