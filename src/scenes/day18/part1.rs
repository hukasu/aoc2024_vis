use std::collections::BTreeSet;

use bevy::{
    app::Update,
    color::{palettes, Color},
    prelude::{
        in_state, BuildChildren, ChildBuild, ChildBuilder, Commands, Condition,
        DespawnRecursiveExt, IntoSystemConfigs, NextState, Res, ResMut, Text,
    },
    ui::{
        BackgroundColor, BorderColor, BorderRadius, FlexDirection, Node, PositionType, UiRect, Val,
    },
};

use crate::{
    scenes::{
        days::{build_content, build_header},
        resources::{FontHandles, GenericDay},
        states::{Part, UiState, VisualizationState},
    },
    tools::{Coord, Maze, Vec2d},
};

use super::input::{Input, BOUNDS};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui
                .run_if(in_state(Part::Part1).and(in_state(VisualizationState::<18>::WaitingUi))),
        );
    }
}

fn build_ui(
    mut commands: Commands,
    day18_resource: Res<GenericDay>,
    mut input: ResMut<Input>,
    mut next_state: ResMut<NextState<UiState>>,
    fonts: Res<FontHandles>,
) {
    bevy::log::trace!("Day 18 Part 1");
    let header = build_header(&mut commands, "day18", true, fonts.font.clone());
    let content = build_content(&mut commands, "day18");

    commands
        .entity(content)
        .with_children(|parent| build_visualization(parent, &mut input));

    commands
        .entity(day18_resource.ui)
        .despawn_descendants()
        .add_children(&[header, content]);

    next_state.set(UiState::Loaded);
}

fn build_visualization(parent: &mut ChildBuilder, input: &mut Input) {
    let obstacles = BTreeSet::from_iter(input.bytes[..1024].iter());
    let obstacles_ref = &obstacles;
    let mut maze_tiles = (0..BOUNDS.row)
        .flat_map(|y| {
            (0..BOUNDS.column).map(move |x| {
                let coord = Coord::new(y, x);
                if obstacles_ref.contains(&coord) {
                    b'#'
                } else if coord == Coord::new(0, 0) {
                    b'S'
                } else if coord == Coord::new(70, 70) {
                    b'E'
                } else {
                    b'.'
                }
            })
        })
        .collect::<Vec<_>>();
    let maze_vec = Vec2d::new(&mut maze_tiles, BOUNDS.row, BOUNDS.column);
    let maze = Maze::new(maze_vec, Coord::new(0, 0), Coord::new(70, 70), BOUNDS, 0);

    let (mut scores_data, path) = maze.calculate_tile_scores();
    let path = {
        let scores = Vec2d::new(&mut scores_data, BOUNDS.row, BOUNDS.column);
        let mut used_scores = BTreeSet::new();
        BTreeSet::from_iter(
            path.iter()
                .filter(|coord| used_scores.insert(scores[**coord])),
        )
    };

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
                    column_gap: Val::Px(5.),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(Node::default()).with_child(Text::new("Steps"));
                    parent
                        .spawn((
                            Node {
                                border: UiRect::all(Val::Px(3.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_child(Text::new((path.len() - 1).to_string()));
                });

            parent
                .spawn(Node {
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    align_self: bevy::ui::AlignSelf::Center,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                height: Val::Percent(100.),
                                flex_direction: FlexDirection::Column,
                                flex_wrap: bevy::ui::FlexWrap::NoWrap,
                                aspect_ratio: Some(BOUNDS.column as f32 / BOUNDS.row as f32),
                                border: UiRect::all(Val::Px(3.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_children(|parent| {
                            for y in 0..BOUNDS.row {
                                parent
                                    .spawn(Node {
                                        width: Val::Percent(100.),
                                        height: Val::Percent(10000. / maze.height() as f32),
                                        flex_direction: FlexDirection::Row,
                                        ..Default::default()
                                    })
                                    .with_children(|parent| {
                                        for x in 0..BOUNDS.column {
                                            let coord = Coord::new(y, x);
                                            let color = if obstacles.contains(&coord) {
                                                palettes::tailwind::GRAY_700.into()
                                            } else if path.contains(&coord) {
                                                palettes::tailwind::GREEN_300.into()
                                            } else {
                                                palettes::tailwind::YELLOW_300.into()
                                            };
                                            parent.spawn((
                                                Node {
                                                    height: Val::Percent(100.),
                                                    aspect_ratio: Some(1.),
                                                    ..Default::default()
                                                },
                                                BackgroundColor(color),
                                            ));
                                        }
                                    });
                            }
                        });
                });
        });
}
