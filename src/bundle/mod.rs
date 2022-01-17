use bevy::prelude::*;
use heron::prelude::*;
use bevy_prototype_lyon::prelude::*;

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

    #[bundle]
    shape_bundle: bevy_prototype_lyon::entity::ShapeBundle,
}
impl CreatureBundle {
    pub fn with_max_life(max_life: f32) -> Self {
        CreatureBundle {
            shape_bundle: bevy_prototype_lyon::entity::ShapeBundle::default(),
            life: components::Life::with_max(max_life),
            direction: components::Direction::South,

            rigid_body: RigidBody::Dynamic,
            collision_shape: CollisionShape::Cuboid {
                half_extends: Vec3::new(5.0, 5.0, 1000.0),
                border_radius: Some(2.0)
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
                shape_bundle: GeometryBuilder::build_as(
                    &shapes.object_south,
                    DrawMode::Outlined {
                        fill_mode: FillMode::color(Color::CYAN),
                        outline_mode: StrokeMode::new(Color::BLACK, 0.1),
                    },
                    Transform::default(),
                ),
                ..CreatureBundle::with_max_life(max_life)
            },
        }
    }
}