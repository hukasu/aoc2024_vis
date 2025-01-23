use std::{collections::BTreeSet, time::Duration};

use bevy::{
    app::Update,
    color::{palettes, Color},
    prelude::{
        in_state, resource_changed, BuildChildren, ChildBuild, ChildBuilder, Commands, Component,
        Condition, DespawnRecursiveExt, Entity, IntoSystemConfigs, NextState, Res, ResMut, Single,
        Text, With,
    },
    text::TextColor,
    time::common_conditions::on_timer,
    ui::{
        BackgroundColor, BorderColor, BorderRadius, FlexDirection, Node, PositionType, UiRect, Val,
    },
};

use crate::scenes::{
    days::{build_content, build_header},
    resources::{FontHandles, GenericDay},
    states::{UiState, VisualizationState},
};

use super::input::{Input, BOUNDS};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui.run_if(in_state(VisualizationState::<14>::WaitingUi)),
        )
        .add_systems(
            Update,
            update_canvas
                .run_if(in_state(VisualizationState::<14>::Ready).and(resource_changed::<Input>)),
        )
        .add_systems(
            Update,
            step_robots.run_if(
                in_state(VisualizationState::<14>::Ready).and(on_timer(Duration::from_millis(100))),
            ),
        );
    }
}

fn build_ui(
    mut commands: Commands,
    day14_resource: Res<GenericDay>,
    input: Res<Input>,
    mut next_state: ResMut<NextState<UiState>>,
    fonts: Res<FontHandles>,
) {
    bevy::log::trace!("Day 14");
    let header = build_header(&mut commands, "day14", false, fonts.font.clone());
    let content = build_content(&mut commands, "day14");

    commands
        .entity(content)
        .with_children(|parent| build_visualization(parent, &input));

    commands
        .entity(day14_resource.ui)
        .despawn_descendants()
        .add_children(&[header, content]);

    next_state.set(UiState::Loaded);
}

fn build_visualization(parent: &mut ChildBuilder, input: &Input) {
    let safety_factor = input.safety_factor(100);
    let time_to_easter_egg = input.easter_egg();

    parent
        .spawn(Node {
            top: Val::Px(50.),
            bottom: Val::Px(10.),
            left: Val::Px(10.),
            right: Val::Px(10.),
            flex_direction: FlexDirection::Column,
            position_type: PositionType::Absolute,
            row_gap: Val::Px(12.),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        })
                        .with_child((Text::new("Safety factor"), TextColor(Color::WHITE)));
                    parent
                        .spawn((
                            Node {
                                border: UiRect::all(Val::Px(3.)),
                                padding: UiRect::all(Val::Px(3.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_child((
                            Text::new(safety_factor.to_string()),
                            TextColor(Color::WHITE),
                        ));
                    parent
                        .spawn(Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        })
                        .with_child((Text::new("|"), TextColor(Color::WHITE)));
                    parent
                        .spawn(Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        })
                        .with_child((Text::new("Easter egg in"), TextColor(Color::WHITE)));
                    parent
                        .spawn((
                            Node {
                                border: UiRect::all(Val::Px(3.)),
                                padding: UiRect::all(Val::Px(3.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_child((
                            Text::new(time_to_easter_egg.to_string()),
                            TextColor(Color::WHITE),
                        ));
                    parent
                        .spawn(Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        })
                        .with_child((Text::new("seconds"), TextColor(Color::WHITE)));
                });

            parent
                .spawn(Node {
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    align_self: bevy::ui::AlignSelf::Center,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Node {
                            height: Val::Percent(100.),
                            flex_direction: FlexDirection::Column,
                            flex_wrap: bevy::ui::FlexWrap::NoWrap,
                            aspect_ratio: Some(BOUNDS.0 as f32 / BOUNDS.1 as f32),
                            border: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        BorderColor(Color::WHITE),
                        RobotCanvas,
                    ));
                });
        });
}

#[derive(Debug, Component)]
pub struct RobotCanvas;

fn step_robots(mut input: ResMut<Input>) {
    input.step(1);
}

fn update_canvas(
    mut commands: Commands,
    canvas: Single<Entity, With<RobotCanvas>>,
    input: Res<Input>,
) {
    let positions = BTreeSet::from_iter(input.robots.iter().map(|robot| robot.position));

    commands
        .entity(*canvas)
        .despawn_descendants()
        .with_children(|parent| {
            for y in 0..BOUNDS.1 {
                parent
                    .spawn(Node {
                        width: Val::Percent(100.),
                        height: Val::Percent(10000. / BOUNDS.1 as f32),
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        for x in 0..BOUNDS.0 {
                            let mut tile = parent.spawn(Node {
                                height: Val::Percent(100.),
                                aspect_ratio: Some(1.),
                                ..Default::default()
                            });

                            if positions.contains(&(x, y)) {
                                tile.insert(BackgroundColor(palettes::tailwind::AMBER_800.into()));
                            }
                        }
                    });
            }
        });
}
