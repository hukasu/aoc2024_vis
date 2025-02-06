use bevy::{
    app::Update,
    color::Color,
    prelude::{
        in_state, BuildChildren, ChildBuild, ChildBuilder, Commands, Condition,
        DespawnRecursiveExt, IntoSystemConfigs, NextState, Res, ResMut, Text,
    },
    ui::{
        BorderColor, BorderRadius, Display, FlexDirection, GridPlacement, Node, PositionType,
        UiRect, Val,
    },
};

use crate::scenes::{
    days::{build_content, build_header},
    resources::{FontHandles, GenericDay},
    states::{Part, UiState, VisualizationState},
};

use super::input::Input;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui::<false>
                .run_if(in_state(Part::Part1).and(in_state(VisualizationState::<21>::WaitingUi))),
        )
        .add_systems(
            Update,
            build_ui::<true>
                .run_if(in_state(Part::Part2).and(in_state(VisualizationState::<21>::WaitingUi))),
        );
    }
}

fn build_ui<const PART2: bool>(
    mut commands: Commands,
    day21_resource: Res<GenericDay>,
    mut input: ResMut<Input>,
    mut next_state: ResMut<NextState<UiState>>,
    fonts: Res<FontHandles>,
) {
    bevy::log::trace!("Day 21");
    let header = build_header(&mut commands, "day21", true, fonts.font.clone());
    let content = build_content(&mut commands, "day21");

    commands
        .entity(content)
        .with_children(|parent| build_visualization::<PART2>(parent, &mut input));

    commands
        .entity(day21_resource.ui)
        .despawn_descendants()
        .add_children(&[header, content]);

    next_state.set(UiState::Loaded);
}

fn build_visualization<const PART2: bool>(parent: &mut ChildBuilder, input: &mut Input) {
    let (presses_per_code, presses) = if PART2 { input.run(25) } else { input.run(2) };

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
                    column_gap: Val::Px(15.),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(Node::default()).with_child(Text::new("Total"));
                    parent
                        .spawn((
                            Node {
                                border: UiRect::all(Val::Px(3.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_child(Text::new(
                            presses_per_code.iter().sum::<usize>().to_string(),
                        ));

                    for (code, cost) in input.codes.iter().zip(presses_per_code) {
                        parent
                            .spawn(Node::default())
                            .with_child(Text::new(String::from_utf8_lossy(code.0.as_slice())));
                        parent
                            .spawn((
                                Node {
                                    border: UiRect::all(Val::Px(3.)),
                                    ..Default::default()
                                },
                                BorderColor(Color::WHITE),
                                BorderRadius::all(Val::Px(5.)),
                            ))
                            .with_child(Text::new(cost.to_string()));
                    }
                });

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_self: bevy::ui::AlignSelf::Center,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((Node {
                            display: Display::Grid,
                            ..Default::default()
                        },))
                        .with_children(|parent| {
                            let list = [
                                0, b'A', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9',
                            ];
                            for (row, row_key) in list.iter().enumerate() {
                                for (column, column_key) in list.iter().enumerate() {
                                    if *row_key == 0 {
                                        if *column_key == 0 {
                                            parent
                                                .spawn((
                                                    Node {
                                                        grid_row: GridPlacement::start(
                                                            (row + 1) as i16,
                                                        ),
                                                        grid_column: GridPlacement::start(
                                                            (column + 1) as i16,
                                                        ),
                                                        flex_direction: FlexDirection::Column,
                                                        border: UiRect::all(Val::Px(1.)),
                                                        ..Default::default()
                                                    },
                                                    BorderColor(Color::WHITE),
                                                ))
                                                .with_children(|parent| {
                                                    parent
                                                        .spawn(Node {
                                                            padding: UiRect::left(Val::Px(10.)),
                                                            ..Default::default()
                                                        })
                                                        .with_child(Text::new("Destination"));
                                                    parent
                                                        .spawn(Node {
                                                            padding: UiRect::right(Val::Px(10.)),
                                                            ..Default::default()
                                                        })
                                                        .with_child(Text::new("Start"));
                                                });
                                        } else {
                                            parent
                                                .spawn((
                                                    Node {
                                                        grid_row: GridPlacement::start(
                                                            (row + 1) as i16,
                                                        ),
                                                        grid_column: GridPlacement::start(
                                                            (column + 1) as i16,
                                                        ),
                                                        flex_direction: FlexDirection::Column,
                                                        border: UiRect::all(Val::Px(1.)),
                                                        ..Default::default()
                                                    },
                                                    BorderColor(Color::WHITE),
                                                ))
                                                .with_child(Text::new(String::from_utf8_lossy(&[
                                                    *column_key,
                                                ])));
                                        }
                                    } else if *column_key == 0 {
                                        parent
                                            .spawn((
                                                Node {
                                                    grid_row: GridPlacement::start(
                                                        (row + 1) as i16,
                                                    ),
                                                    grid_column: GridPlacement::start(
                                                        (column + 1) as i16,
                                                    ),
                                                    flex_direction: FlexDirection::Column,
                                                    border: UiRect::all(Val::Px(1.)),
                                                    ..Default::default()
                                                },
                                                BorderColor(Color::WHITE),
                                            ))
                                            .with_child(Text::new(String::from_utf8_lossy(&[
                                                *row_key,
                                            ])));
                                    } else {
                                        parent
                                            .spawn((
                                                Node {
                                                    grid_row: GridPlacement::start(
                                                        (row + 1) as i16,
                                                    ),
                                                    grid_column: GridPlacement::start(
                                                        (column + 1) as i16,
                                                    ),
                                                    flex_direction: FlexDirection::Column,
                                                    border: UiRect::all(Val::Px(1.)),
                                                    ..Default::default()
                                                },
                                                BorderColor(Color::WHITE),
                                            ))
                                            .with_child(Text::new(
                                                presses
                                                    .get(&(*column_key, *row_key))
                                                    .copied()
                                                    .unwrap_or_default()
                                                    .to_string(),
                                            ));
                                    }
                                }
                            }
                        });
                });
        });
}
