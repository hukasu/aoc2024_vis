use std::collections::BTreeMap;

use bevy::{
    app::Update,
    asset::Handle,
    color::{palettes, Color},
    core::Name,
    prelude::{
        in_state, BuildChildren, Button, Changed, ChildBuild, ChildBuilder, Commands, Condition,
        DespawnRecursiveExt, DetectChanges, Entity, IntoSystemConfigs, NextState, Query, Ref, Res,
        ResMut, Single, Text, With,
    },
    text::{Font, TextColor, TextFont},
    ui::{
        AlignItems, BackgroundColor, BorderColor, FlexDirection, FlexWrap, Interaction,
        JustifyContent, Node, PositionType, UiRect, Val,
    },
};

use crate::scenes::{
    days::{build_content, build_footer, build_header, button_node},
    resources::{FontHandles, GenericDay},
    states::{Part, UiState, VisualizationState},
    BUTTON_BACKGROUND_COLOR, BUTTON_HOVERED_BACKGROUND_COLOR,
};

use super::{
    components::{Controls, Wire},
    input::{ExecutionResult, Input},
};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui
                .run_if(in_state(Part::Part1))
                .run_if(in_state(VisualizationState::<24>::WaitingUi)),
        )
        .add_systems(
            Update,
            controls_interaction
                .run_if(in_state(Part::Part1))
                .run_if(in_state(VisualizationState::<24>::Ready)),
        )
        .add_systems(
            Update,
            update_visualization.run_if(
                in_state(Part::Part1)
                    .and(in_state(VisualizationState::<24>::Ready))
                    .and(update_visualization_condition),
            ),
        );
    }
}

type ControlWithChangedInteractionQuery<'a, 'b> = Query<
    'a,
    'b,
    (
        &'static mut BackgroundColor,
        &'static Interaction,
        &'static Controls,
    ),
    (With<Button>, Changed<Interaction>),
>;

fn controls_interaction(
    mut commands: Commands,
    mut controls: ControlWithChangedInteractionQuery,
    mut input: Single<(Entity, &mut Input)>,
    original_input: Res<Input>,
) {
    for (mut background_color, interaction, control) in controls.iter_mut() {
        match interaction {
            Interaction::None => background_color.0 = BUTTON_BACKGROUND_COLOR,
            Interaction::Hovered => background_color.0 = BUTTON_HOVERED_BACKGROUND_COLOR,
            Interaction::Pressed => match control {
                Controls::Reset => {
                    *input.1 = original_input.clone();
                }
                Controls::Step => {
                    if !input.1.operations.is_empty() {
                        commands.entity(input.0).insert(input.1.execute_top());
                    }
                }
                Controls::FastForward => {
                    input.1.run_program();
                }
            },
        }
    }
}

fn build_ui(
    mut commands: Commands,
    day24_resource: Res<GenericDay>,
    input: Res<Input>,
    mut next_state: ResMut<NextState<UiState>>,
    fonts: Res<FontHandles>,
) {
    let header = build_header(&mut commands, "day24", true, fonts.font.clone());
    let content = build_content(&mut commands, "day24");
    let footer = build_footer(&mut commands, "day24");

    commands
        .entity(content)
        .with_children(|parent| build_visualization(parent, &input));
    commands
        .entity(footer)
        .with_children(|parent| build_control_buttons(parent, fonts.symbol2.clone()));

    commands
        .entity(day24_resource.ui)
        .despawn_descendants()
        .add_children(&[header, content, footer]);

    next_state.set(UiState::Loaded);
}

fn build_visualization(parent: &mut ChildBuilder, input: &Input) {
    parent.spawn((
        Node {
            top: Val::Px(50.),
            bottom: Val::Px(10.),
            left: Val::Px(10.),
            right: Val::Px(10.),
            flex_direction: FlexDirection::Row,
            position_type: PositionType::Absolute,
            row_gap: Val::Px(12.),
            ..Default::default()
        },
        input.clone(),
    ));
}

fn update_visualization_condition(visualization: Single<Ref<Input>>) -> bool {
    visualization.is_changed()
}

