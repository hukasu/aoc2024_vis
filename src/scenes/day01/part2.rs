use std::collections::{BTreeMap, BTreeSet};

use bevy::{
    app::Update,
    color::Color,
    prelude::{
        in_state, BuildChildren, ChildBuild, ChildBuilder, Commands, DespawnRecursiveExt,
        IntoSystemConfigs, NextState, OnExit, Res, ResMut, Text,
    },
    text::TextColor,
    ui::{
        BorderColor, BorderRadius, FlexDirection, JustifyContent, Node, Overflow, PositionType,
        UiRect, Val,
    },
};

use crate::{
    scenes::{
        days::{build_content, build_header},
        resources::{FontHandles, GenericDay},
        states::{InputState, Part, UiState, VisualizationState},
    },
    scroll_controls::{ui::build_vertical_scroll_buttons, ScrollWindow, BUTTON_BACKGROUND_COLOR},
};

use super::input::Input;

const SCROLL_SPEED: f32 = 512.;
const VAL_GRID_WIDTH: f32 = 130.;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui
                .run_if(in_state(Part::Part2))
                .run_if(in_state(VisualizationState::<1>::WaitingUi)),
        )
        .add_systems(OnExit(Part::Part2), destroy_ui.before(super::destroy_day_1))
        .add_systems(
            Update,
            super::process_input
                .run_if(in_state(Part::Part2))
                .run_if(in_state(VisualizationState::<1>::WaitingInput)),
        );
    }
}

fn build_ui(
    mut commands: Commands,
    day1_resource: Res<GenericDay>,
    input: Res<Input>,
    mut next_state: ResMut<NextState<UiState>>,
    fonts: Res<FontHandles>,
) {
    bevy::log::trace!("Day 1 Part 2");
    let header = build_header(&mut commands, "day1", true, fonts.font.clone());
    let content = build_content(&mut commands, "day1");

    commands
        .entity(content)
        .with_children(|parent| build_visualization(parent, &input, &fonts));

    commands
        .entity(day1_resource.ui)
        .add_children(&[header, content]);

    next_state.set(UiState::Loaded);
}

fn destroy_ui(
    mut commands: Commands,
    day1_resource: Res<GenericDay>,
    mut input_state: ResMut<NextState<InputState>>,
    mut ui_state: ResMut<NextState<UiState>>,
) {
    commands.remove_resource::<Input>();
    commands.entity(day1_resource.ui).despawn_descendants();

    input_state.set(InputState::NotLoaded);
    ui_state.set(UiState::NotLoaded);
}

fn build_visualization(parent: &mut ChildBuilder, input: &Input, fonts: &FontHandles) {
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

                    build_vertical_scroll_buttons(
                        parent,
                        window,
                        SCROLL_SPEED,
                        BUTTON_BACKGROUND_COLOR,
                        fonts.symbol1.clone(),
                    );
                });
        });
}
