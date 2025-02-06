use bevy::{
    app::Update,
    color::{palettes, Color, Srgba},
    prelude::{
        in_state, Animatable, BuildChildren, Button, ChildBuild, ChildBuilder, Commands, Component,
        Condition, DespawnRecursiveExt, Entity, IntoSystemConfigs, NextState, Query, Res, ResMut,
        Text, With,
    },
    ui::{
        BackgroundColor, BorderColor, BorderRadius, FlexDirection, Interaction, Node, PositionType,
        UiRect, Val,
    },
};

use crate::{
    scenes::{
        days::{build_content, build_header},
        resources::{FontHandles, GenericDay},
        states::{Part, UiState, VisualizationState},
    },
    tools::Coord,
};

use super::input::Input;

const MAIN_PATH_ZERO_SHORTCUT_COLOR: Srgba = palettes::tailwind::YELLOW_300;
const MAIN_PATH_HAS_SHORTCUT_COLOR: Srgba = palettes::tailwind::YELLOW_500;
const WALL_COLOR: Srgba = palettes::tailwind::GRAY_700;
const SHORT_SHORTCUT_COLOR: Srgba = palettes::tailwind::GREEN_300;
const LONG_SHORTCUT_COLOR: Srgba = palettes::tailwind::RED_300;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui::<false>
                .run_if(in_state(Part::Part1).and(in_state(VisualizationState::<20>::WaitingUi))),
        )
        .add_systems(
            Update,
            build_ui::<true>
                .run_if(in_state(Part::Part2).and(in_state(VisualizationState::<20>::WaitingUi))),
        )
        .add_systems(
            Update,
            update_tiles.run_if(in_state(VisualizationState::<20>::Ready)),
        );
    }
}

fn build_ui<const PART2: bool>(
    mut commands: Commands,
    day20_resource: Res<GenericDay>,
    mut input: ResMut<Input>,
    mut next_state: ResMut<NextState<UiState>>,
    fonts: Res<FontHandles>,
) {
    bevy::log::trace!("Day 20");
    let header = build_header(&mut commands, "day20", true, fonts.font.clone());
    let content = build_content(&mut commands, "day20");

    commands
        .entity(content)
        .with_children(|parent| build_visualization::<PART2>(parent, &mut input));

    commands
        .entity(day20_resource.ui)
        .despawn_descendants()
        .add_children(&[header, content]);

    next_state.set(UiState::Loaded);
}

fn build_visualization<const PART2: bool>(parent: &mut ChildBuilder, input: &mut Input) {
    let shortcuts = if PART2 {
        input.cheat(20)
    } else {
        input.cheat(2)
    };
    let max = shortcuts.keys().max().unwrap();
    let max_shortcut = shortcuts
        .values()
        .filter_map(|ends| ends.iter().map(|end| end.1).max())
        .max()
        .unwrap_or_default() as f32;

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
                        .with_child(Text::new("Shortcuts"));
                    parent
                        .spawn((
                            Node {
                                border: UiRect::all(Val::Px(3.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_child(Text::new(
                            shortcuts
                                .values()
                                .map(|ends| ends.iter().filter(|len| len.1 >= 100).count())
                                .sum::<usize>()
                                .to_string(),
                        ));
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
                                aspect_ratio: Some(max.column as f32 / max.row as f32),
                                border: UiRect::all(Val::Px(3.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_children(|parent| {
                            for row in 0..=max.row {
                                parent
                                    .spawn(Node {
                                        width: Val::Percent(100.),
                                        height: Val::Percent(10000. / max.row as f32),
                                        flex_direction: FlexDirection::Row,
                                        ..Default::default()
                                    })
                                    .with_children(|parent| {
                                        for column in 0..=max.column {
                                            let coord = Coord::new(row, column);

                                            let shortcuts_starting_from =
                                                shortcuts.get(&coord).cloned();
                                            let ends = shortcuts_starting_from
                                                .as_ref()
                                                .map(|ends| {
                                                    ends.iter()
                                                        .map(|(a, b)| {
                                                            (*a, *b as f32 / max_shortcut)
                                                        })
                                                        .collect::<Vec<_>>()
                                                })
                                                .unwrap_or_default();

                                            let background_color =
                                                if shortcuts_starting_from.is_some() {
                                                    if shortcuts_starting_from
                                                        .as_ref()
                                                        .filter(|ends| !ends.is_empty())
                                                        .is_some()
                                                    {
                                                        MAIN_PATH_HAS_SHORTCUT_COLOR
                                                    } else {
                                                        MAIN_PATH_ZERO_SHORTCUT_COLOR
                                                    }
                                                } else {
                                                    WALL_COLOR
                                                };
                                            parent.spawn((
                                                Node {
                                                    height: Val::Percent(100.),
                                                    aspect_ratio: Some(1.),
                                                    ..Default::default()
                                                },
                                                BackgroundColor(background_color.into()),
                                                Tile(coord, background_color.into()),
                                                Ends(ends),
                                            ));
                                        }
                                    });
                            }
                        });
                });
        });
}

#[derive(Debug, Component)]
struct Tile(Coord, Color);

#[derive(Debug, Component)]
#[require(Button)]
struct Ends(Vec<(Coord, f32)>);

fn update_tiles(
    ends: Query<&Ends>,
    interactions: Query<(Entity, &Interaction), With<Ends>>,
    mut backgrounds: Query<(&Tile, &mut BackgroundColor), With<Ends>>,
) {
    if let Some((hovered, _)) = interactions
        .iter()
        .find(|(_, interaction)| matches!(interaction, Interaction::Hovered | Interaction::Pressed))
    {
        let ends = ends.get(hovered).unwrap();
        for (tile, mut background) in backgrounds.iter_mut() {
            if let Some((_, percent_of_max_shortcut)) =
                ends.0.iter().find(|(coord, _)| coord == &tile.0)
            {
                background.0 = Srgba::interpolate(
                    &SHORT_SHORTCUT_COLOR,
                    &LONG_SHORTCUT_COLOR,
                    *percent_of_max_shortcut,
                )
                .into();
            } else {
                background.0 = tile.1;
            }
        }
    }
}
