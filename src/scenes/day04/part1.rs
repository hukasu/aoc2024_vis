use bevy::{
    app::Update,
    color::{palettes, Color},
    prelude::{
        in_state, BuildChildren, ChildBuild, ChildBuilder, Commands, DespawnRecursiveExt,
        IntoSystemConfigs, NextState, OnExit, Res, ResMut, Text,
    },
    text::{TextColor, TextFont},
    ui::{BorderColor, BorderRadius, FlexDirection, Node, Overflow, PositionType, UiRect, Val},
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

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui
                .run_if(in_state(Part::Part1))
                .run_if(in_state(VisualizationState::<4>::WaitingUi)),
        )
        .add_systems(OnExit(Part::Part1), destroy_ui.before(super::destroy_day_4))
        .add_systems(
            Update,
            super::process_input
                .run_if(in_state(Part::Part1))
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
    bevy::log::trace!("Day 4 Part 1");
    let header = build_header(&mut commands, "day4", true);
    let content = build_content(&mut commands, "day4");

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
                            Text::new(input.result_part1.to_string()),
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
                                    let color = if input.positions_xmas.contains(&(j, i)) {
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

            build_vertical_scroll_buttons(
                parent,
                window,
                SCROLL_SPEED,
                BUTTON_BACKGROUND_COLOR,
                FONT_SYMBOLS_HANDLE.get().unwrap().clone(),
            );
        });
}
