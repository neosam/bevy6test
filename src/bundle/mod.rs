use bevy::prelude::*;
use heron::prelude::*;

use crate::components;
use crate::resources;

#[derive(Bundle)]
pub struct CreatureBundle {
    life: components::Life,
    direction: components::Direction,
    rigid_body: RigidBody,
    collision_shape: CollisionShape,
    rotation_constraints: RotationConstraints,
    velocity: Velocity,
    creature_shape: components::CreatureShapes,

    #[bundle]
    sprite_bundle: SpriteBundle,
}
impl CreatureBundle {
    pub fn with_max_life(max_life: f32, shapes: &resources::Shapes) -> Self {
        CreatureBundle {
            sprite_bundle: SpriteBundle {
                texture: shapes.object_south.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..Default::default()
                },
                ..SpriteBundle::default()
            },
            life: components::Life::with_max(max_life),
            direction: components::Direction::South,
            creature_shape: components::CreatureShapes {
                north: shapes.object_north.clone(),
                south: shapes.object_south.clone(),
                east: shapes.object_east.clone(),
                west: shapes.object_west.clone(),
            },

            rigid_body: RigidBody::Dynamic,
            collision_shape: CollisionShape::Cuboid {
                half_extends: Vec3::new(0.4, 0.4, 1000.0),
                border_radius: Some(0.1)
            },
            rotation_constraints: RotationConstraints::lock(),
            velocity: Velocity::from_linear(Vec3::new(0.0, 0.0, 0.0)),
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
    pub fn from_max_life(max_life: f32, shapes: &resources::Shapes) -> Self {
        PlayerBundle {
            player: components::Player,
            creature: CreatureBundle {
                ..CreatureBundle::with_max_life(max_life, shapes)
            },
        }
    }
}

#[derive(Bundle)]
pub struct TreeBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,

    rigid_body: RigidBody,
    collision_shape: CollisionShape,
}
impl TreeBundle {
    pub fn new(shapes: &resources::Shapes, x: f32, y: f32) -> Self {
        TreeBundle {
            sprite_bundle: SpriteBundle {
                texture: shapes.tree.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(x, y, 100.0),
                ..Default::default()
            },
            rigid_body: RigidBody::Static,
            collision_shape: CollisionShape::Cuboid {
                half_extends: Vec3::new(0.4, 0.4, 1000.0),
                border_radius: Some(0.1),
            }
        }
    }
}
