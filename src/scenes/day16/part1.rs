use bevy::{
    app::Update,
    color::{palettes, Color, Srgba},
    prelude::{
        in_state, Animatable, BuildChildren, ChildBuild, ChildBuilder, Commands, Condition,
        DespawnRecursiveExt, IntoSystemConfigs, NextState, OnExit, Res, ResMut, Text,
    },
    ui::{
        BackgroundColor, BorderColor, BorderRadius, FlexDirection, Node, PositionType, UiRect, Val,
    },
};

use crate::{
    scenes::{
        days::{build_content, build_header},
        resources::{FontHandles, GenericDay},
        states::{Part, Scene, UiState, VisualizationState},
    },
    tools::{Coord, Maze, Vec2d},
};

use super::input::Input;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            OnExit(Part::Part1),
            tear_down_sokoban.run_if(in_state(Scene::Day(16))),
        )
        .add_systems(
            Update,
            build_ui
                .run_if(in_state(Part::Part1).and(in_state(VisualizationState::<16>::WaitingUi))),
        );
    }
}

fn tear_down_sokoban(
    mut commands: Commands,
    day16_resource: Res<GenericDay>,
    mut next_state: ResMut<NextState<UiState>>,
) {
    commands.entity(day16_resource.ui).despawn_descendants();
    next_state.set(UiState::NotLoaded);
}

fn build_ui(
    mut commands: Commands,
    day16_resource: Res<GenericDay>,
    mut input: ResMut<Input>,
    mut next_state: ResMut<NextState<UiState>>,
    fonts: Res<FontHandles>,
) {
    bevy::log::trace!("Day 16 Part 1");
    let header = build_header(&mut commands, "day16", true, fonts.font.clone());
    let content = build_content(&mut commands, "day16");

    commands
        .entity(content)
        .with_children(|parent| build_visualization(parent, &mut input));

    commands
        .entity(day16_resource.ui)
        .despawn_descendants()
        .add_children(&[header, content]);

    next_state.set(UiState::Loaded);
}

fn build_visualization(parent: &mut ChildBuilder, input: &mut Input) {
    let maze = Maze::parse(&mut input.input, 1000);

    let (mut maze_tiles_data, _) = maze.calculate_tile_scores();
    let maze_tiles = Vec2d::new(maze_tiles_data.as_mut_slice(), maze.width(), maze.height());

    let start = maze_tiles[maze.start()];
    let end = maze_tiles[maze.end()];

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
                    parent.spawn(Node::default()).with_child(Text::new("Score"));
                    parent
                        .spawn((
                            Node {
                                border: UiRect::all(Val::Px(3.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_child(Text::new(end.to_string()));
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
                                aspect_ratio: Some(maze.width() as f32 / maze.height() as f32),
                                border: UiRect::all(Val::Px(3.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                        ))
                        .with_children(|parent| {
                            for y in 0..maze.height() {
                                parent
                                    .spawn(Node {
                                        width: Val::Percent(100.),
                                        height: Val::Percent(10000. / maze.height() as f32),
                                        flex_direction: FlexDirection::Row,
                                        ..Default::default()
                                    })
                                    .with_children(|parent| {
                                        for x in 0..maze.width() {
                                            let background = if maze_tiles[Coord::new(y, x)]
                                                == usize::MAX
                                            {
                                                BackgroundColor(palettes::tailwind::GRAY_700.into())
                                            } else {
                                                let tile = maze_tiles[Coord::new(y, x)];
                                                let start_color = palettes::tailwind::BLUE_200;
                                                let end_color = palettes::tailwind::PURPLE_700;
                                                Srgba::interpolate(
                                                    &start_color,
                                                    &end_color,
                                                    (tile - start) as f32 / (end - start) as f32,
                                                )
                                                .into()
                                            };

                                            parent.spawn((
                                                Node {
                                                    height: Val::Percent(100.),
                                                    aspect_ratio: Some(1.),

                                                    ..Default::default()
                                                },
                                                background,
                                            ));
                                        }
                                    });
                            }
                        });
                });
        });
}
