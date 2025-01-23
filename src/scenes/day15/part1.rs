use bevy::{
    app::Update,
    color::Color,
    prelude::{
        in_state, BuildChildren, ChildBuild, ChildBuilder, Commands, Condition,
        DespawnRecursiveExt, IntoSystemConfigs, NextState, OnExit, Res, ResMut, Text,
    },
    text::TextColor,
    ui::{BorderColor, BorderRadius, FlexDirection, Node, PositionType, UiRect, Val},
};

use crate::scenes::{
    day15::controls::build_control,
    days::{build_content, build_footer, build_header},
    resources::{FontHandles, GenericDay},
    states::{Part, Scene, UiState, VisualizationState},
};

use super::{
    input::Input,
    sokoban::{SokobanCanvas, Warehouse},
};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            OnExit(Part::Part1),
            tear_down_sokoban.run_if(in_state(Scene::Day(15))),
        )
        .add_systems(
            Update,
            build_ui
                .run_if(in_state(Part::Part1).and(in_state(VisualizationState::<15>::WaitingUi))),
        );
    }
}

fn tear_down_sokoban(
    mut commands: Commands,
    day15_resource: Res<GenericDay>,
    mut next_state: ResMut<NextState<UiState>>,
) {
    commands.remove_resource::<Warehouse>();
    commands.entity(day15_resource.ui).despawn_descendants();
    next_state.set(UiState::NotLoaded);
}

fn build_ui(
    mut commands: Commands,
    day15_resource: Res<GenericDay>,
    input: Res<Input>,
    mut next_state: ResMut<NextState<UiState>>,
    fonts: Res<FontHandles>,
) {
    bevy::log::trace!("Day 15 Part 1");
    let header = build_header(&mut commands, "day15", true, fonts.font.clone());
    let content = build_content(&mut commands, "day15");
    let footer = build_footer(&mut commands, "day15");

    commands.insert_resource(Warehouse::from_input(&input, false));

    commands
        .entity(content)
        .with_children(|parent| build_visualization(parent, &input));
    commands
        .entity(footer)
        .with_children(|parent| build_control(parent, fonts.symbol2.clone()));

    commands
        .entity(day15_resource.ui)
        .despawn_descendants()
        .add_children(&[header, content, footer]);

    next_state.set(UiState::Loaded);
}

fn build_visualization(parent: &mut ChildBuilder, input: &Input) {
    let mut warehouse = Warehouse::from_input(input, false);

    while warehouse.has_instructions() {
        warehouse.next_move();
    }

    let gps = warehouse.compute_gps();

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
                    column_gap: Val::Px(10.),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        })
                        .with_child((Text::new("GPS"), TextColor(Color::WHITE)));
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
                        .with_child((Text::new(gps.to_string()), TextColor(Color::WHITE)));
                });

            parent
                .spawn(Node {
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    align_self: bevy::ui::AlignSelf::Center,
                    ..Default::default()
                })
                .with_children(|parent| {
                    let dimensions = warehouse.dimensions();
                    parent.spawn((
                        Node {
                            height: Val::Percent(100.),
                            flex_direction: FlexDirection::Column,
                            flex_wrap: bevy::ui::FlexWrap::NoWrap,
                            aspect_ratio: Some(dimensions.column as f32 / dimensions.row as f32),
                            border: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        },
                        BorderColor(Color::WHITE),
                        SokobanCanvas,
                    ));
                });
        });
}
