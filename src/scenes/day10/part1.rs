use std::collections::BTreeMap;

use bevy::{
    app::Update,
    color::Color,
    prelude::{
        in_state, BuildChildren, ChildBuild, ChildBuilder, Commands, Condition,
        DespawnRecursiveExt, IntoSystemConfigs, NextState, Res, ResMut, Text,
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
        resources::{FontHandles, GenericDay},
        states::{Part, UiState, VisualizationState},
    },
    scroll_controls::{ui::build_vertical_scroll_buttons, ScrollWindow, BUTTON_BACKGROUND_COLOR},
};

use super::{
    components::{PartOfTrail, Start},
    input::Input,
};

const SCROLL_SPEED: f32 = 512.;
const BLOCK_SIZE: f32 = 16.;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui
                .run_if(in_state(Part::Part1).and(in_state(VisualizationState::<10>::WaitingUi))),
        );
    }
}

fn build_ui(
    mut commands: Commands,
    day10_resource: Res<GenericDay>,
    input: Res<Input>,
    mut next_state: ResMut<NextState<UiState>>,
    fonts: Res<FontHandles>,
) {
    bevy::log::trace!("Day 10 Part 1");
    let header = build_header(&mut commands, "day10", true, fonts.font.clone());
    let content = build_content(&mut commands, "day10");

    commands
        .entity(content)
        .with_children(|parent| build_visualization(parent, &input, &fonts));

    commands
        .entity(day10_resource.ui)
        .despawn_descendants()
        .add_children(&[header, content]);

    next_state.set(UiState::Loaded);
}

fn build_visualization(parent: &mut ChildBuilder, input: &Input, fonts: &FontHandles) {
    let mut trails = BTreeMap::new();
    let mut part_of_trail = BTreeMap::new();

    for (trail_head, trails_from_this_trail_head) in input.trails.iter() {
        for trail in trails_from_this_trail_head {
            let start_end = (trail_head, trail[8]);
            if let std::collections::btree_map::Entry::Vacant(e) = trails.entry(start_end) {
                e.insert(trail.clone());
                for (i, path) in trail.iter().enumerate() {
                    part_of_trail
                        .entry(path)
                        .and_modify(|path: &mut PartOfTrail| {
                            path.starts.push(*trail_head);
                        })
                        .or_insert_with(|| PartOfTrail {
                            coord: *path,
                            starts: vec![*trail_head],
                            is_end: i == 8,
                        });
                }
            }
        }
    }

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
                        .with_child((Text::new("Trails score"), TextColor(Color::WHITE)));
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
                        .with_child((Text::new(trails.len().to_string()), TextColor(Color::WHITE)));
                });

            let window = parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(1.),
                        column_gap: Val::Px(1.),
                        overflow: Overflow::scroll_y(),
                        ..Default::default()
                    },
                    ScrollWindow,
                ))
                .with_children(|parent| {
                    for (y, line) in input.tiles.iter().enumerate() {
                        parent
                            .spawn(Node {
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(1.),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                for (x, tile) in line.iter().enumerate() {
                                    let mut start = parent.spawn((
                                        Node {
                                            width: Val::Px(BLOCK_SIZE),
                                            height: Val::Px(BLOCK_SIZE),
                                            flex_direction: FlexDirection::Row,
                                            justify_content: JustifyContent::Center,
                                            ..Default::default()
                                        },
                                        BackgroundColor(
                                            Color::hsv(
                                                180.,
                                                0.1 + (0.8 * ((tile + 1) as f32 / 10.)),
                                                1.,
                                            )
                                            .to_linear()
                                            .into(),
                                        ),
                                    ));

                                    if *tile == 0 {
                                        start.insert(Start((x, y)));
                                        start.with_child((
                                            Node {
                                                align_self: bevy::ui::AlignSelf::Center,
                                                justify_self: bevy::ui::JustifySelf::Center,
                                                ..Default::default()
                                            },
                                            Text::new("S"),
                                            TextColor(Color::BLACK),
                                            TextFont {
                                                font_size: 12.,
                                                ..Default::default()
                                            },
                                        ));
                                    } else if let Some(part_of_trail) = part_of_trail.get(&(x, y)) {
                                        start.insert(part_of_trail.clone());
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
