use bevy::{
    app::Update,
    color::{palettes, Color},
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
        resources::GenericDay,
        states::{InputState, Part, UiState, VisualizationState},
        FONT_SYMBOLS_HANDLE,
    },
    scroll_controls::{ScrollControl, ScrollWindow, BUTTON_BACKGROUND_COLOR},
};

use super::input::Input;

const SCROLL_SPEED: f32 = 512.;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui
                .run_if(in_state(Part::Part2))
                .run_if(in_state(VisualizationState::<4>::WaitingUi)),
        )
        .add_systems(OnExit(Part::Part2), destroy_ui.before(super::destroy_day_4))
        .add_systems(
            Update,
            super::process_input
                .run_if(in_state(Part::Part2))
                .run_if(in_state(VisualizationState::<4>::WaitingInput)),
        );
    }
}

fn build_ui(
    mut commands: Commands,
    day1_resource: Res<GenericDay>,
    input: Res<Input>,
    mut next_state: ResMut<NextState<UiState>>,
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
    parent
        .spawn(Node {
            top: Val::Px(50.),
            bottom: Val::Px(10.),
            left: Val::Px(10.),
            right: Val::Px(10.),
            flex_direction: FlexDirection::Column,
            position_type: PositionType::Absolute,
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
                        .with_child((Text::new("Result"), TextColor(Color::WHITE)));
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
                            Text::new(input.result_part2.to_string()),
                            TextColor(Color::WHITE),
                        ));
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
                    for (i, line) in input.lines.iter().enumerate() {
                        if line.is_empty() {
                            break;
                        }

                        parent
                            .spawn(Node {
                                flex_direction: FlexDirection::Row,
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                for (j, c) in line.iter().enumerate() {
                                    let color = if input.positions_x_mas.contains(&(j, i)) {
                                        Color::WHITE
                                    } else {
                                        palettes::basic::GRAY.into()
                                    };

                                    parent.spawn(Node::default()).with_child((
                                        Text::new(char::from_u32(u32::from(*c)).unwrap()),
                                        TextColor(color),
                                        TextFont {
                                            font_size: 12.,
                                            ..Default::default()
                                        },
                                    ));
                                }
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
}
