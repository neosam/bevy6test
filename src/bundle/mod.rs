use bevy::prelude::*;
use heron::prelude::*;

use crate::components;
use crate::resources;

#[derive(Bundle)]
pub struct CreatureBundle {
    life: components::Health,
    direction: components::Direction,
    rigid_body: RigidBody,
    collision_shape: CollisionShape,
    rotation_constraints: RotationConstraints,
    velocity: Velocity,
    creature_shape: components::CreatureShapes,
    burnable: components::Burnable,

    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}
impl CreatureBundle {
    pub fn with_max_life(max_life: f32, sprites: &resources::SpriteIndices) -> Self {
        CreatureBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    index: sprites.object_south,
                    ..Default::default()
                },
                texture_atlas: sprites.atlas_handle.clone(),
                ..SpriteSheetBundle::default()
            },
            life: components::Health::with_max(max_life),
            direction: components::Direction::South,
            creature_shape: components::CreatureShapes {
                north: sprites.object_north,
                south: sprites.object_south,
                east: sprites.object_east,
                west: sprites.object_west,
            },

            rigid_body: RigidBody::Dynamic,
            collision_shape: CollisionShape::Cuboid {
                half_extends: Vec3::new(0.4, 0.4, 1000.0),
                border_radius: Some(0.1),
            },
            rotation_constraints: RotationConstraints::lock(),
            velocity: Velocity::from_linear(Vec3::new(0.0, 0.0, 0.0)),
            burnable: components::Burnable {
                resist: 5.0,
                max_resistence: 5.0,
                min_resistence_to_burn: 5.0,
                recover: 0.5,
                inactive: false,
                burning: false,
                burn: components::Burn {
                    fuel: 10.0,
                    radius: 1.0,
                    strength: 1.0,
                },
            },
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: components::Player,

    #[bundle]
    creature: CreatureBundle,
}
impl PlayerBundle {
    pub fn from_max_life(max_life: f32, sprites: &resources::SpriteIndices) -> Self {
        PlayerBundle {
            player: components::Player,
            creature: CreatureBundle {
                ..CreatureBundle::with_max_life(max_life, sprites)
            },
        }
    }
}

#[derive(Bundle)]
pub struct TreeBundle {
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,

    rigid_body: RigidBody,
    collision_shape: CollisionShape,
    burnable: components::Burnable,
    health: components::Health,
}
impl TreeBundle {
    pub fn new(sprites: &resources::SpriteIndices, x: f32, y: f32) -> Self {
        TreeBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    index: sprites.tree,
                    ..Default::default()
                },
                texture_atlas: sprites.atlas_handle.clone(),
                transform: Transform::from_xyz(x, y, 100.0),
                ..Default::default()
            },
            rigid_body: RigidBody::Static,
            collision_shape: CollisionShape::Cuboid {
                half_extends: Vec3::new(0.4, 0.4, 1000.0),
                border_radius: Some(0.1),
            },
            burnable: components::Burnable {
                resist: 5.0,
                max_resistence: 5.0,
                min_resistence_to_burn: 5.0,
                recover: 0.25,
                inactive: false,
                burning: false,
                burn: components::Burn {
                    fuel: 11.0,
                    radius: 1.0,
                    strength: 1.0,
                },
            },
            health: components::Health::with_max(10.0),
        }
    }
}

#[derive(Bundle)]
pub struct CampfireBundle {
    #[bundle]
    sprite_sheet_bundle: SpriteSheetBundle,

    burnable: components::Burnable,
}
impl CampfireBundle {
    pub fn new(sprites: &resources::SpriteIndices, x: f32, y: f32) -> Self {
        CampfireBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    index: sprites.camp,
                    ..Default::default()
                },
                transform: Transform::from_xyz(x, y, 100.0),
                texture_atlas: sprites.atlas_handle.clone(),
                ..Default::default()
            },
            burnable: components::Burnable {
                resist: 0.0,
                max_resistence: 1.0,
                min_resistence_to_burn: 1.0,
                recover: 1.0,
                inactive: false,
                burning: false,
                burn: components::Burn {
                    fuel: 10000000.0,
                    radius: 0.8,
                    strength: 1.0,
                },
            },
        }
    }
}

#[derive(Bundle)]
pub struct Fire {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,

    pub burn: components::Burn,
    pub rigid_body: RigidBody,
    pub collision_shape: CollisionShape,
}
impl Fire {
    pub fn new(sprites: &resources::SpriteIndices, burnable: &components::Burnable) -> Self {
        Fire {
            sprite_sheet_bundle: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(0.8, 0.8)),
                    index: sprites.fire,
                    ..Default::default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 100.0),
                texture_atlas: sprites.atlas_handle.clone(),
                ..Default::default()
            },
            burn: burnable.burn.clone(),
            rigid_body: RigidBody::Sensor,
            collision_shape: CollisionShape::Sphere {
                radius: burnable.burn.radius,
            },
        }
    }
}

#[derive(Bundle)]
pub struct BulletBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,

    pub rigid_body: RigidBody,
    pub collision_shape: CollisionShape,
    pub velocity: Velocity,
    pub burnable: components::Burnable,
    pub health: components::Health,
}
impl BulletBundle {
    pub fn new(
        sprites: &resources::SpriteIndices,
        direction: &components::Direction,
        position: Vec3,
        radius: f32,
        speed: f32,
    ) -> Self {
        let velocity_speed = match *direction {
            components::Direction::North => Vec3::new(0.0, speed, 0.0),
            components::Direction::South => Vec3::new(0.0, -speed, 0.0),
            components::Direction::East => Vec3::new(speed, 0.0, 0.0),
            components::Direction::West => Vec3::new(-speed, 0.0, 0.0),
        };
        BulletBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(radius / 2.0, radius / 2.0)),
                    index: sprites.bullet,
                    ..Default::default()
                },
                transform: Transform::identity().with_translation(position),
                texture_atlas: sprites.atlas_handle.clone(),
                ..Default::default()
            },

            rigid_body: RigidBody::Dynamic,
            collision_shape: CollisionShape::Sphere { radius: radius },
            velocity: Velocity::from_linear(velocity_speed),
            burnable: components::Burnable {
                resist: 0.1,
                max_resistence: 0.1,
                min_resistence_to_burn: 0.1,
                recover: 0.0,
                inactive: false,
                burning: false,
                burn: components::Burn {
                    fuel: 20.0,
                    radius: 0.3,
                    strength: 0.5,
                },
            },
            health: components::Health::with_max(1.0),
        }
    }
}
