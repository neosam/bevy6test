use bevy::prelude::*;

use crate::components;

pub fn life_display_system(
    mut ui_query: Query<&mut Style, With<components::LifeUI>>,
    player_query: Query<&components::Life, With<components::Player>>,
) {
    let mut style = ui_query.single_mut();
    let life = player_query.single();
    style.size.width = Val::Percent(life.current / life.max * 100.0);
}
