use bevy::{
    app::Update,
    asset::{AssetServer, Handle},
    color::Color,
    image::Image,
    prelude::{
        in_state, resource_changed, BuildChildren, ChildBuild, ChildBuilder, Commands, Condition,
        DespawnRecursiveExt, Entity, ImageNode, IntoSystemConfigs, NextState, Res, ResMut, Single,
        Text, With,
    },
    text::TextColor,
    ui::{
        BorderColor, BorderRadius, FlexDirection, JustifyContent, Node, Overflow, PositionType,
        UiRect, Val,
    },
};

use crate::{
    scenes::{
        day11::controls::build_controls,
        days::{build_content, build_footer, build_header},
        resources::{FontHandles, GenericDay},
        states::{UiState, VisualizationState},
    },
    scroll_controls::{ui::build_horizontal_scroll_buttons, ScrollWindow, BUTTON_BACKGROUND_COLOR},
};

use super::input::Input;

const SCROLL_SPEED: f32 = 512.;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui.run_if(in_state(VisualizationState::<11>::WaitingUi)),
        )
        .add_systems(
            Update,
            super::process_input.run_if(in_state(VisualizationState::<11>::WaitingInput)),
        )
        .add_systems(
            Update,
            rebuild_pebbles
                .run_if(in_state(VisualizationState::<11>::Ready).and(resource_changed::<Input>)),
        );
    }
}

fn build_ui(
    mut commands: Commands,
    day11_resource: Res<GenericDay>,
    input: Res<Input>,
    mut next_state: ResMut<NextState<UiState>>,
    fonts: Res<FontHandles>,
    asset_server: Res<AssetServer>,
) {
    bevy::log::trace!("Day 11");
    let header = build_header(&mut commands, "day11", false, fonts.font.clone());
    let content = build_content(&mut commands, "day11");
    let footer = build_footer(&mut commands, "day11");

    commands.entity(content).with_children(|parent| {
        build_visualization(parent, &input, asset_server.load("pebble.png"), &fonts)
    });
    commands.entity(footer).with_children(build_controls);

    commands
        .entity(day11_resource.ui)
        .despawn_descendants()
        .add_children(&[header, content, footer]);

    next_state.set(UiState::Loaded);
}

fn build_visualization(
    parent: &mut ChildBuilder,
    input: &Input,
    pebble_image: Handle<Image>,
    fonts: &FontHandles,
) {
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
                        .with_child((Text::new("After 25 blinks"), TextColor(Color::WHITE)));
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
                            Text::new(input.twenty_five.to_string()),
                            TextColor(Color::WHITE),
                        ));
                    parent
                        .spawn(Node {
                            padding: UiRect::all(Val::Px(3.)),
                            ..Default::default()
                        })
                        .with_child((Text::new("After 75 blinks"), TextColor(Color::WHITE)));
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
                            Text::new(input.seventy_five.to_string()),
                            TextColor(Color::WHITE),
                        ));
                });

            parent
                .spawn(Node {
                    padding: UiRect::bottom(Val::Px(48.)),
                    ..Default::default()
                })
                .with_children(|parent| {
                    let window = parent
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(12.),
                                overflow: Overflow::scroll_x(),
                                ..Default::default()
                            },
                            ScrollWindow,
                        ))
                        .with_children(|parent| build_pebbles(parent, input, pebble_image))
                        .id();
                    build_horizontal_scroll_buttons(
                        parent,
                        window,
                        SCROLL_SPEED,
                        BUTTON_BACKGROUND_COLOR,
                        fonts.symbol2.clone(),
                    );
                });
        });
}

fn rebuild_pebbles(
    mut commands: Commands,
    pebbles: Single<Entity, With<ScrollWindow>>,
    input: Res<Input>,
    asset_server: Res<AssetServer>,
) {
    commands
        .entity(*pebbles)
        .despawn_descendants()
        .with_children(|parent| build_pebbles(parent, &input, asset_server.load("pebble.png")));
}

fn build_pebbles(parent: &mut ChildBuilder, input: &Input, pebble_image: Handle<Image>) {
    for (pebble, count) in input.pebbles.iter() {
        parent
            .spawn((Node {
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },))
            .with_children(|parent| {
                parent
                    .spawn((Node {
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },))
                    .with_child((
                        Node {
                            max_width: Val::Px(32.),
                            max_height: Val::Px(32.),
                            ..Default::default()
                        },
                        ImageNode {
                            image: pebble_image.clone(),
                            ..Default::default()
                        },
                    ));
                parent
                    .spawn(Node {
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    })
                    .with_child(Text::new(pebble.to_string()));
                parent
                    .spawn(Node {
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    })
                    .with_child(Text::new(count.to_string()));
            });
    }
}
