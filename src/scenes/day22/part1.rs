use bevy::{
    app::Update,
    color::Color,
    prelude::{
        in_state, BuildChildren, Button, ChildBuild, ChildBuilder, Click, Commands, Component,
        Condition, DespawnRecursiveExt, Entity, Event, IntoSystemConfigs, NextState, Pointer,
        Query, Res, ResMut, Single, Text, Trigger, With,
    },
    text::TextFont,
    ui::{
        BackgroundColor, BorderColor, BorderRadius, FlexDirection, FlexWrap, Node, PositionType,
        UiRect, Val,
    },
};

use crate::{
    scenes::{
        day22::input::Rng,
        days::{build_content, build_header},
        resources::{FontHandles, GenericDay},
        states::{Part, UiState, VisualizationState},
    },
    scroll_controls::{BUTTON_BACKGROUND_COLOR, BUTTON_SELECTED_BACKGROUND_COLOR},
};

use super::input::Input;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui
                .run_if(in_state(Part::Part1).and(in_state(VisualizationState::<22>::WaitingUi))),
        );
    }
}

fn build_ui(
    mut commands: Commands,
    day22_resource: Res<GenericDay>,
    mut input: ResMut<Input>,
    mut next_state: ResMut<NextState<UiState>>,
    fonts: Res<FontHandles>,
) {
    bevy::log::trace!("Day 22 Part 1");
    let header = build_header(&mut commands, "day22", true, fonts.font.clone());
    let content = build_content(&mut commands, "day22");

    commands
        .entity(content)
        .with_children(|parent| build_visualization(parent, &mut input));

    commands
        .entity(day22_resource.ui)
        .despawn_descendants()
        .add_children(&[header, content]);

    next_state.set(UiState::Loaded);
}

fn build_visualization(parent: &mut ChildBuilder, input: &mut Input) {
    let rngs = input.rngs();
    let rng_sum = rngs
        .iter()
        .cloned()
        .map(|mut rng| rng.nth(1999).unwrap())
        .sum::<usize>();

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
                    parent.spawn(Node::default()).with_child(Text::new("Sum"));
                    parent
                        .spawn((
                            Node {
                                border: UiRect::all(Val::Px(3.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_child(Text::new(rng_sum.to_string()));
                });

            parent
                .spawn((Node {
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::Wrap,
                    row_gap: Val::Px(1.),
                    column_gap: Val::Px(1.),
                    ..Default::default()
                },))
                .with_children(|parent| {
                    for rng in rngs.iter().cloned() {
                        parent
                            .spawn((
                                Node {
                                    width: Val::Px(8.),
                                    height: Val::Px(8.),
                                    ..Default::default()
                                },
                                BackgroundColor(BUTTON_BACKGROUND_COLOR),
                                Monkey(rng),
                            ))
                            .observe(select_money);
                    }
                });

            parent
                .spawn((
                    Node {
                        height: Val::Percent(100.),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(4.),
                        ..Default::default()
                    },
                    Canvas,
                ))
                .observe(update_canvas);
        });
}

#[derive(Debug, Component)]
struct Canvas;

#[derive(Debug, Event)]
struct UpdateCanvas;

#[derive(Debug, Component)]
struct Selected;

#[derive(Debug, Component)]
#[require(Button)]
struct Monkey(Rng);

#[allow(clippy::type_complexity)]
fn select_money(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut monkeys: Query<(Entity, &mut BackgroundColor), (With<Monkey>, With<Selected>)>,
    canvas: Single<Entity, With<Canvas>>,
) {
    for (monkey, mut background_color) in monkeys.iter_mut() {
        background_color.0 = BUTTON_BACKGROUND_COLOR;
        commands.entity(monkey).remove::<Selected>();
    }

    commands
        .entity(trigger.entity())
        .insert((Selected, BackgroundColor(BUTTON_SELECTED_BACKGROUND_COLOR)));
    commands.trigger_targets(UpdateCanvas, *canvas);
}

fn update_canvas(
    trigger: Trigger<UpdateCanvas>,
    mut commands: Commands,
    monkey: Single<&Monkey, With<Selected>>,
) {
    commands
        .entity(trigger.entity())
        .despawn_descendants()
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(12.),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(Node {
                            border: UiRect::all(Val::Px(2.)),
                            padding: UiRect::all(Val::Px(4.)),
                            ..Default::default()
                        })
                        .with_child(Text::new("Seed"));

                    parent
                        .spawn((
                            Node {
                                border: UiRect::all(Val::Px(2.)),
                                padding: UiRect::all(Val::Px(4.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_child(Text::new(monkey.0.secret.to_string()));
                });

            parent
                .spawn((
                    Node {
                        border: UiRect::all(Val::Px(2.)),
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        row_gap: Val::Px(5.),
                        column_gap: Val::Px(5.),
                        padding: UiRect::all(Val::Px(4.)),
                        ..Default::default()
                    },
                    BorderColor(Color::WHITE),
                    BorderRadius::all(Val::Px(5.)),
                ))
                .with_children(|parent| {
                    for val in monkey.0.take(1999) {
                        parent.spawn(Node::default()).with_child((
                            Text::new(val.to_string()),
                            TextFont {
                                font_size: 8.,
                                ..Default::default()
                            },
                        ));
                    }
                });
        });
}