fn update_visualization(
    mut commands: Commands,
    visualization: Single<(Entity, &Input, Option<&ExecutionResult>)>,
    fonts: Res<FontHandles>,
) {
    commands
        .entity(visualization.0)
        .despawn_descendants()
        .with_children(|parent| {
            let input = &visualization.1;
            let execution_result = visualization.2.cloned();
            parent
                .spawn((
                    Name::new("day_24_part1_visualization"),
                    Node {
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                ))
                .with_children(|parent| {
                    build_input_row(parent, b'x', &input.x, true, execution_result.clone());
                    build_input_row(parent, b'y', &input.y, true, execution_result.clone());
                    parent.spawn(Node {
                        height: Val::Px(5.),
                        ..Default::default()
                    });
                    build_input_row(parent, b'z', &input.z, false, execution_result.clone());
                    parent.spawn(Node {
                        height: Val::Px(5.),
                        ..Default::default()
                    });
                    build_intermediates(parent, &input.intermediate, execution_result);
                });
            parent
                .spawn((
                    Name::new("day_24_part1_operations"),
                    Node {
                        min_width: Val::Px(48. * 5.),
                        flex_direction: FlexDirection::Column,
                        border: UiRect::all(Val::Px(5.)),
                        ..Default::default()
                    },
                    BorderColor(Color::WHITE),
                ))
                .with_children(|parent| {
                    for operation in &input.operations {
                        parent
                            .spawn(Node {
                                flex_direction: FlexDirection::Row,
                                flex_wrap: FlexWrap::NoWrap,
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                build_operation_wires(parent, &operation.l, fonts.font.clone());
                                build_operation_symbol(
                                    parent,
                                    operation.operator.to_string().as_bytes(),
                                    fonts.font.clone(),
                                );
                                build_operation_wires(parent, &operation.r, fonts.font.clone());
                                build_operation_symbol(
                                    parent,
                                    "→".as_bytes(),
                                    fonts.symbol1.clone(),
                                );
                                build_operation_wires(parent, &operation.out, fonts.font.clone());
                            });
                    }
                });
        });
}

fn build_input_row(
    parent: &mut ChildBuilder,
    title: u8,
    row: &[u8],
    pad: bool,
    execution_result: Option<ExecutionResult>,
) {
    parent
        .spawn((Node {
            width: Val::Percent(100.),
            flex_direction: FlexDirection::Row,
            ..Default::default()
        },))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(40.),
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BackgroundColor(BUTTON_BACKGROUND_COLOR),
                ))
                .with_child((
                    Text::new(String::from_utf8_lossy(&[title])),
                    TextColor::BLACK,
                ));
            if pad {
                parent.spawn((
                    Node {
                        width: Val::Px(16.),
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BackgroundColor(BUTTON_BACKGROUND_COLOR),
                ));
            }
            for (i, val) in row.iter().enumerate().rev() {
                let Ok(i) = u8::try_from(i) else {
                    unreachable!("Will never have more than u8 values");
                };
                let border_color = if let Some(result) = &execution_result {
                    let test_wire = [title, (i / 10) + b'0', (i % 10) + b'0'];
                    match result {
                        ExecutionResult::Success(l, r, out) => {
                            if test_wire == *l || test_wire == *r {
                                palettes::tailwind::GREEN_500.into()
                            } else if test_wire == *out {
                                palettes::tailwind::YELLOW_500.into()
                            } else {
                                BUTTON_BACKGROUND_COLOR
                            }
                        }
                        ExecutionResult::Failure(l, r) => {
                            if test_wire == *l || test_wire == *r {
                                palettes::tailwind::RED_500.into()
                            } else {
                                BUTTON_BACKGROUND_COLOR
                            }
                        }
                    }
                } else {
                    BUTTON_BACKGROUND_COLOR
                };
                parent
                    .spawn((
                        Node {
                            width: Val::Px(16.),
                            border: UiRect::all(Val::Px(2.)),
                            justify_content: JustifyContent::SpaceEvenly,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        BorderColor(border_color),
                        Wire([title, (i / 10) + b'0', (i % 10) + b'0']),
                    ))
                    .with_child((Text::new(val.to_string()), TextColor(Color::WHITE)));
            }
        });
}

