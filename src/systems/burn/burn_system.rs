use bevy::prelude::*;

use crate::bundle;
use crate::components;
use crate::resources;

pub fn burn_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut components::Burnable), Changed<components::Burnable>>,
    sprites: Res<resources::SpriteIndices>,
) {
    for (entity, mut burnable) in query.iter_mut() {
        if !burnable.inactive && burnable.resist <= 0.0 {
            burnable.inactive = true;
            burnable.burning = true;
            let fire = commands
                .spawn_bundle(bundle::Fire::new(&sprites, &burnable))
                .insert(Name::new("Fire"))
                .id();
            commands.entity(entity).push_children(&[fire]);
        }
    }
}
