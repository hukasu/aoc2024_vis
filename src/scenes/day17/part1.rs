use std::time::Duration;

use bevy::{
    app::Update,
    color::{palettes, Color},
    prelude::{
        in_state, BuildChildren, ChildBuild, ChildBuilder, Commands, Component, Condition,
        DespawnRecursiveExt, Entity, IntoSystemConfigs, NextState, Res, ResMut, Single, Text, With,
    },
    text::{TextColor, TextFont},
    time::common_conditions::on_timer,
    ui::{
        BorderColor, BorderRadius, Display, FlexDirection, JustifyContent, Node, PositionType,
        UiRect, Val,
    },
};

use crate::scenes::{
    days::{build_content, build_header},
    resources::{FontHandles, GenericDay},
    states::{Part, UiState, VisualizationState},
};

use super::input::Input;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui
                .run_if(in_state(Part::Part1).and(in_state(VisualizationState::<17>::WaitingUi))),
        )
        .add_systems(
            Update,
            (
                update_blinking_cursor.run_if(on_timer(Duration::from_secs(1))),
                update_screen.run_if(on_timer(Duration::from_millis(200))),
            )
                .run_if(in_state(Part::Part1).and(in_state(VisualizationState::<17>::Ready))),
        );
    }
}

fn build_ui(
    mut commands: Commands,
    day17_resource: Res<GenericDay>,
    mut input: ResMut<Input>,
    mut next_state: ResMut<NextState<UiState>>,
    fonts: Res<FontHandles>,
) {
    bevy::log::trace!("Day 17 Part 1");
    let header = build_header(&mut commands, "day17", true, fonts.font.clone());
    let content = build_content(&mut commands, "day17");

    commands
        .entity(content)
        .with_children(|parent| build_visualization(parent, &mut input));

    commands
        .entity(day17_resource.ui)
        .despawn_descendants()
        .add_children(&[header, content]);

    next_state.set(UiState::Loaded);
}

