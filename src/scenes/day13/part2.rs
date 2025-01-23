use bevy::{
    app::Update,
    color::Color,
    prelude::{
        in_state, BuildChildren, ChildBuild, ChildBuilder, Commands, Condition,
        DespawnRecursiveExt, IntoSystemConfigs, NextState, Res, ResMut, Text,
    },
    text::{TextColor, TextFont},
    ui::{BorderColor, BorderRadius, FlexDirection, FlexWrap, Node, PositionType, UiRect, Val},
};

use crate::scenes::{
    days::{build_content, build_header},
    resources::{FontHandles, GenericDay},
    states::{Part, UiState, VisualizationState},
};

use super::{
    claw_machine::{ClawMachineCanvas, SelectedClawMachine},
    input::Input,
};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui
                .run_if(in_state(Part::Part2).and(in_state(VisualizationState::<13>::WaitingUi))),
        );
    }
}

fn build_ui(
    mut commands: Commands,
    day13_resource: Res<GenericDay>,
    input: Res<Input>,
    mut next_state: ResMut<NextState<UiState>>,
    fonts: Res<FontHandles>,
) {
    bevy::log::trace!("Day 13 Part 2");
    let header = build_header(&mut commands, "day13", true, fonts.font.clone());
    let content = build_content(&mut commands, "day13");

    commands
        .entity(content)
        .with_children(|parent| build_visualization(parent, &input));

    commands
        .entity(day13_resource.ui)
        .despawn_descendants()
        .add_children(&[header, content]);

    next_state.set(UiState::Loaded);
}

fn build_visualization(parent: &mut ChildBuilder, input: &Input) {
    let total_cost: i64 = input
        .machines
        .iter()
        .flat_map(|machine| {
            machine
                .find_cheapest_solution(10000000000000)
                .map(|res| res.0)
        })
        .sum();

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
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            height: Val::Percent(100.),
                            column_gap: Val::Px(10.),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(Node {
                                    padding: UiRect::all(Val::Px(3.)),
                                    ..Default::default()
                                })
                                .with_child((Text::new("Total cost"), TextColor(Color::WHITE)));
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
                                    Text::new(total_cost.to_string()),
                                    TextColor(Color::WHITE),
                                ));
                        });
                });

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::Wrap,
                    column_gap: Val::Px(4.),
                    ..Default::default()
                })
                .with_children(|parent| {
                    for i in 0..input.machines.len() {
                        parent
                            .spawn((
                                Node {
                                    padding: UiRect::all(Val::Px(4.)),
                                    ..Default::default()
                                },
                                SelectedClawMachine(i),
                            ))
                            .with_child((
                                Text::new((i + 1).to_string()),
                                TextColor(Color::BLACK),
                                TextFont {
                                    font_size: 8.,
                                    ..Default::default()
                                },
                            ));
                    }
                });

            parent.spawn((
                Node {
                    height: Val::Percent(100.),
                    ..Default::default()
                },
                ClawMachineCanvas(10000000000000),
            ));
        });
}
