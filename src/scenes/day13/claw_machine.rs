use bevy::{
    app::Update,
    color::{palettes, Color},
    prelude::{
        in_state, resource_changed, resource_exists, state_changed, BuildChildren, Button, Changed,
        ChildBuild, Commands, Component, Condition, DespawnRecursiveExt, Entity, IntoSystemConfigs,
        Query, Res, Resource, Single, Text,
    },
    text::TextColor,
    ui::{
        AlignSelf, BackgroundColor, BorderColor, BorderRadius, FlexDirection, Interaction, Node,
        PositionType, UiRect, Val,
    },
};

use crate::{
    scenes::states::{Part, VisualizationState},
    scroll_controls::{BUTTON_BACKGROUND_COLOR, BUTTON_HOVERED_BACKGROUND_COLOR},
};

use super::input::Input;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            reset_selected
                .run_if(resource_exists::<SelectedClawMachine>.and(state_changed::<Part>)),
        )
        .add_systems(
            Update,
            update_canvas.run_if(
                in_state(VisualizationState::<13>::Ready)
                    .and(resource_changed::<SelectedClawMachine>),
            ),
        )
        .add_systems(
            Update,
            selected_claw_machine_interaction.run_if(in_state(VisualizationState::<13>::Ready)),
        );
    }
}

#[derive(Debug, Component)]
#[require(Node)]
pub struct ClawMachineCanvas(pub i64);

#[derive(Debug, Default, Clone, Copy, Resource, Component)]
#[require(Button)]
pub struct SelectedClawMachine(pub usize);

fn reset_selected(mut commands: Commands) {
    commands.insert_resource(SelectedClawMachine::default());
}

fn update_canvas(
    mut commands: Commands,
    canvas: Single<(Entity, &ClawMachineCanvas)>,
    selected_claw_machine: Res<SelectedClawMachine>,
    input: Res<Input>,
) {
    let claw_machine = &input.machines[selected_claw_machine.0];
    let claw_machine_cost = claw_machine.find_cheapest_solution(canvas.1 .0);

    let bounds = input
        .machines
        .iter()
        .map(|claw_machine| {
            (claw_machine.prize.0 + canvas.1 .0).max(claw_machine.prize.1 + canvas.1 .0)
        })
        .max()
        .unwrap() as f32;

    commands
        .entity(canvas.0)
        .despawn_descendants()
        .with_children(|parent| {
            parent
                .spawn(Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                })
                .with_children(|parent| {
                    let mut header = parent.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(4.),
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    });
                    if let Some(cost) = claw_machine_cost {
                        header.with_children(|parent| {
                            parent
                                .spawn((
                                    Node {
                                        border: UiRect::all(Val::Px(2.)),
                                        ..Default::default()
                                    },
                                    BorderColor(Color::WHITE),
                                    BorderRadius::all(Val::Px(5.)),
                                ))
                                .with_child(Text::new(cost.0.to_string()));
                            parent.spawn(Node::default()).with_child(Text::new("="));
                            parent
                                .spawn((
                                    Node {
                                        border: UiRect::all(Val::Px(2.)),
                                        ..Default::default()
                                    },
                                    BorderColor(Color::WHITE),
                                    BorderRadius::all(Val::Px(5.)),
                                ))
                                .with_child(Text::new(cost.1.to_string()));
                            parent.spawn(Node::default()).with_child(Text::new("x"));
                            parent.spawn(Node::default()).with_child(Text::new("3"));
                            parent.spawn(Node::default()).with_child(Text::new("+"));
                            parent
                                .spawn((
                                    Node {
                                        border: UiRect::all(Val::Px(2.)),
                                        ..Default::default()
                                    },
                                    BorderColor(Color::WHITE),
                                    BorderRadius::all(Val::Px(5.)),
                                ))
                                .with_child(Text::new(cost.2.to_string()));
                        });
                    } else {
                        header.with_child(Text::new("Impossible machine"));
                    }

                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            align_self: AlignSelf::Center,
                            height: Val::Percent(100.),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Node {
                                        aspect_ratio: Some(1.),
                                        height: Val::Percent(100.),
                                        border: UiRect::all(Val::Px(2.)),
                                        ..Default::default()
                                    },
                                    BorderColor(Color::WHITE),
                                    BorderRadius::all(Val::Px(5.)),
                                    BackgroundColor(palettes::tailwind::YELLOW_200.into()),
                                ))
                                .with_children(|parent| {
                                    let claw_position = (20f32, 20f32);
                                    let prize_position = (
                                        (claw_machine.prize.0 + canvas.1 .0 + 20) as f32,
                                        (claw_machine.prize.1 + canvas.1 .0 + 20) as f32,
                                    );

                                    parent
                                        .spawn((
                                            Node {
                                                position_type: PositionType::Absolute,
                                                bottom: Val::Percent(
                                                    100. * (claw_position.1 / bounds),
                                                ),
                                                left: Val::Percent(
                                                    100. * (claw_position.0 / bounds),
                                                ),
                                                border: UiRect::bottom(Val::Px(1.))
                                                    .with_left(Val::Px(1.)),
                                                ..Default::default()
                                            },
                                            BorderColor(Color::BLACK),
                                        ))
                                        .with_child((Text::new("C"), TextColor(Color::BLACK)));
                                    parent
                                        .spawn((
                                            Node {
                                                position_type: PositionType::Absolute,
                                                top: Val::Percent(
                                                    100. * (1. - (prize_position.1 / bounds)),
                                                ),
                                                right: Val::Percent(
                                                    100. * (1. - (prize_position.0 / bounds)),
                                                ),
                                                border: UiRect::top(Val::Px(1.))
                                                    .with_right(Val::Px(1.)),
                                                ..Default::default()
                                            },
                                            BorderColor(Color::BLACK),
                                        ))
                                        .with_child((Text::new("P"), TextColor(Color::BLACK)));
                                });
                        });
                });
        });
}

fn selected_claw_machine_interaction(
    mut commands: Commands,
    mut buttons: Query<
        (&SelectedClawMachine, &Interaction, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (selected_claw_machine, interaction, mut background_color) in buttons.iter_mut() {
        match interaction {
            Interaction::None => background_color.0 = BUTTON_BACKGROUND_COLOR,
            Interaction::Hovered => background_color.0 = BUTTON_HOVERED_BACKGROUND_COLOR,
            Interaction::Pressed => commands.insert_resource(*selected_claw_machine),
        }
    }
}
