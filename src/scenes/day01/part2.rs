use std::collections::{BTreeMap, BTreeSet};

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

use super::{input::Input, resources::Day01, states};

const SCROLL_SPEED: f32 = 512.;
const VAL_GRID_WIDTH: f32 = 130.;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui
                .run_if(in_state(states::States::Part2))
                .run_if(in_state(states::VisualizationState::WaitingUi)),
        )
        .add_systems(
            OnExit(states::States::Part2),
            destroy_ui.before(super::destroy_day_1),
        )
        .add_systems(
            Update,
            super::process_input
                .run_if(in_state(states::States::Part2))
                .run_if(in_state(states::VisualizationState::WaitingInput)),
        );
    }
}

fn build_ui(
    mut commands: Commands,
    day1_resource: Res<Day01>,
    input: Res<Input>,
    mut next_state: ResMut<NextState<states::UiState>>,
) {
    bevy::log::trace!("Day 1 Part 2");
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
    let all_ids = BTreeSet::from_iter(input.left.iter().chain(input.right.iter()).copied());

    let frequencies = BTreeMap::from_iter(
        all_ids
            .iter()
            .map(|id| (*id, input.right.iter().filter(|r| *r == id).count() as u32)),
    );

    let similarity_score: u32 = input
        .left
        .iter()
        .map(|id| id * frequencies.get(id).copied().unwrap_or(0))
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
                        .spawn(Node::default())
                        .with_child((Text::new("Similarity score"), TextColor(Color::WHITE)));
                    parent
                        .spawn((
                            Node {
                                border: UiRect::all(Val::Px(3.)),
                                justify_content: JustifyContent::End,
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_child((
                            Text::new(similarity_score.to_string()),
                            TextColor(Color::WHITE),
                        ));
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
                            padding: UiRect::all(Val::Px(5.)),
                            column_gap: Val::Px(5.),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(Node {
                                    min_width: Val::Px(VAL_GRID_WIDTH),
                                    ..Default::default()
                                })
                                .with_child((Text::new("Location ID"), TextColor::WHITE));
                            parent
                                .spawn(Node {
                                    min_width: Val::Px(VAL_GRID_WIDTH),
                                    ..Default::default()
                                })
                                .with_child((Text::new("Frequency"), TextColor::WHITE));
                            parent
                                .spawn(Node {
                                    min_width: Val::Px(VAL_GRID_WIDTH),
                                    ..Default::default()
                                })
                                .with_child((Text::new("Score"), TextColor::WHITE));
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
                            for id in &input.left {
                                parent
                                    .spawn((
                                        Node {
                                            flex_direction: FlexDirection::Row,
                                            border: UiRect::all(Val::Px(3.)),
                                            padding: UiRect::all(Val::Px(5.)),
                                            column_gap: Val::Px(5.),
                                            ..Default::default()
                                        },
                                        BorderColor(Color::WHITE),
                                        BorderRadius::all(Val::Px(5.)),
                                    ))
                                    .with_children(|parent| {
                                        let frequency = frequencies.get(id).copied().unwrap_or(0);
                                        parent
                                            .spawn(Node {
                                                min_width: Val::Px(VAL_GRID_WIDTH),
                                                ..Default::default()
                                            })
                                            .with_child((
                                                Text::new(id.to_string()),
                                                TextColor::WHITE,
                                            ));
                                        parent
                                            .spawn(Node {
                                                min_width: Val::Px(VAL_GRID_WIDTH),
                                                ..Default::default()
                                            })
                                            .with_child((
                                                Text::new(frequency.to_string()),
                                                TextColor::WHITE,
                                            ));
                                        parent
                                            .spawn(Node {
                                                min_width: Val::Px(VAL_GRID_WIDTH),
                                                ..Default::default()
                                            })
                                            .with_child((
                                                Text::new((id * frequency).to_string()),
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
