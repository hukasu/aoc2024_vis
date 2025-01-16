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
        resources::GenericDay,
        states::{InputState, Part, UiState, VisualizationState},
        FONT_SYMBOLS_HANDLE,
    },
    scroll_controls::{ui::build_vertical_scroll_buttons, ScrollWindow, BUTTON_BACKGROUND_COLOR},
};

use super::input::Input;

const SCROLL_SPEED: f32 = 512.;
const VAL_GRID_WIDTH: f32 = 80.;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui
                .run_if(in_state(Part::Part1))
                .run_if(in_state(VisualizationState::<1>::WaitingUi)),
        )
        .add_systems(OnExit(Part::Part1), destroy_ui.before(super::destroy_day_1))
        .add_systems(
            Update,
            super::process_input
                .run_if(in_state(Part::Part1))
                .run_if(in_state(VisualizationState::<1>::WaitingInput)),
        );
    }
}

fn build_ui(
    mut commands: Commands,
    day1_resource: Res<GenericDay>,
    input: Res<Input>,
    mut next_state: ResMut<NextState<UiState>>,
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

                    build_vertical_scroll_buttons(
                        parent,
                        window,
                        SCROLL_SPEED,
                        BUTTON_BACKGROUND_COLOR,
                        FONT_SYMBOLS_HANDLE.get().unwrap().clone(),
                    );
                });
        });
}
