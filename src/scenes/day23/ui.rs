use bevy::{
    app::Update,
    color::{palettes, Color},
    math::Vec2,
    prelude::{
        in_state, BuildChildren, Button, ChildBuild, ChildBuilder, Commands, Component,
        DespawnRecursiveExt, Entity, Gizmos, GlobalTransform, IntoSystemConfigs, NextState, Query,
        Res, ResMut, Single, Text, With,
    },
    text::TextFont,
    ui::{
        AlignSelf, BorderColor, BorderRadius, FlexDirection, FlexWrap, Interaction, Node,
        PositionType, UiRect, Val,
    },
    window::Window,
};

use crate::scenes::{
    days::{build_content, build_header},
    resources::{FontHandles, GenericDay},
    states::{UiState, VisualizationState},
};

use super::input::Input;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui.run_if(in_state(VisualizationState::<23>::WaitingUi)),
        )
        .add_systems(
            Update,
            draw_gizmos.run_if(in_state(VisualizationState::<23>::Ready)),
        );
    }
}

fn build_ui(
    mut commands: Commands,
    day23_resource: Res<GenericDay>,
    mut input: ResMut<Input>,
    mut next_state: ResMut<NextState<UiState>>,
    fonts: Res<FontHandles>,
) {
    bevy::log::trace!("Day 23");
    let header = build_header(&mut commands, "day23", false, fonts.font.clone());
    let content = build_content(&mut commands, "day23");

    commands
        .entity(content)
        .with_children(|parent| build_visualization(parent, &mut input));

    commands
        .entity(day23_resource.ui)
        .despawn_descendants()
        .add_children(&[header, content]);

    next_state.set(UiState::Loaded);
}

fn build_visualization(parent: &mut ChildBuilder, input: &mut Input) {
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
                    column_gap: Val::Px(15.),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(Node::default())
                        .with_child(Text::new("Triplets"));
                    parent
                        .spawn((
                            Node {
                                border: UiRect::all(Val::Px(3.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_child(Text::new(input.triples.len().to_string()));
                });
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(15.),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(Node::default())
                        .with_child(Text::new("Password"));
                    parent
                        .spawn((
                            Node {
                                border: UiRect::all(Val::Px(3.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_child(Text::new(input.password.clone()));
                });

            parent
                .spawn(Node {
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    align_self: AlignSelf::Center,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                height: Val::Percent(100.),
                                flex_direction: FlexDirection::Column,
                                flex_wrap: FlexWrap::NoWrap,
                                aspect_ratio: Some(30. / 26.),
                                border: UiRect::all(Val::Px(3.)),
                                ..Default::default()
                            },
                            BorderColor(Color::WHITE),
                            BorderRadius::all(Val::Px(5.)),
                        ))
                        .with_children(|parent| {
                            for node in input.connections.keys() {
                                let left = (node[0] - b'a') as f32 / 26.;
                                let bottom = (node[1] - b'a') as f32 / 26.;
                                parent
                                    .spawn((
                                        Node {
                                            position_type: PositionType::Absolute,
                                            border: UiRect::left(Val::Px(1.))
                                                .with_bottom(Val::Px(1.)),
                                            left: Val::Percent(100. * left),
                                            bottom: Val::Percent(100. * bottom),
                                            ..Default::default()
                                        },
                                        BorderColor(Color::WHITE),
                                        PcNode(*node),
                                    ))
                                    .with_child((
                                        Text::new(String::from_utf8_lossy(node)),
                                        TextFont {
                                            font_size: 12.,
                                            ..Default::default()
                                        },
                                    ));
                            }
                        });
                });
        });
}

#[derive(Debug, Component)]
#[require(Button)]
struct PcNode([u8; 2]);

fn draw_gizmos(
    mut gizmos: Gizmos,
    input: Res<Input>,
    window: Single<&Window>,
    buttons: Query<(Entity, &Interaction), With<PcNode>>,
    pc_nodes: Query<(&PcNode, &GlobalTransform)>,
) {
    let window_size = window.physical_size().as_vec2() / 2.;

    if let Some((hovered, _)) = buttons
        .iter()
        .find(|(_, interaction)| matches!(interaction, Interaction::Hovered | Interaction::Pressed))
    {
        let (root, location) = pc_nodes.get(hovered).unwrap();
        let connections = input.connections.get(&root.0).unwrap();
        for connection in connections {
            let color = if input.fully_connected.contains(&root.0)
                && input.fully_connected.contains(connection)
            {
                palettes::tailwind::RED_300
            } else if input
                .triples
                .iter()
                .any(|triplet| triplet.contains(&root.0) && triplet.contains(connection))
            {
                palettes::tailwind::GREEN_300
            } else {
                palettes::tailwind::YELLOW_300
            };

            let (_, end_node) = pc_nodes
                .iter()
                .find(|(node, _)| &node.0 == connection)
                .unwrap();

            gizmos.line_2d(
                (location.translation().truncate() - window_size) * Vec2::new(1., -1.),
                (end_node.translation().truncate() - window_size) * Vec2::new(1., -1.),
                color,
            );
        }
    }
}
