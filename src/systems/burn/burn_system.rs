use bevy::prelude::*;

use crate::components;
use crate::bundle;
use crate::components::Burnable;
use crate::resources;

pub fn burn_system(
    mut commands: Commands,
    query: Query<(Entity, &components::Burnable), Changed<components::Burnable>>,
    shapes: Res<resources::Shapes>,
) {
    for (entity, burnable) in query.iter() {
        if burnable.resist <= 0.0 {
            let fire = commands.spawn_bundle(bundle::Fire::new(&shapes, burnable)).id();
            commands.entity(entity)
                .remove::<Burnable>();
            commands.entity(entity)
                .push_children(&[fire]);
        }
    }
}