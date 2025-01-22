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
        BackgroundColor, BorderColor, BorderRadius, FlexDirection, FlexWrap, Node, Overflow,
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
    tools::Convolution,
};

use super::input::Input;

const SCROLL_SPEED: f32 = 512.;
const BLOCK_SIZE: f32 = 16.;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui
                .run_if(in_state(Part::Part1).and(in_state(VisualizationState::<12>::WaitingUi))),
        );
    }
}

fn build_ui(
    mut commands: Commands,
    day12_resource: Res<GenericDay>,
    input: Res<Input>,
    mut next_state: ResMut<NextState<UiState>>,
    fonts: Res<FontHandles>,
) {
    bevy::log::trace!("Day 12 Part 1");
    let header = build_header(&mut commands, "day12", true, fonts.font.clone());
    let content = build_content(&mut commands, "day12");

    commands
        .entity(content)
        .with_children(|parent| build_visualization(parent, &input, &fonts));

    commands
        .entity(day12_resource.ui)
        .despawn_descendants()
        .add_children(&[header, content]);

    next_state.set(UiState::Loaded);
}

fn build_visualization(parent: &mut ChildBuilder, input: &Input, fonts: &FontHandles) {
    let slices = input.tiles.iter().map(AsRef::as_ref).collect::<Vec<_>>();
    let mut convolution = Convolution::new(slices.as_slice());
    let tile_influence = input
        .tiles
        .iter()
        .map(|row| {
            row.iter()
                .map(|crop| {
                    let (window, _, _) = convolution.next().unwrap();
                    let fence_pieces = [
                        window[0][1] != *crop,
                        window[1][0] != *crop,
                        window[1][2] != *crop,
                        window[2][1] != *crop,
                    ]
                    .into_iter()
                    .filter(Clone::clone)
                    .count();
                    (crop, fence_pieces)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut colors = BTreeMap::new();

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
                        .with_child((Text::new("Fences"), TextColor(Color::WHITE)));
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
                        .with_child((Text::new(input.fences.to_string()), TextColor(Color::WHITE)));
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
                    for row in tile_influence {
                        parent
                            .spawn(Node {
                                flex_direction: FlexDirection::Row,
                                flex_wrap: FlexWrap::NoWrap,
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                for (crop, fences) in row {
                                    let color = colors.entry(crop).or_insert_with(|| {
                                        Color::hsv(
                                            (((360 / 26) * u32::from(*crop)) % 360) as f32,
                                            1.,
                                            1.,
                                        )
                                    });

                                    let mut field = parent.spawn((
                                        Node {
                                            width: Val::Px(BLOCK_SIZE),
                                            height: Val::Px(BLOCK_SIZE),
                                            ..Default::default()
                                        },
                                        BackgroundColor(*color),
                                    ));

                                    if fences != 0 {
                                        field.with_child((
                                            Text::new(fences.to_string()),
                                            TextColor(Color::BLACK),
                                            TextFont {
                                                font_size: 12.,
                                                ..Default::default()
                                            },
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
