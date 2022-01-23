use bevy::prelude::*;
use heron::prelude::*;

mod events;
mod bundle;
mod components;
mod resources;
mod systems;



#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, SystemLabel)]
pub enum GameSystemLabel {
    Input
}


fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "My test game".into(),
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_event::<events::InputEvent>()
        .add_event::<events::BurnBurnableEvent>()
        .add_startup_system(systems::startup_system)

        .add_system(systems::input_system.label(GameSystemLabel::Input))
        .add_system(systems::player_system.after(GameSystemLabel::Input))
        .add_system(systems::life_display_system)
        .add_system(systems::shape_update_system)
        .add_system(systems::burn_system)
        .add_system(systems::collision_handler_system)
        .add_system(systems::burn::burning_system::burning_system)
        .add_system(systems::burn::burn_down_system::burn_down_system)

        .run();
}
