use bevy::{
    app::Update,
    color::{palettes, Color},
    prelude::{
        in_state, BuildChildren, ChildBuild, ChildBuilder, Commands, Condition,
        DespawnRecursiveExt, IntoSystemConfigs, NextState, Res, ResMut, Text,
    },
    text::{TextColor, TextFont},
    ui::{
        BorderColor, BorderRadius, FlexDirection, FlexWrap, Node, Overflow, PositionType, UiRect,
        Val,
    },
};

use crate::{
    scenes::{
        days::{build_content, build_header},
        resources::{FontHandles, GenericDay},
        states::{Part, UiState, VisualizationState},
    },
    scroll_controls::{self, BUTTON_BACKGROUND_COLOR},
};

use super::input::Input;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui
                .run_if(in_state(Part::Part2).and(in_state(VisualizationState::<22>::WaitingUi))),
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
    bevy::log::trace!("Day 22 Part 2");
    let header = build_header(&mut commands, "day22", true, fonts.font.clone());
    let content = build_content(&mut commands, "day22");

    commands
        .entity(content)
        .with_children(|parent| build_visualization(parent, &mut input, &fonts));

    commands
        .entity(day22_resource.ui)
        .despawn_descendants()
        .add_children(&[header, content]);

    next_state.set(UiState::Loaded);
}

fn build_visualization(parent: &mut ChildBuilder, input: &mut Input, fonts: &FontHandles) {
    let (best_sell_change, best_sell) = input.part2();
    let best_sell_change_string = best_sell_change.map(|change| change.to_string());

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
                        .spawn(Node {
                            border: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        })
                        .with_child(Text::new("Best sell"));
                    parent
                        .spawn((
                            Node {
                                border: UiRect::all(Val::Px(3.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_child(Text::new(best_sell.to_string()));
                });

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(5.),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(Node {
                            border: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        })
                        .with_child(Text::new("Best Series Change"));
                    parent
                        .spawn((
                            Node {
                                border: UiRect::all(Val::Px(3.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_child(Text::new(best_sell_change_string.join(",")));
                });

            let window = parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        height: Val::Percent(100.),
                        overflow: Overflow::scroll_y(),

                        ..Default::default()
                    },
                    scroll_controls::ScrollWindow,
                ))
                .with_children(|parent| {
                    for rng in input.rngs() {
                        parent
                            .spawn(Node {
                                flex_direction: FlexDirection::Row,
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn((
                                        Node {
                                            flex_direction: FlexDirection::Row,
                                            border: UiRect::all(Val::Px(2.)),
                                            ..Default::default()
                                        },
                                        BorderColor(Color::WHITE),
                                        BorderRadius::all(Val::Px(5.)),
                                    ))
                                    .with_children(|parent| {
                                        if let Some(first_occurance) =
                                            rng.find_first_occurance(&best_sell_change, 2000)
                                        {
                                            let mut changes = best_sell_change_string.iter();
                                            let mut prices = first_occurance
                                                .iter()
                                                .map(|price| price.to_string());
                                            for i in 0..9 {
                                                let string = if i % 2 == 0 {
                                                    prices.next().unwrap()
                                                } else {
                                                    changes.next().unwrap().clone()
                                                };
                                                let color = if i == 8 {
                                                    palettes::tailwind::GREEN_500
                                                } else if i % 2 == 0 {
                                                    palettes::basic::WHITE
                                                } else {
                                                    palettes::tailwind::YELLOW_500
                                                };
                                                let padding = if i % 2 == 0 {
                                                    UiRect::top(Val::Px(8.))
                                                } else {
                                                    UiRect::bottom(Val::Px(8.))
                                                };
                                                parent
                                                    .spawn(Node {
                                                        padding,
                                                        ..Default::default()
                                                    })
                                                    .with_child((
                                                        Text::new(string),
                                                        TextColor(color.into()),
                                                        TextFont {
                                                            font_size: 12.,
                                                            ..Default::default()
                                                        },
                                                    ));
                                            }
                                        } else {
                                            parent.spawn((
                                                Text::new("None"),
                                                TextFont {
                                                    font_size: 12.,
                                                    ..Default::default()
                                                },
                                            ));
                                        }
                                    });
                            });
                    }
                })
                .id();

            scroll_controls::ui::build_vertical_scroll_buttons(
                parent,
                window,
                512.,
                BUTTON_BACKGROUND_COLOR,
                fonts.symbol1.clone(),
            );
        });
}
