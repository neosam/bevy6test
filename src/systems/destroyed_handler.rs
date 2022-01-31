use bevy::prelude::*;

use crate::{components, bundle, resources};

pub fn destroyed_handler(
    mut commands: Commands,
    destroyed_query: Query<(Entity, &Transform, &components::Destroyed, &components::DestroyTransform)>,
    sprites: Res<resources::SpriteIndices>,
) {
    for (entity, transform, _, destroy_transform) in destroyed_query.iter() {
        match *destroy_transform {
            components::DestroyTransform::Log =>
                commands.spawn_bundle(bundle::LogBundle::new(&sprites, transform.translation))
                    .insert(Name::new("Log")),
        };

        commands.entity(entity).despawn_recursive();
    }
}
