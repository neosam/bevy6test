use bevy::prelude::*;

use crate::bundle;
use crate::components;
use crate::resources;

pub fn startup_system(
    mut commands: Commands,
    sprites: Res<resources::SpriteIndices>,
) {
    commands.spawn_bundle(OrthographicCameraBundle {
        orthographic_projection: OrthographicProjection {
            scale: 1.0 / 40.0,
            ..Default::default()
        },
        ..OrthographicCameraBundle::new_2d()
    }).insert(components::MainCamera);
    commands.spawn_bundle(bundle::PlayerBundle::from_max_life(100.0, &sprites))
        .insert(Name::new("Player"));
    commands.spawn()
        .insert(Name::new("Trees"))
        .insert(Transform::identity())
        .insert(GlobalTransform::identity())
        .with_children(|parent| {
            for y in -10..10 {
                parent.spawn_bundle(bundle::TreeBundle::new(&sprites, -6.0, y as f32))
                    .insert(Name::new("Tree"));
            }
        });

    commands.spawn_bundle(bundle::CampfireBundle::new(&sprites, -2.0, -1.0))
        .insert(Name::new("Campfire"));

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
        .insert(Name::new("UI"))
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
                .insert(Name::new("Bottom Bar"))
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
                        .insert(Name::new("Life Block"))
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
                                .insert(components::LifeUI)
                                .insert(Name::new("Life Fill"));
                        });

                    // Bottom Bar separator
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Px(1.0), Val::Auto),
                            ..Default::default()
                        },
                        color: Color::BLACK.into(),
                        ..Default::default()
                    })
                    .insert(Name::new("Bottom Bar Separator"));
                });

            // Separator
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Px(1.0)),
                    ..Default::default()
                },
                color: Color::BLACK.into(),
                ..Default::default()
            })
            .insert(Name::new("Bar Top Separator"));
        });

    commands.insert_resource(resources::InputStore::default());
}
