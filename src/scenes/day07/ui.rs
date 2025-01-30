use bevy::{
    app::Update,
    color::{palettes, Color},
    prelude::{
        in_state, BuildChildren, ChildBuild, ChildBuilder, Commands, DespawnRecursiveExt,
        IntoSystemConfigs, NextState, Res, ResMut, Text,
    },
    text::TextColor,
    ui::{BorderColor, BorderRadius, FlexDirection, Node, Overflow, PositionType, UiRect, Val},
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

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui.run_if(in_state(VisualizationState::<7>::WaitingUi)),
        );
    }
}

fn build_ui(
    mut commands: Commands,
    day7_resource: Res<GenericDay>,
    input: Res<Input>,
    mut next_state: ResMut<NextState<UiState>>,
    fonts: Res<FontHandles>,
) {
    bevy::log::trace!("Day 7");
    let header = build_header(&mut commands, "day7", false, fonts.font.clone());
    let content = build_content(&mut commands, "day7");

    commands
        .entity(content)
        .with_children(|parent| build_visualization(parent, &input, &fonts));
    commands
        .entity(day7_resource.ui)
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
                        .with_child((
                            Text::new("Result with two operators"),
                            TextColor(Color::WHITE),
                        ));
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
                            Text::new(input.two_ops.to_string()),
                            TextColor(Color::WHITE),
                        ));
                    parent
                        .spawn(Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        })
                        .with_child((
                            Text::new("Result with three operators"),
                            TextColor(Color::WHITE),
                        ));
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
                            Text::new(input.three_ops.to_string()),
                            TextColor(Color::WHITE),
                        ));
                });

            let window = parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(4.),
                        overflow: Overflow::scroll_y(),
                        ..Default::default()
                    },
                    ScrollWindow,
                ))
                .with_children(|parent| {
                    for operation in input.operations.iter() {
                        let border_color = if operation.operators.is_empty() {
                            palettes::basic::RED.into()
                        } else {
                            Color::WHITE
                        };

                        parent
                            .spawn((
                                Node {
                                    flex_direction: FlexDirection::Row,
                                    aspect_ratio: Some(1.),
                                    border: UiRect::all(Val::Px(3.)),
                                    column_gap: Val::Px(8.),
                                    ..Default::default()
                                },
                                BorderColor(border_color),
                                BorderRadius::all(Val::Px(5.)),
                            ))
                            .with_children(|parent| {
                                parent
                                    .spawn((
                                        Node {
                                            min_width: Val::Px(192.),
                                            border: UiRect::right(Val::Px(3.)),
                                            ..Default::default()
                                        },
                                        BorderColor(border_color),
                                    ))
                                    .with_child((
                                        Text::new(operation.result.to_string()),
                                        TextColor(Color::WHITE),
                                    ));

                                let mut operators_iter = operation.operators.iter();
                                for operand in operation.operands.iter() {
                                    parent.spawn(Node::default()).with_child((
                                        Text::new(operand.to_string()),
                                        TextColor(Color::WHITE),
                                    ));

                                    if let Some(operator) = operators_iter.next() {
                                        parent.spawn(Node::default()).with_child((
                                            Text::new(*operator),
                                            TextColor(Color::WHITE),
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
