use std::time::Duration;

use bevy::{
    app::Update,
    color::Color,
    prelude::{
        in_state, BuildChildren, ChildBuild, ChildBuilder, Commands, Condition,
        DespawnRecursiveExt, Entity, IntoSystemConfigs, NextState, Res, ResMut, Single, Text,
    },
    time::common_conditions::on_timer,
    ui::{
        BorderColor, BorderRadius, Display, FlexDirection, GridPlacement, Node, PositionType,
        UiRect, Val,
    },
};

use crate::scenes::{
    days::{build_content, build_header},
    resources::{FontHandles, GenericDay},
    states::{Part, UiState, VisualizationState},
};

use super::input::{Debugger, Input};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui
                .run_if(in_state(Part::Part2).and(in_state(VisualizationState::<17>::WaitingUi))),
        )
        .add_systems(
            Update,
            update_debugger.run_if(
                in_state(Part::Part2)
                    .and(in_state(VisualizationState::<17>::Ready))
                    .and(on_timer(Duration::from_millis(200))),
            ),
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
    bevy::log::trace!("Day 17 Part 2");
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
    let result = {
        let mut debug = input.debug();
        while debug.i > 0 {
            debug.step();
        }
        debug.queue.iter().min().copied().unwrap()
    };
    let debugger = input.debug();

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
                        .with_child(Text::new("Debugger result"));
                    parent
                        .spawn((
                            Node {
                                border: UiRect::all(Val::Px(3.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_child(Text::new(result.to_string()));
                });

            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                debugger,
            ));
        });
}

fn update_debugger(mut commands: Commands, mut debugger: Single<(Entity, &mut Debugger)>) {
    if debugger.1.i != 0 {
        debugger.1.step();
        while debugger.1.j != 0 {
            debugger.1.step();
        }
    }

    commands
        .entity(debugger.0)
        .despawn_descendants()
        .with_children(|parent| {
            parent
                .spawn(Node {
                    display: Display::Grid,
                    ..Default::default()
                })
                .with_children(|grid| {
                    let mut step_debugger = debugger.1.vm.clone();
                    step_debugger.registers.a = debugger.1.a;
                    let output = step_debugger.execute();

                    grid.spawn((
                        Node {
                            border: UiRect::all(Val::Px(1.)),
                            padding: UiRect::all(Val::Px(5.)),
                            grid_row: GridPlacement::start(1),
                            ..Default::default()
                        },
                        BorderColor(Color::WHITE),
                    ))
                    .with_child(Text::new("A"));
                    grid.spawn((
                        Node {
                            border: UiRect::all(Val::Px(1.)),
                            padding: UiRect::all(Val::Px(5.)),
                            grid_row: GridPlacement::start(1),
                            ..Default::default()
                        },
                        BorderColor(Color::WHITE),
                    ))
                    .with_child(Text::new(debugger.1.a.to_string()));
                    grid.spawn((
                        Node {
                            border: UiRect::all(Val::Px(1.)),
                            padding: UiRect::all(Val::Px(5.)),
                            grid_row: GridPlacement::start(2),
                            ..Default::default()
                        },
                        BorderColor(Color::WHITE),
                    ))
                    .with_child(Text::new("Output"));
                    grid.spawn((
                        Node {
                            border: UiRect::all(Val::Px(1.)),
                            padding: UiRect::all(Val::Px(5.)),
                            grid_row: GridPlacement::start(2),
                            ..Default::default()
                        },
                        BorderColor(Color::WHITE),
                    ))
                    .with_child(Text::new(
                        output
                            .iter()
                            .map(u8::to_string)
                            .collect::<Vec<_>>()
                            .join(","),
                    ));
                    grid.spawn((
                        Node {
                            border: UiRect::all(Val::Px(1.)),
                            padding: UiRect::all(Val::Px(5.)),
                            grid_row: GridPlacement::start(3),
                            ..Default::default()
                        },
                        BorderColor(Color::WHITE),
                    ))
                    .with_child(Text::new("Original program"));
                    grid.spawn((
                        Node {
                            border: UiRect::all(Val::Px(1.)),
                            padding: UiRect::all(Val::Px(5.)),
                            grid_row: GridPlacement::start(3),
                            ..Default::default()
                        },
                        BorderColor(Color::WHITE),
                    ))
                    .with_child(Text::new(
                        debugger
                            .1
                            .vm
                            .raw_program
                            .iter()
                            .map(u8::to_string)
                            .collect::<Vec<_>>()
                            .join(","),
                    ));
                });
        });
}
