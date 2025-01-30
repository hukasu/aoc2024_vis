use bevy::{
    app::Update,
    color::{palettes, Color},
    prelude::{
        in_state, resource_changed, BuildChildren, Button, Changed, ChildBuild, ChildBuilder,
        Commands, Component, Condition, DespawnRecursiveExt, Entity, ImageNode, IntoSystemConfigs,
        NextState, Query, Res, ResMut, Resource, Single, Text, With,
    },
    ui::{
        BackgroundColor, BorderColor, BorderRadius, FlexDirection, FlexWrap, Interaction, Node,
        PositionType, UiRect, Val,
    },
};

use crate::scenes::{
    day19::input::IMAGE_HEIGHT,
    days::{build_content, build_header},
    resources::{FontHandles, GenericDay},
    states::{Part, UiState, VisualizationState},
};

use super::input::{Input, Pattern};

const PATTERN_IMAGE_MULTIPLIER: f32 = 5.;
const TOWEL_IMAGE_MULTIPLIER: f32 = 3.;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui::<false>
                .run_if(in_state(Part::Part1).and(in_state(VisualizationState::<19>::WaitingUi))),
        )
        .add_systems(
            Update,
            build_ui::<true>
                .run_if(in_state(Part::Part2).and(in_state(VisualizationState::<19>::WaitingUi))),
        )
        .add_systems(
            Update,
            pattern_button_interaction.run_if(in_state(VisualizationState::<19>::Ready)),
        )
        .add_systems(
            Update,
            update_pattern_buttons.run_if(
                in_state(VisualizationState::<19>::Ready).and(resource_changed::<SelectedPattern>),
            ),
        )
        .add_systems(
            Update,
            update_canvas::<1>.run_if(
                in_state(Part::Part1)
                    .and(in_state(VisualizationState::<19>::Ready))
                    .and(resource_changed::<SelectedPattern>),
            ),
        )
        .add_systems(
            Update,
            update_canvas::<16>.run_if(
                in_state(Part::Part2)
                    .and(in_state(VisualizationState::<19>::Ready))
                    .and(resource_changed::<SelectedPattern>),
            ),
        );
    }
}

fn build_ui<const PART2: bool>(
    mut commands: Commands,
    day19_resource: Res<GenericDay>,
    input: Res<Input>,
    mut next_state: ResMut<NextState<UiState>>,
    fonts: Res<FontHandles>,
) {
    bevy::log::trace!("Day 19");
    let header = build_header(&mut commands, "day19", true, fonts.font.clone());
    let content = build_content(&mut commands, "day19");

    commands.insert_resource(SelectedPattern(input.patterns[0].clone()));

    commands
        .entity(content)
        .with_children(|parent| build_visualization::<PART2>(parent, &input));

    commands
        .entity(day19_resource.ui)
        .despawn_descendants()
        .add_children(&[header, content]);

    next_state.set(UiState::Loaded);
}

fn build_visualization<const PART2: bool>(parent: &mut ChildBuilder, input: &Input) {
    let (text, patterns) = if PART2 {
        let count: usize = input.count_patterns();
        ("Possible patterns", count)
    } else {
        let count: usize = input
            .patterns
            .iter()
            .filter(|pattern| !input.match_pattern(pattern).is_empty())
            .count();
        ("Possible patterns", count)
    };

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
                    parent.spawn(Node::default()).with_child(Text::new(text));
                    parent
                        .spawn((
                            Node {
                                border: UiRect::all(Val::Px(3.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_child(Text::new(patterns.to_string()));
                });

            parent.spawn((
                Node {
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::Wrap,
                    column_gap: Val::Px(5.),
                    row_gap: Val::Px(5.),
                    ..Default::default()
                },
                PatternButtons,
            ));

            parent.spawn((
                Node {
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    align_self: bevy::ui::AlignSelf::Center,
                    ..Default::default()
                },
                Canvas,
            ));
        });
}

#[derive(Debug, Resource)]
pub struct SelectedPattern(Pattern);

#[derive(Debug, Component)]
struct PatternButtons;

fn update_pattern_buttons(
    mut commands: Commands,
    canvas: Single<Entity, With<PatternButtons>>,
    selected_pattern: Res<SelectedPattern>,
    input: Res<Input>,
) {
    commands
        .entity(*canvas)
        .despawn_descendants()
        .with_children(|parent| {
            for pattern in input.patterns.iter() {
                let color = if pattern == &selected_pattern.0 {
                    palettes::tailwind::RED_500.into()
                } else {
                    palettes::tailwind::GREEN_500.into()
                };
                parent.spawn((
                    Node {
                        width: Val::Px(12.),
                        height: Val::Px(12.),
                        display: bevy::ui::Display::Block,
                        ..Default::default()
                    },
                    Button,
                    BackgroundColor(color),
                    pattern.clone(),
                ));
            }
        });
}

fn pattern_button_interaction(
    mut commands: Commands,
    buttons: Query<(&Interaction, &Pattern), Changed<Interaction>>,
) {
    for (button, pattern) in buttons.iter() {
        if button == &Interaction::Pressed {
            commands.insert_resource(SelectedPattern(pattern.clone()))
        }
    }
}

#[derive(Debug, Component)]
struct Canvas;

fn update_canvas<const N: usize>(
    mut commands: Commands,
    canvas: Single<Entity, With<Canvas>>,
    selected_pattern: Res<SelectedPattern>,
    input: Res<Input>,
) {
    commands
        .entity(*canvas)
        .despawn_descendants()
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        border: UiRect::all(Val::Px(3.)),
                        padding: UiRect::all(Val::Px(10.)),
                        ..Default::default()
                    },
                    BorderColor(Color::WHITE),
                    BorderRadius::all(Val::Px(5.)),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((Node {
                            flex_direction: FlexDirection::Row,
                            padding: UiRect::all(Val::Px(2.)),
                            ..Default::default()
                        },))
                        .with_child((
                            Node {
                                width: Val::Px(
                                    selected_pattern.0.pattern.len() as f32
                                        * PATTERN_IMAGE_MULTIPLIER,
                                ),
                                height: Val::Px(IMAGE_HEIGHT as f32 * PATTERN_IMAGE_MULTIPLIER),
                                ..Default::default()
                            },
                            ImageNode {
                                image: selected_pattern.0.image.clone(),
                                ..Default::default()
                            },
                        ));

                    for towels in input.match_pattern(&selected_pattern.0).iter().take(N) {
                        parent
                            .spawn((
                                Node {
                                    flex_direction: FlexDirection::Row,
                                    column_gap: Val::Px(2.),
                                    border: UiRect::top(Val::Px(2.)),
                                    padding: UiRect::all(Val::Px(2.)),
                                    ..Default::default()
                                },
                                BorderColor(Color::WHITE),
                            ))
                            .with_children(|parent| {
                                for towel in towels {
                                    parent.spawn((
                                        Node {
                                            width: Val::Px(
                                                input.longest_towel as f32 * TOWEL_IMAGE_MULTIPLIER,
                                            ),
                                            height: Val::Px(
                                                IMAGE_HEIGHT as f32 * TOWEL_IMAGE_MULTIPLIER,
                                            ),
                                            ..Default::default()
                                        },
                                        ImageNode {
                                            image: towel.image.clone(),
                                            ..Default::default()
                                        },
                                    ));
                                }
                            });
                    }
                });
        });
}
