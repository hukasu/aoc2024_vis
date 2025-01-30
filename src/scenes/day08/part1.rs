use std::collections::BTreeMap;

use bevy::{
    app::Update,
    color::{palettes, Color, Luminance},
    prelude::{
        in_state, BuildChildren, ChildBuild, ChildBuilder, Commands, DespawnRecursiveExt,
        IntoSystemConfigs, NextState, Res, ResMut, Text,
    },
    text::TextColor,
    ui::{
        BackgroundColor, BorderColor, BorderRadius, FlexDirection, Node, Overflow, PositionType,
        UiRect, Val,
    },
};

use crate::{
    scenes::{
        days::{build_content, build_header},
        resources::{FontHandles, GenericDay},
        states::{Part, UiState, VisualizationState},
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
                .run_if(in_state(VisualizationState::<8>::WaitingUi)),
        );
    }
}

fn build_ui(
    mut commands: Commands,
    day8_resource: Res<GenericDay>,
    input: Res<Input>,
    mut next_state: ResMut<NextState<UiState>>,
    fonts: Res<FontHandles>,
) {
    bevy::log::trace!("Day 8 Part 1");
    let header = build_header(&mut commands, "day8", true, fonts.font.clone());
    let content = build_content(&mut commands, "day8");

    commands
        .entity(content)
        .with_children(|parent| build_visualization(parent, &input, &fonts));
    commands
        .entity(day8_resource.ui)
        .despawn_descendants()
        .add_children(&[header, content]);

    next_state.set(UiState::Loaded);
}

fn build_visualization(parent: &mut ChildBuilder, input: &Input, fonts: &FontHandles) {
    let nodes = input
        .slopes
        .iter()
        .flat_map(|slope| {
            [
                slope.interpolate(-1).map(|node| (node, slope.color)),
                slope.interpolate(2).map(|node| (node, slope.color)),
            ]
        })
        .flatten()
        .collect::<BTreeMap<_, _>>();

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
                    column_gap: Val::Px(10.),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        })
                        .with_child((Text::new("Nodes"), TextColor(Color::WHITE)));
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
                        .with_child((Text::new(nodes.len().to_string()), TextColor(Color::WHITE)));
                });

            let window = parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(1.),
                        overflow: Overflow::scroll_y(),
                        ..Default::default()
                    },
                    ScrollWindow,
                ))
                .with_children(|parent| {
                    for y in 0..input.bounds.1 {
                        parent
                            .spawn(Node {
                                flex_direction: FlexDirection::Row,
                                aspect_ratio: Some(1.),
                                column_gap: Val::Px(1.),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                for x in 0..input.bounds.0 {
                                    let color =
                                        input.antennas.get(&(x, y)).copied().unwrap_or_else(|| {
                                            palettes::tailwind::YELLOW_200.into()
                                        });

                                    let mut tile = parent.spawn((
                                        Node {
                                            width: Val::Px(16.),
                                            height: Val::Px(16.),
                                            flex_direction: FlexDirection::Column,
                                            justify_content: bevy::ui::JustifyContent::SpaceEvenly,
                                            ..Default::default()
                                        },
                                        BackgroundColor(color),
                                    ));

                                    if let Some(node_color) = nodes.get(&(x, y)) {
                                        tile.with_child((
                                            Node {
                                                width: Val::Px(8.),
                                                height: Val::Px(8.),
                                                align_self: bevy::ui::AlignSelf::Center,
                                                ..Default::default()
                                            },
                                            BackgroundColor(node_color.lighter(0.125)),
                                        ));
                                    }
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
}
