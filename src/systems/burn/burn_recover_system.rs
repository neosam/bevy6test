use bevy::prelude::*;

use crate::components;

pub fn burn_recover_system(mut query: Query<&mut components::Burnable>, time: Res<Time>) {
    for mut burnable in query.iter_mut() {
        if burnable.burning {
            burnable.resist += burnable.recover * time.delta_seconds();
            if burnable.resist >= burnable.max_resistence {
                burnable.resist = burnable.max_resistence;
            }
            if burnable.inactive && burnable.resist >= burnable.min_resistence_to_burn {
                burnable.inactive = false;
            }
        }
    }
}
