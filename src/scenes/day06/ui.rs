use bevy::{
    app::Update,
    color::{palettes, Alpha, Color, Srgba},
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
        states::{UiState, VisualizationState},
    },
    scroll_controls::{ui::build_vertical_scroll_buttons, ScrollWindow, BUTTON_BACKGROUND_COLOR},
};

use super::input::Input;

const SCROLL_SPEED: f32 = 512.;
const STARTING_COLOR: Srgba = palettes::tailwind::GREEN_950;
const GROUND_COLOR: Srgba = palettes::tailwind::YELLOW_100;
const PATH_COLOR: Srgba = palettes::tailwind::GREEN_100;
const BOX_COLOR: Srgba = palettes::tailwind::RED_700;
const POSSIBLE_BOX_COLOR: Srgba = palettes::tailwind::RED_700;
const TILE_DIMENSION: f32 = 12.;
const BOX_DIMENSION: f32 = 8.;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui.run_if(in_state(VisualizationState::<6>::WaitingUi)),
        );
    }
}

fn build_ui(
    mut commands: Commands,
    day6_resource: Res<GenericDay>,
    input: Res<Input>,
    mut next_state: ResMut<NextState<UiState>>,
    fonts: Res<FontHandles>,
) {
    bevy::log::trace!("Day 6");
    let header = build_header(&mut commands, "day6", false, fonts.font.clone());
    let content = build_content(&mut commands, "day6");

    commands
        .entity(content)
        .with_children(|parent| build_visualization(parent, &input, &fonts));
    commands
        .entity(day6_resource.ui)
        .despawn_descendants()
        .add_children(&[header, content]);

    next_state.set(UiState::Loaded);
}

fn build_visualization(parent: &mut ChildBuilder, input: &Input, fonts: &FontHandles) {
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
                        .with_child((Text::new("Path length"), TextColor(Color::WHITE)));
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
                            Text::new(input.paths.len().to_string()),
                            TextColor(Color::WHITE),
                        ));
                    parent
                        .spawn(Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        })
                        .with_child((Text::new("Possible loops"), TextColor(Color::WHITE)));
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
                            Text::new(input.possible_obstacles.len().to_string()),
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
                    for (y, line) in input.lines.iter().enumerate() {
                        parent
                            .spawn(Node {
                                flex_direction: FlexDirection::Row,
                                aspect_ratio: Some(1.),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                for (x, c) in line.iter().enumerate() {
                                    let tile_color = if *c == b'^' {
                                        STARTING_COLOR
                                    } else if input.paths.contains(&(x, y)) {
                                        PATH_COLOR
                                    } else {
                                        GROUND_COLOR
                                    };

                                    let mut tile = parent.spawn((
                                        Node {
                                            width: Val::Px(TILE_DIMENSION),
                                            height: Val::Px(TILE_DIMENSION),
                                            aspect_ratio: Some(1.),
                                            ..Default::default()
                                        },
                                        BackgroundColor(tile_color.into()),
                                    ));

                                    if *c == b'#' {
                                        tile.with_child((
                                            Node {
                                                width: Val::Px(BOX_DIMENSION),
                                                height: Val::Px(BOX_DIMENSION),
                                                aspect_ratio: Some(1.),
                                                ..Default::default()
                                            },
                                            BackgroundColor(BOX_COLOR.into()),
                                        ));
                                    } else if input.possible_obstacles.contains(&(x, y)) {
                                        tile.with_child((
                                            Node {
                                                width: Val::Px(BOX_DIMENSION),
                                                height: Val::Px(BOX_DIMENSION),
                                                aspect_ratio: Some(1.),
                                                ..Default::default()
                                            },
                                            BackgroundColor(
                                                POSSIBLE_BOX_COLOR.with_alpha(0.25).into(),
                                            ),
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
