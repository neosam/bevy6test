use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub struct Shapes {
    pub object_north: shapes::Polygon,
    pub object_south: shapes::Polygon,
    pub object_east: shapes::Polygon,
    pub object_west: shapes::Polygon,
}
impl Default for Shapes {
    fn default() -> Self {
        Shapes {
            object_north:
                shapes::Polygon {
                    points: vec![
                        Vec2::new(0.0, 1.0),
                        Vec2::new(-1.0, -1.0),
                        Vec2::new(1.0, -1.0),
                    ],
                    closed: true,
                },
            object_south:
                shapes::Polygon {
                    points: vec![
                        Vec2::new(0.0, -1.0),
                        Vec2::new(-1.0, 1.0),
                        Vec2::new(1.0, 1.0),
                    ],
                    closed: true,
                },
            object_east:
                shapes::Polygon {
                    points: vec![
                        Vec2::new(1.0, 0.0),
                        Vec2::new(-1.0, -1.0),
                        Vec2::new(-1.0, 1.0),
                    ],
                    closed: true,
                },
            object_west:
                shapes::Polygon {
                    points: vec![
                        Vec2::new(-1.0, 0.0),
                        Vec2::new(1.0, -1.0),
                        Vec2::new(1.0, 1.0),
                    ],
                    closed: true,
                },
        }
    }
}

#[derive(Default)]
pub struct InputStore {
    pub player_move_north: bool,
    pub player_move_south: bool,
    pub player_move_east: bool,
    pub player_move_west: bool,
}