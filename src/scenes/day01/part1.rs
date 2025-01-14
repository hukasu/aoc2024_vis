use bevy::{
    app::Update,
    color::Color,
    prelude::{
        in_state, BuildChildren, ChildBuild, ChildBuilder, Commands, DespawnRecursiveExt,
        IntoSystemConfigs, NextState, OnExit, Res, ResMut, Text,
    },
    text::{TextColor, TextFont},
    ui::{
        BackgroundColor, BorderColor, BorderRadius, FlexDirection, JustifyContent, Node, Overflow,
        PositionType, UiRect, Val,
    },
};

use crate::{
    scenes::{
        days::{build_content, build_header},
        FONT_SYMBOLS_HANDLE,
    },
    scroll_controls::{ScrollControl, ScrollWindow, BUTTON_BACKGROUND_COLOR},
};

use super::{
    input::Input,
    resources::Day01,
    states::{self, States, VisualizationState},
};

const SCROLL_SPEED: f32 = 512.;
const VAL_GRID_WIDTH: f32 = 80.;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui
                .run_if(in_state(States::Part1))
                .run_if(in_state(VisualizationState::WaitingUi)),
        )
        .add_systems(
            OnExit(States::Part1),
            destroy_ui.before(super::destroy_day_1),
        )
        .add_systems(
            Update,
            super::process_input
                .run_if(in_state(States::Part1))
                .run_if(in_state(VisualizationState::WaitingInput)),
        );
    }
}

fn build_ui(
    mut commands: Commands,
    day1_resource: Res<Day01>,
    input: Res<Input>,
    mut next_state: ResMut<NextState<states::UiState>>,
) {
    bevy::log::trace!("Day 1 Part 1");
    let header = build_header(&mut commands, "day1", true);
    let content = build_content(&mut commands, "day1");

    commands
        .entity(content)
        .with_children(|parent| build_visualization(parent, &input));

    commands
        .entity(day1_resource.ui)
        .add_children(&[header, content]);

    next_state.set(states::UiState::Loaded);
}

fn destroy_ui(
    mut commands: Commands,
    day1_resource: Res<Day01>,
    mut input_state: ResMut<NextState<states::InputState>>,
    mut ui_state: ResMut<NextState<states::UiState>>,
) {
    commands.remove_resource::<Input>();
    commands.entity(day1_resource.ui).despawn_descendants();

    input_state.set(states::InputState::NotLoaded);
    ui_state.set(states::UiState::NotLoaded);
}

fn build_visualization(parent: &mut ChildBuilder, input: &Input) {
    let mut left = input.left.clone();
    left.sort();
    let mut right = input.right.clone();
    right.sort();

    let diff: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    parent
        .spawn(Node {
            top: Val::Px(50.),
            bottom: Val::Px(10.),
            left: Val::Px(10.),
            right: Val::Px(10.),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(20.),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((Node {
                            ..Default::default()
                        },))
                        .with_child((Text::new("Total distance"), TextColor::WHITE));
                    parent
                        .spawn((
                            Node {
                                border: UiRect::all(Val::Px(3.)),
                                padding: UiRect::axes(Val::Px(5.), Val::Px(2.)),
                                justify_content: JustifyContent::End,
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_child((Text::new(diff.to_string()), TextColor::WHITE));
                });

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            padding: UiRect::axes(Val::Px(5.), Val::Px(2.)),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(Node {
                                    min_width: Val::Px(VAL_GRID_WIDTH),
                                    ..Default::default()
                                })
                                .with_child(Text::new("Left"));
                            parent
                                .spawn(Node {
                                    min_width: Val::Px(VAL_GRID_WIDTH),
                                    ..Default::default()
                                })
                                .with_child(Text::new("Right"));
                            parent
                                .spawn(Node {
                                    min_width: Val::Px(VAL_GRID_WIDTH),
                                    ..Default::default()
                                })
                                .with_child(Text::new("Diff"));
                        });
                    let window = parent
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                overflow: Overflow::scroll_y(),
                                ..Default::default()
                            },
                            ScrollWindow,
                        ))
                        .with_children(|parent| {
                            for (l, r) in left.iter().zip(right) {
                                parent
                                    .spawn((
                                        Node {
                                            border: UiRect::all(Val::Px(3.)),
                                            padding: UiRect::axes(Val::Px(5.), Val::Px(2.)),
                                            ..Default::default()
                                        },
                                        BorderColor(Color::WHITE),
                                        BorderRadius::all(Val::Px(5.)),
                                    ))
                                    .with_children(|parent| {
                                        parent
                                            .spawn(Node {
                                                min_width: Val::Px(VAL_GRID_WIDTH),
                                                ..Default::default()
                                            })
                                            .with_child((
                                                Text::new(l.to_string()),
                                                TextColor::WHITE,
                                            ));
                                        parent
                                            .spawn(Node {
                                                min_width: Val::Px(VAL_GRID_WIDTH),
                                                ..Default::default()
                                            })
                                            .with_child((
                                                Text::new(r.to_string()),
                                                TextColor::WHITE,
                                            ));
                                        parent
                                            .spawn(Node {
                                                min_width: Val::Px(VAL_GRID_WIDTH),
                                                ..Default::default()
                                            })
                                            .with_child((
                                                Text::new((l.abs_diff(r)).to_string()),
                                                TextColor::WHITE,
                                            ));
                                    });
                            }
                        })
                        .id();

                    parent
                        .spawn((Node {
                            bottom: Val::Px(1.),
                            right: Val::Px(1.),
                            position_type: PositionType::Absolute,
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(5.),
                            ..Default::default()
                        },))
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Node {
                                        width: Val::Px(25.),
                                        height: Val::Px(25.),
                                        justify_content: JustifyContent::Center,
                                        ..Default::default()
                                    },
                                    BackgroundColor(BUTTON_BACKGROUND_COLOR),
                                    ScrollControl {
                                        horizontal: 0.,
                                        vertical: -SCROLL_SPEED,
                                        target: window,
                                    },
                                ))
                                .with_child((
                                    Text::new("↑"),
                                    TextColor::BLACK,
                                    TextFont {
                                        font: FONT_SYMBOLS_HANDLE.get().unwrap().clone(),
                                        ..Default::default()
                                    },
                                ));
                            parent
                                .spawn((
                                    Node {
                                        width: Val::Px(25.),
                                        height: Val::Px(25.),
                                        justify_content: JustifyContent::Center,
                                        ..Default::default()
                                    },
                                    BackgroundColor(BUTTON_BACKGROUND_COLOR),
                                    ScrollControl {
                                        horizontal: 0.,
                                        vertical: SCROLL_SPEED,
                                        target: window,
                                    },
                                ))
                                .with_child((
                                    Text::new("↓"),
                                    TextColor::BLACK,
                                    TextFont {
                                        font: FONT_SYMBOLS_HANDLE.get().unwrap().clone(),
                                        ..Default::default()
                                    },
                                ));
                        });
                });
        });
}
