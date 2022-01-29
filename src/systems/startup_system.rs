use bevy::prelude::*;

use crate::bundle;
use crate::components;
use crate::resources;

pub fn startup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let shapes = resources::Shapes {
        object_north: asset_server.load("north.png"),
        object_south: asset_server.load("south.png"),
        object_east: asset_server.load("east.png"),
        object_west: asset_server.load("west.png"),
        tree: asset_server.load("tree.png"),
        camp: asset_server.load("camp.png"),
        fire: asset_server.load("fire.png"),
        bullet: asset_server.load("bullet.png"),
    };

    commands.spawn_bundle(OrthographicCameraBundle {
        orthographic_projection: OrthographicProjection {
            scale: 1.0 / 40.0,
            ..Default::default()
        },
        ..OrthographicCameraBundle::new_2d()
    });
    commands.spawn_bundle(bundle::PlayerBundle::from_max_life(100.0, &shapes));
    for y in -10..10 {
        commands.spawn_bundle(bundle::TreeBundle::new(&shapes, -6.0, y as f32));
    }

    commands.spawn_bundle(bundle::CampfireBundle::new(&shapes, -2.0, -1.0));

    commands.spawn_bundle(UiCameraBundle {
        ..Default::default()
    });

    // Root
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // Bottom Bar
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        min_size: Size::new(Val::Auto, Val::Px(50.0)),
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    color: Color::GRAY.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Life Block
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Px(100.0), Val::Auto),
                                ..Default::default()
                            },
                            color: Color::GRAY.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // Life display
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(66.67), Val::Auto),
                                        ..Default::default()
                                    },
                                    color: Color::RED.into(),
                                    ..Default::default()
                                })
                                .insert(components::LifeUI);
                        });

                    // Bottom Bar separator
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Px(1.0), Val::Auto),
                            ..Default::default()
                        },
                        color: Color::BLACK.into(),
                        ..Default::default()
                    });
                });

            // Separator
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Px(1.0)),
                    ..Default::default()
                },
                color: Color::BLACK.into(),
                ..Default::default()
            });
        });

    commands.insert_resource(shapes);
    commands.insert_resource(resources::InputStore::default());
}