fn build_intermediates(
    parent: &mut ChildBuilder,
    intermediates: &BTreeMap<[u8; 3], u8>,
    execution_result: Option<ExecutionResult>,
) {
    parent
        .spawn((Node {
            width: Val::Percent(100.),
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::Wrap,
            ..Default::default()
        },))
        .with_children(|parent| {
            for (key, val) in intermediates.iter().rev() {
                let border_color = if let Some(result) = &execution_result {
                    match result {
                        ExecutionResult::Success(l, r, out) => {
                            if key == l || key == r {
                                palettes::tailwind::GREEN_500.into()
                            } else if key == out {
                                palettes::tailwind::YELLOW_500.into()
                            } else {
                                BUTTON_BACKGROUND_COLOR
                            }
                        }
                        ExecutionResult::Failure(l, r) => {
                            if key == l || key == r {
                                palettes::tailwind::RED_500.into()
                            } else {
                                BUTTON_BACKGROUND_COLOR
                            }
                        }
                    }
                } else {
                    BUTTON_BACKGROUND_COLOR
                };
                parent.spawn(Node::default()).with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                width: Val::Px(40.),
                                justify_content: JustifyContent::SpaceEvenly,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            BackgroundColor(BUTTON_BACKGROUND_COLOR),
                        ))
                        .with_child((Text::new(String::from_utf8_lossy(key)), TextColor::BLACK));
                    let mut value_node = parent.spawn((
                        Node {
                            width: Val::Px(16.),
                            border: UiRect::all(Val::Px(2.)),
                            justify_content: JustifyContent::SpaceEvenly,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        BorderColor(border_color),
                        Wire(*key),
                    ));
                    if *val != u8::MAX {
                        value_node
                            .with_child((Text::new(val.to_string()), TextColor(Color::WHITE)));
                    }
                });
            }
        });
}

fn build_control_buttons(parent: &mut ChildBuilder, symbol: Handle<Font>) {
    parent
        .spawn((
            button_node(),
            Controls::Reset,
            BackgroundColor(BUTTON_BACKGROUND_COLOR),
        ))
        .with_child((
            Text::new("⏮"),
            TextFont {
                font: symbol.clone(),
                ..Default::default()
            },
            TextColor(Color::BLACK),
        ));
    parent
        .spawn((
            button_node(),
            Controls::Step,
            BackgroundColor(BUTTON_BACKGROUND_COLOR),
        ))
        .with_child((
            Text::new("⏵"),
            TextFont {
                font: symbol.clone(),

                ..Default::default()
            },
            TextColor(Color::BLACK),
        ));
    parent
        .spawn((
            button_node(),
            Controls::FastForward,
            BackgroundColor(BUTTON_BACKGROUND_COLOR),
        ))
        .with_child((
            Text::new("⏭"),
            TextFont {
                font: symbol.clone(),
                ..Default::default()
            },
            TextColor(Color::BLACK),
        ));
}

fn build_operation_wires(parent: &mut ChildBuilder, text: &[u8], font: Handle<Font>) {
    parent
        .spawn((
            Node {
                width: Val::Px(48.),
                justify_content: JustifyContent::Center,
                align_content: bevy::ui::AlignContent::Center,
                ..Default::default()
            },
            BackgroundColor(BUTTON_BACKGROUND_COLOR),
        ))
        .with_child((
            Text::new(String::from_utf8_lossy(text)),
            TextFont {
                font,
                ..Default::default()
            },
            TextColor(Color::BLACK),
        ));
}

fn build_operation_symbol(parent: &mut ChildBuilder, text: &[u8], font: Handle<Font>) {
    parent
        .spawn(Node {
            width: Val::Px(48.),
            justify_content: JustifyContent::Center,
            align_content: bevy::ui::AlignContent::Center,
            ..Default::default()
        })
        .with_child((
            Text::new(String::from_utf8_lossy(text)),
            TextFont {
                font,
                ..Default::default()
            },
        ));
}
