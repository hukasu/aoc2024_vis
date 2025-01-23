use bevy::{
    app::Update,
    color::Color,
    prelude::{
        in_state, BuildChildren, ChildBuild, ChildBuilder, Commands, Condition,
        DespawnRecursiveExt, IntoSystemConfigs, NextState, Res, ResMut, Text,
    },
    text::TextColor,
    ui::{AlignContent, BorderColor, BorderRadius, FlexDirection, Node, PositionType, UiRect, Val},
};

use crate::scenes::{
    days::{build_content, build_header},
    resources::{FontHandles, GenericDay},
    states::{Part, UiState, VisualizationState},
};

use super::{
    claw_machine::{build_claw_machine_buttons, ClawMachineCanvas},
    input::Input,
};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui
                .run_if(in_state(Part::Part1).and(in_state(VisualizationState::<13>::WaitingUi))),
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
    bevy::log::trace!("Day 13 Part 1");
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
        .flat_map(|machine| machine.find_cheapest_solution(0).map(|res| res.0))
        .sum();

    parent
        .spawn(Node {
            top: Val::Px(50.),
            bottom: Val::Px(10.),
            left: Val::Px(10.),
            right: Val::Px(10.),
            flex_direction: FlexDirection::Column,
            align_content: AlignContent::FlexStart,
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

            build_claw_machine_buttons(parent, input);

            parent.spawn((
                Node {
                    height: Val::Percent(100.),
                    ..Default::default()
                },
                ClawMachineCanvas(0),
            ));
        });
}
