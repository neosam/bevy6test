use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::{WorldInspectorPlugin, RegisterInspectable};
use heron::prelude::*;

mod bundle;
mod components;
mod events;
mod resources;
mod systems;
mod state;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, SystemLabel)]
pub enum GameSystemLabel {
    Input,
    PlayerUpdate,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "My test game".into(),
            vsync: false,

            ..Default::default()
        })
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // Adds a system that prints diagnostics to the console
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<components::AutoDespawn>()
        .register_inspectable::<components::Burn>()
        .register_inspectable::<components::Burnable>()
        .register_inspectable::<components::CreatureShapes>()
        .register_inspectable::<components::Destroyed>()
        .register_inspectable::<components::Health>()
        .register_inspectable::<components::LifeUI>()
        .register_inspectable::<components::Player>()
        .register_inspectable::<components::Direction>()

        .add_state(state::State::Loading)
        .add_event::<events::InputEvent>()
        .add_event::<events::BurnBurnableEvent>()
        .insert_resource(resources::GraphicsHandles::default())
        .insert_resource(resources::SpriteIndices::default())

        .add_system_set(SystemSet::on_enter(state::State::Loading)
            .with_system(systems::loading::loading_startup))
        .add_system_set(SystemSet::on_update(state::State::Loading)
            .with_system(systems::loading::loading_update))
        .add_system_set(SystemSet::on_update(state::State::PostLoading)
            .with_system(systems::loading::post_loading_startup))

        .add_system_set(SystemSet::on_enter(state::State::Ingame)
            .with_system(systems::startup_system))
        .add_system_set(SystemSet::on_update(state::State::Ingame)
            .with_system(systems::input_system.label(GameSystemLabel::Input))
            .with_system(systems::player_system.after(GameSystemLabel::Input).label(GameSystemLabel::PlayerUpdate))
            .with_system(systems::player_shoot_system.after(GameSystemLabel::Input))
            .with_system(systems::camera_update_system)
            .with_system(systems::life_display_system)
            .with_system(systems::shape_update_system)
            .with_system(systems::burn_system)
            .with_system(systems::collision_handler_system)
            .with_system(systems::burn::burning_system::burning_system)
            .with_system(systems::burn::burn_down_system::burn_down_system)
            .with_system(systems::burn::burn_recover_system::burn_recover_system)
            .with_system(systems::damage_system)
            .with_system(systems::despawn_system)
        )
        .run();
}