fn build_visualization(parent: &mut ChildBuilder, input: &mut Input) {
    let output = input
        .clone()
        .execute()
        .iter()
        .map(u8::to_string)
        .collect::<Vec<_>>();

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
                    column_gap: Val::Px(5.),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(Node::default())
                        .with_child(Text::new("Output"));
                    parent
                        .spawn((
                            Node {
                                border: UiRect::all(Val::Px(3.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_child(Text::new(output.join(",")));
                });

            parent
                .spawn(Node {
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    align_self: bevy::ui::AlignSelf::Center,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                height: Val::Percent(100.),
                                flex_direction: FlexDirection::Column,
                                flex_wrap: bevy::ui::FlexWrap::NoWrap,
                                aspect_ratio: Some(16. / 9.),
                                border: UiRect::all(Val::Px(3.)),
                                padding: UiRect::all(Val::Px(5.)),
                                row_gap: Val::Px(5.),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_children(|screen| {
                            screen.spawn((
                                Node {
                                    flex_direction: FlexDirection::Row,
                                    height: Val::Percent(100.),
                                    column_gap: Val::Px(5.),
                                    ..Default::default()
                                },
                                input.clone(),
                            ));

                            screen
                                .spawn((
                                    Node {
                                        flex_direction: FlexDirection::Row,
                                        height: Val::Px(32.),
                                        border: UiRect::all(Val::Px(2.)),
                                        padding: UiRect::horizontal(Val::Px(5.)),
                                        ..Default::default()
                                    },
                                    BorderColor(Color::WHITE),
                                    BorderRadius::all(Val::Px(3.)),
                                ))
                                .with_child((Text::new("_"), BlinkingCursor));
                        });
                });
        });
}

fn update_screen(
    mut commands: Commands,
    mut screen: Single<(Entity, &mut Input)>,
    fonts: Res<FontHandles>,
) {
    let entity = screen.0;

    let program = &mut screen.1;
    if program.registers.pc < program.program.len() {
        let instruction = program.program[program.registers.pc];
        instruction.execute(&mut program.registers);
    }

    commands
        .entity(entity)
        .despawn_descendants()
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(100.),
                        border: UiRect::all(Val::Px(3.)),
                        ..Default::default()
                    },
                    BorderColor(Color::WHITE),
                    BorderRadius::all(Val::Px(3.)),
                ))
                .with_children(|instructions| {
                    for (i, instruction) in program.program.iter().enumerate() {
                        instructions
                            .spawn(Node {
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(5.),
                                ..Default::default()
                            })
                            .with_children(|instruction_node| {
                                instruction_node
                                    .spawn(Node {
                                        width: Val::Px(16.),
                                        ..Default::default()
                                    })
                                    .with_children(|breakpoint| {
                                        if i == 0 {
                                            breakpoint
                                                .spawn(Node {
                                                    position_type: PositionType::Absolute,
                                                    width: Val::Px(32.),
                                                    justify_content: JustifyContent::Center,
                                                    ..Default::default()
                                                })
                                                .with_child((
                                                    Text::new("•"),
                                                    TextColor(palettes::tailwind::RED_900.into()),
                                                    TextFont {
                                                        font: fonts.symbol1.clone(),
                                                        ..Default::default()
                                                    },
                                                ));
                                        }
                                        if i == program.registers.pc {
                                            breakpoint
                                                .spawn(Node {
                                                    position_type: PositionType::Absolute,
                                                    width: Val::Px(32.),
                                                    justify_content: JustifyContent::Center,
                                                    ..Default::default()
                                                })
                                                .with_child((
                                                    Text::new("→"),
                                                    TextColor(palettes::tailwind::GREEN_500.into()),
                                                    TextFont {
                                                        font: fonts.symbol1.clone(),
                                                        ..Default::default()
                                                    },
                                                ));
                                        }
                                    });

                                instruction_node
                                    .spawn(Node {
                                        padding: UiRect::horizontal(Val::Px(16.)),
                                        ..Default::default()
                                    })
                                    .with_child(Text::new(instruction.to_string()));
                            });
                    }
                });

            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(2.),
                        width: Val::Px(200.),
                        border: UiRect::all(Val::Px(3.)),
                        ..Default::default()
                    },
                    BorderColor(Color::WHITE),
                    BorderRadius::all(Val::Px(3.)),
                ))
                .with_children(|registers| {
                    registers
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(5.),
                                border: UiRect::bottom(Val::Px(2.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                        ))
                        .with_children(|pc| {
                            pc.spawn((
                                Node {
                                    border: UiRect::right(Val::Px(2.)),
                                    ..Default::default()
                                },
                                BorderColor(Color::WHITE),
                            ))
                            .with_child(Text::new("PC"));
                            pc.spawn(Node::default())
                                .with_child(Text::new(program.registers.pc.to_string()));
                        });

                    registers
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Row,
                                border: UiRect::bottom(Val::Px(2.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                        ))
                        .with_children(|a_reg| {
                            a_reg
                                .spawn((
                                    Node {
                                        border: UiRect::right(Val::Px(2.)),
                                        ..Default::default()
                                    },
                                    BorderColor(Color::WHITE),
                                ))
                                .with_child(Text::new("A "));
                            a_reg
                                .spawn(Node::default())
                                .with_child(Text::new(program.registers.a.to_string()));
                        });

                    registers
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Row,
                                border: UiRect::bottom(Val::Px(2.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                        ))
                        .with_children(|b_reg| {
                            b_reg
                                .spawn((
                                    Node {
                                        border: UiRect::right(Val::Px(2.)),
                                        ..Default::default()
                                    },
                                    BorderColor(Color::WHITE),
                                ))
                                .with_child(Text::new("B "));
                            b_reg
                                .spawn(Node::default())
                                .with_child(Text::new(program.registers.b.to_string()));
                        });

                    registers
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Row,
                                border: UiRect::bottom(Val::Px(2.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                        ))
                        .with_children(|c_reg| {
                            c_reg
                                .spawn((
                                    Node {
                                        border: UiRect::right(Val::Px(2.)),
                                        ..Default::default()
                                    },
                                    BorderColor(Color::WHITE),
                                ))
                                .with_child(Text::new("C "));
                            c_reg
                                .spawn(Node::default())
                                .with_child(Text::new(program.registers.c.to_string()));
                        });
                });
        });
}

#[derive(Debug, Component)]
struct BlinkingCursor;

fn update_blinking_cursor(mut cursor: Single<&mut Node, With<BlinkingCursor>>) {
    cursor.display = if cursor.display == Display::Flex {
        Display::None
    } else {
        Display::Flex
    };
}
