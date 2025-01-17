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
        resources::{FontHandles, GenericDay},
        states::{InputState, Part, UiState, VisualizationState},
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
                .run_if(in_state(VisualizationState::<5>::WaitingUi)),
        )
        .add_systems(OnExit(Part::Part1), destroy_ui.before(super::destroy_day_5))
        .add_systems(
            Update,
            super::process_input
                .run_if(in_state(Part::Part1))
                .run_if(in_state(VisualizationState::<5>::WaitingInput)),
        );
    }
}

fn build_ui(
    mut commands: Commands,
    day5_resource: Res<GenericDay>,
    input: Res<Input>,
    mut next_state: ResMut<NextState<UiState>>,
    fonts: Res<FontHandles>,
) {
    bevy::log::trace!("Day 5 Part 1");
    let header = build_header(&mut commands, "day5", true, fonts.font.clone());
    let content = build_content(&mut commands, "day5");

    commands
        .entity(content)
        .with_children(|parent| build_visualization(parent, &input, &fonts));
    commands
        .entity(day5_resource.ui)
        .add_children(&[header, content]);

    next_state.set(UiState::Loaded);
}

fn destroy_ui(
    mut commands: Commands,
    day5_resource: Res<GenericDay>,
    mut input_state: ResMut<NextState<InputState>>,
    mut ui_state: ResMut<NextState<UiState>>,
) {
    commands.remove_resource::<Input>();
    commands.entity(day5_resource.ui).despawn_descendants();

    input_state.set(InputState::NotLoaded);
    ui_state.set(UiState::NotLoaded);
}

fn build_visualization(parent: &mut ChildBuilder, input: &Input, fonts: &FontHandles) {
    let sorted_manuals = input
        .manuals
        .iter()
        .filter(|manual| manual.sorted)
        .collect::<Vec<_>>();
    let result = sorted_manuals
        .iter()
        .map(|manual| manual.pages[manual.pages.len() / 2])
        .sum::<u32>();

    parent
        .spawn(Node {
            top: Val::Px(50.),
            bottom: Val::Px(10.),
            left: Val::Px(10.),
            right: Val::Px(10.),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(12.),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(Node {
                            padding: UiRect::all(Val::Px(5.)),
                            ..Default::default()
                        })
                        .with_child(Text::new("Result"));
                    parent
                        .spawn((
                            Node {
                                padding: UiRect::all(Val::Px(5.)),
                                border: UiRect::all(Val::Px(3.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_child(Text::new(result.to_string()));
                    parent
                        .spawn(Node {
                            padding: UiRect::all(Val::Px(5.)),
                            ..Default::default()
                        })
                        .with_child(Text::new("Sorted manuals"));
                    parent
                        .spawn((
                            Node {
                                padding: UiRect::all(Val::Px(5.)),
                                border: UiRect::all(Val::Px(3.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_child(Text::new((sorted_manuals.len()).to_string()));
                });

            parent.spawn(Node::default()).with_children(|parent| {
                let window = parent
                    .spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            overflow: Overflow::scroll_y(),
                            row_gap: Val::Px(4.),
                            ..Default::default()
                        },
                        ScrollWindow,
                    ))
                    .with_children(|parent| {
                        for manual in sorted_manuals {
                            parent
                                .spawn((
                                    Node {
                                        flex_direction: FlexDirection::Row,
                                        column_gap: Val::Px(8.),
                                        border: UiRect::all(Val::Px(3.)),
                                        ..Default::default()
                                    },
                                    BorderColor(Color::WHITE),
                                    BorderRadius::all(Val::Px(5.)),
                                ))
                                .with_children(|parent| {
                                    let mid = manual.pages.len() / 2;
                                    for (i, page) in manual.pages.iter().enumerate() {
                                        let color = if i == mid {
                                            Color::WHITE
                                        } else {
                                            palettes::tailwind::GRAY_500.into()
                                        };
                                        parent.spawn(Node::default()).with_child((
                                            Text::new(page.to_string()),
                                            TextColor(color),
                                            TextFont {
                                                font: fonts.font.clone(),
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
                    fonts.symbol1.clone(),
                );
            });
        });
}
