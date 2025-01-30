use bevy::{
    app::Update,
    color::{palettes, Color},
    prelude::{
        in_state, BuildChildren, ChildBuild, ChildBuilder, Commands, DespawnRecursiveExt,
        IntoSystemConfigs, NextState, Res, ResMut, Text,
    },
    text::{TextColor, TextFont},
    ui::{BorderColor, BorderRadius, FlexDirection, Node, Overflow, PositionType, UiRect, Val},
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
                .run_if(in_state(VisualizationState::<3>::WaitingUi)),
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
    bevy::log::trace!("Day 3 Part 1");
    let header = build_header(&mut commands, "day3", true, fonts.font.clone());
    let content = build_content(&mut commands, "day3");

    commands
        .entity(content)
        .with_children(|parent| build_visualization(parent, &input, &fonts));

    commands
        .entity(day1_resource.ui)
        .despawn_descendants()
        .add_children(&[header, content]);

    next_state.set(UiState::Loaded);
}

fn build_visualization(parent: &mut ChildBuilder, input: &Input, fonts: &FontHandles) {
    let res: u32 = input.segments.iter().map(|segment| segment.mul).sum();
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
                        .with_child((Text::new(res.to_string()), TextColor(Color::WHITE)));
                });

            let window = parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        flex_wrap: bevy::ui::FlexWrap::Wrap,
                        overflow: Overflow::scroll_y(),
                        ..Default::default()
                    },
                    ScrollWindow,
                ))
                .with_children(|parent| {
                    for segment in input.segments.iter() {
                        let color = if segment.mul != 0 {
                            Color::WHITE
                        } else {
                            palettes::basic::GRAY.into()
                        };

                        let segment_text =
                            &input.input[segment.start..(segment.start + segment.len)];

                        parent.spawn(Node::default()).with_child((
                            Text::new(segment_text.replace("\n", " ")),
                            TextColor(color),
                            TextFont {
                                font: fonts.font.clone(),
                                ..Default::default()
                            },
                        ));
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
