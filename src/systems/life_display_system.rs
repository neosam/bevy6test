use bevy::prelude::*;

use crate::components;

pub fn life_display_system(
    mut ui_query: Query<&mut Style, With<components::LifeUI>>,
    player_query: Query<&components::Health, With<components::Player>>,
) {
    if let (Ok(mut style), Ok(life)) = (ui_query.get_single_mut(), player_query.get_single()) {
        style.size.width = Val::Percent(life.current / life.max * 100.0);
    }
}
