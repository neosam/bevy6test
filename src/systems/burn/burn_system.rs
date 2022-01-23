use bevy::prelude::*;

use crate::components;
use crate::bundle;
use crate::resources;

pub fn burn_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut components::Burnable), Changed<components::Burnable>>,
    shapes: Res<resources::Shapes>,
) {
    for (entity, mut burnable) in query.iter_mut() {
        if !burnable.inactive && burnable.resist <= 0.0 {
            burnable.inactive = true;
            let fire = commands.spawn_bundle(bundle::Fire::new(&shapes, &burnable)).id();
            commands.entity(entity)
                .push_children(&[fire]);
        }
    }
}