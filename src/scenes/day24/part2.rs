use std::collections::BTreeMap;

use bevy::{
    app::{PreUpdate, Update},
    asset::{AssetServer, Handle},
    color::{palettes, Color},
    core::Name,
    image::Image,
    math::{UVec2, Vec2, Vec3},
    prelude::{
        in_state, not, resource_added, resource_exists, BuildChildren, Button, Camera, Camera2d,
        ChildBuild, ChildBuilder, Commands, DespawnRecursiveExt, Entity, Gizmos, GlobalTransform,
        ImageNode, IntoSystemConfigs, OnEnter, OnExit, Query, Res, Single, Text, With, Without,
    },
    render::view::RenderLayers,
    sprite::{TextureAtlas, TextureAtlasLayout},
    text::{TextColor, TextFont},
    time::Time,
    ui::{
        AlignContent, BackgroundColor, Interaction, JustifyContent, Node, Overflow, PositionType,
        ScrollPosition, Val,
    },
    window::{PrimaryWindow, Window},
};

use crate::scenes::{
    day24::{components::Gate, operation::Operator},
    days::{build_content, build_footer, build_header, button_node},
    BUTTON_BACKGROUND_COLOR, BUTTON_HOVERED_BACKGROUND_COLOR, FONT_SYMBOLS_2_HANDLE,
};

use super::{
    components::{Adder, Controls, GizmosCamera},
    resources::{Day24, Input},
    states::States,
};

const IMAGE_SIZE: f32 = 32.;
const PADDING_BETWEEN_GATES: f32 = 16.;
const ADDER_PADDING: f32 = 20.;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PreUpdate,
            build_ui
                .run_if(in_state(States::Part2))
                .run_if(resource_added::<Input>),
        )
        .add_systems(OnExit(States::Part2), destroy_ui)
        .add_systems(
            Update,
            super::process_input
                .run_if(not(resource_exists::<Input>))
                .run_if(in_state(States::Part2)),
        )
        .add_systems(
            Update,
            (controls_interaction, draw_connections)
                .run_if(in_state(States::Part2))
                .run_if(resource_exists::<Input>)
                .chain(),
        )
        .add_systems(OnEnter(States::Part2), spawn_gizmos_camera)
        .add_systems(OnExit(States::Part2), despawn_gizmos_camera);
    }
}

type ControlWithChangedInteractionQuery<'a, 'b> = Query<
    'a,
    'b,
    (
        &'static mut BackgroundColor,
        &'static Interaction,
        &'static Controls,
    ),
    With<Button>,
>;

fn controls_interaction(
    mut controls: ControlWithChangedInteractionQuery,
    mut nodes: Single<&mut ScrollPosition, With<Adder>>,
    time: Res<Time>,
) {
    const SCROLL_SPEED: f32 = 512.;

    for (mut background_color, interaction, control) in controls.iter_mut() {
        match interaction {
            Interaction::None => background_color.0 = BUTTON_BACKGROUND_COLOR,
            Interaction::Hovered | Interaction::Pressed => {
                match control {
                    Controls::Reset => {
                        nodes.offset_x -= SCROLL_SPEED * time.delta_secs();
                    }
                    Controls::Step => {
                        unreachable!("Not present on part 2");
                    }
                    Controls::FastForward => {
                        nodes.offset_x += SCROLL_SPEED * time.delta_secs();
                    }
                };
                background_color.0 = BUTTON_HOVERED_BACKGROUND_COLOR;
            }
        }
    }
}

fn draw_connections(
    gates: Query<(&Gate, &GlobalTransform), Without<PrimaryWindow>>,
    windows: Single<&Window>,
    mut gizmos: Gizmos,
) {
    let mut inputs: BTreeMap<[u8; 3], Vec<Vec3>> = BTreeMap::new();
    let mut outputs = BTreeMap::new();

    let window_size = windows.physical_size().as_vec2() / 2.;

    for (gate, transform) in gates.iter() {
        let left_pos =
            transform.translation() + Vec3::new(-(IMAGE_SIZE / 4.), -(IMAGE_SIZE / 2.), 0.);
        let right_pos =
            transform.translation() + Vec3::new(IMAGE_SIZE / 4., -(IMAGE_SIZE / 2.), 0.);
        let out_pos = transform.translation() + Vec3::new(0., IMAGE_SIZE / 2., 0.);
        inputs
            .entry(gate.left)
            .and_modify(|vec| {
                vec.push(left_pos);
            })
            .or_insert_with(|| vec![left_pos]);
        inputs
            .entry(gate.right)
            .and_modify(|vec| {
                vec.push(right_pos);
            })
            .or_insert_with(|| vec![right_pos]);
        outputs.insert(gate.out, out_pos);
    }

    for (output, origin) in outputs.iter().filter(|(k, _)| **k != [0; 3]) {
        for input in inputs.get(output).unwrap() {
            gizmos.line_2d(
                (origin.truncate() - window_size) * Vec2::new(1., -1.),
                (input.truncate() - window_size) * Vec2::new(1., -1.),
                palettes::basic::RED,
            );
        }
    }
}

fn build_ui(
    mut commands: Commands,
    day24_resource: Res<Day24>,
    input: Res<Input>,
    asset_server: Res<AssetServer>,
) {
    bevy::log::trace!("Day 24 Part 2");

    let header = build_header(&mut commands, "day24", true);
    let content = build_content(&mut commands, "day24");
    let footer = build_footer(&mut commands, "day24");

    let gates = asset_server.load("gates.png");
    let gates_atlas_layout = asset_server.add(TextureAtlasLayout::from_grid(
        UVec2::splat(32),
        2,
        2,
        None,
        None,
    ));

    let content_data = build_visualization(&mut commands, &input, gates, gates_atlas_layout);

    commands.entity(content).add_child(content_data);
    commands.entity(footer).with_children(build_control_buttons);

    commands
        .entity(day24_resource.ui)
        .add_children(&[header, content, footer]);
}

fn destroy_ui(mut commands: Commands, day24_resource: Res<Day24>) {
    commands.remove_resource::<Input>();
    commands.entity(day24_resource.ui).despawn_descendants();
}

fn spawn_gizmos_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("day24_part2_gizmos_camera"),
        Camera2d,
        Camera {
            order: 100,
            ..Default::default()
        },
        RenderLayers::from_layers(&[1]),
        GizmosCamera,
    ));
}

fn despawn_gizmos_camera(mut commands: Commands, cameras: Query<Entity, With<GizmosCamera>>) {
    for camera in cameras.iter() {
        commands.entity(camera).despawn_recursive();
    }
}

fn build_visualization(
    parent: &mut Commands,
    input: &Input,
    gates: Handle<Image>,
    gates_atlas_layout: Handle<TextureAtlasLayout>,
) -> Entity {
    parent
        .spawn((
            Name::new("day24_part2_visualization"),
            Adder,
            Node {
                top: Val::Px(50.),
                width: Val::Vw(100.),
                flex_direction: bevy::ui::FlexDirection::Row,
                column_gap: Val::Px(20.),
                overflow: Overflow::scroll_x(),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            let mut reverse_operations = input.operations.clone();
            reverse_operations.reverse();
            for (i, chunk) in reverse_operations.chunks(5).enumerate() {
                let index = u8::try_from(44 - i).unwrap();
                let xkey = [b'x', (index / 10) + b'0', (index % 10) + b'0'];
                let ykey = [b'y', (index / 10) + b'0', (index % 10) + b'0'];
                let zkey = [b'z', (index / 10) + b'0', (index % 10) + b'0'];

                match chunk {
                    [or, and_b, xor_b, and, xor] => {
                        assert_eq!(or.operator, Operator::Or);
                        assert_eq!(and_b.operator, Operator::And);
                        assert_eq!(xor_b.operator, Operator::Xor);
                        assert_eq!(and.operator, Operator::And);
                        assert_eq!(xor.operator, Operator::Xor);

                        parent
                            .spawn((
                                Node {
                                    min_width: Val::Px(
                                        ADDER_PADDING * 2.
                                            + IMAGE_SIZE * 3.
                                            + PADDING_BETWEEN_GATES * 6.,
                                    ),
                                    max_height: Val::Px(
                                        ADDER_PADDING * 5.
                                            + IMAGE_SIZE * 4.
                                            + PADDING_BETWEEN_GATES * 8.,
                                    ),
                                    ..Default::default()
                                },
                                BackgroundColor(palettes::tailwind::BLUE_100.into()),
                            ))
                            .with_children(|parent| {
                                parent
                                    .spawn((
                                        Node {
                                            top: Val::Px(ADDER_PADDING),
                                            left: Val::Px(ADDER_PADDING + PADDING_BETWEEN_GATES),
                                            min_width: Val::Px(IMAGE_SIZE),
                                            min_height: Val::Px(IMAGE_SIZE),
                                            position_type: PositionType::Absolute,
                                            justify_content: JustifyContent::Center,
                                            align_content: AlignContent::Center,
                                            ..Default::default()
                                        },
                                        Gate {
                                            left: [0; 3],
                                            right: [0; 3],
                                            out: xkey,
                                        },
                                    ))
                                    .with_child((
                                        Text::new(String::from_utf8_lossy(&xkey)),
                                        TextColor(Color::BLACK),
                                    ));
                                parent
                                    .spawn((
                                        Node {
                                            top: Val::Px(ADDER_PADDING),
                                            left: Val::Px(
                                                ADDER_PADDING
                                                    + IMAGE_SIZE
                                                    + PADDING_BETWEEN_GATES * 3.,
                                            ),
                                            min_width: Val::Px(IMAGE_SIZE),
                                            min_height: Val::Px(IMAGE_SIZE),
                                            position_type: PositionType::Absolute,
                                            justify_content: JustifyContent::Center,
                                            align_content: AlignContent::Center,
                                            ..Default::default()
                                        },
                                        Gate {
                                            left: [0; 3],
                                            right: [0; 3],
                                            out: ykey,
                                        },
                                    ))
                                    .with_child((
                                        Text::new(String::from_utf8_lossy(&ykey)),
                                        TextColor(Color::BLACK),
                                    ));
                                parent
                                    .spawn((
                                        Node {
                                            top: Val::Px(
                                                ADDER_PADDING * 2.
                                                    + IMAGE_SIZE
                                                    + PADDING_BETWEEN_GATES * 2.,
                                            ),
                                            left: Val::Px(ADDER_PADDING),
                                            min_width: Val::Px(
                                                IMAGE_SIZE * 2. + PADDING_BETWEEN_GATES * 4.,
                                            ),
                                            min_height: Val::Px(
                                                IMAGE_SIZE + PADDING_BETWEEN_GATES * 2.,
                                            ),
                                            position_type: PositionType::Absolute,
                                            ..Default::default()
                                        },
                                        BackgroundColor(palettes::tailwind::BLUE_300.into()),
                                    ))
                                    .with_children(|parent| {
                                        parent.spawn((
                                            Node {
                                                top: Val::Px(PADDING_BETWEEN_GATES),
                                                left: Val::Px(PADDING_BETWEEN_GATES),
                                                position_type: PositionType::Absolute,
                                                ..Default::default()
                                            },
                                            ImageNode {
                                                image: gates.clone(),
                                                texture_atlas: Some(TextureAtlas {
                                                    layout: gates_atlas_layout.clone(),
                                                    index: 0,
                                                }),
                                                ..Default::default()
                                            },
                                            Gate {
                                                left: and.l,
                                                right: and.r,
                                                out: and.out,
                                            },
                                        ));
                                        parent.spawn((
                                            Node {
                                                top: Val::Px(PADDING_BETWEEN_GATES),
                                                left: Val::Px(
                                                    IMAGE_SIZE + PADDING_BETWEEN_GATES * 3.,
                                                ),
                                                position_type: PositionType::Absolute,
                                                ..Default::default()
                                            },
                                            ImageNode {
                                                image: gates.clone(),
                                                texture_atlas: Some(TextureAtlas {
                                                    layout: gates_atlas_layout.clone(),
                                                    index: 3,
                                                }),
                                                ..Default::default()
                                            },
                                            Gate {
                                                left: xor.l,
                                                right: xor.r,
                                                out: xor.out,
                                            },
                                        ));
                                    });
                                parent
                                    .spawn((
                                        Node {
                                            top: Val::Px(
                                                ADDER_PADDING * 3.
                                                    + IMAGE_SIZE * 2.
                                                    + PADDING_BETWEEN_GATES * 4.,
                                            ),
                                            left: Val::Px(
                                                ADDER_PADDING
                                                    + IMAGE_SIZE
                                                    + PADDING_BETWEEN_GATES * 2.,
                                            ),
                                            min_width: Val::Px(
                                                IMAGE_SIZE * 2. + PADDING_BETWEEN_GATES * 4.,
                                            ),
                                            min_height: Val::Px(
                                                IMAGE_SIZE + PADDING_BETWEEN_GATES * 2.,
                                            ),
                                            position_type: PositionType::Absolute,
                                            ..Default::default()
                                        },
                                        BackgroundColor(palettes::tailwind::BLUE_300.into()),
                                    ))
                                    .with_children(|parent| {
                                        parent.spawn((
                                            Node {
                                                top: Val::Px(PADDING_BETWEEN_GATES),
                                                left: Val::Px(PADDING_BETWEEN_GATES),
                                                position_type: PositionType::Absolute,
                                                ..Default::default()
                                            },
                                            ImageNode {
                                                image: gates.clone(),
                                                texture_atlas: Some(TextureAtlas {
                                                    layout: gates_atlas_layout.clone(),
                                                    index: 0,
                                                }),
                                                ..Default::default()
                                            },
                                            Gate {
                                                left: and_b.l,
                                                right: and_b.r,
                                                out: and_b.out,
                                            },
                                        ));
                                        parent.spawn((
                                            Node {
                                                top: Val::Px(PADDING_BETWEEN_GATES),
                                                left: Val::Px(
                                                    IMAGE_SIZE + PADDING_BETWEEN_GATES * 3.,
                                                ),
                                                position_type: PositionType::Absolute,
                                                ..Default::default()
                                            },
                                            ImageNode {
                                                image: gates.clone(),
                                                texture_atlas: Some(TextureAtlas {
                                                    layout: gates_atlas_layout.clone(),
                                                    index: 3,
                                                }),
                                                ..Default::default()
                                            },
                                            Gate {
                                                left: xor_b.l,
                                                right: xor_b.r,
                                                out: xor_b.out,
                                            },
                                        ));
                                    });
                                parent.spawn((
                                    Node {
                                        top: Val::Px(
                                            ADDER_PADDING * 3.
                                                + IMAGE_SIZE * 2.
                                                + PADDING_BETWEEN_GATES * 5.,
                                        ),
                                        left: Val::Px(ADDER_PADDING + PADDING_BETWEEN_GATES),
                                        position_type: PositionType::Absolute,
                                        ..Default::default()
                                    },
                                    ImageNode {
                                        image: gates.clone(),
                                        texture_atlas: Some(TextureAtlas {
                                            layout: gates_atlas_layout.clone(),
                                            index: 2,
                                        }),
                                        ..Default::default()
                                    },
                                    Gate {
                                        left: or.l,
                                        right: or.r,
                                        out: or.out,
                                    },
                                ));
                                if index == 44 {
                                    parent
                                        .spawn((
                                            Node {
                                                top: Val::Px(
                                                    ADDER_PADDING * 4.
                                                        + IMAGE_SIZE * 3.
                                                        + PADDING_BETWEEN_GATES * 7.,
                                                ),
                                                left: Val::Px(
                                                    ADDER_PADDING + PADDING_BETWEEN_GATES,
                                                ),
                                                min_width: Val::Px(IMAGE_SIZE),
                                                min_height: Val::Px(IMAGE_SIZE),
                                                position_type: PositionType::Absolute,
                                                justify_content: JustifyContent::Center,
                                                align_content: AlignContent::Center,
                                                ..Default::default()
                                            },
                                            Gate {
                                                left: b"z45".to_owned(),
                                                right: [0; 3],
                                                out: [0; 3],
                                            },
                                        ))
                                        .with_child((Text::new("z45"), TextColor(Color::BLACK)));
                                }
                                parent
                                    .spawn((
                                        Node {
                                            top: Val::Px(
                                                ADDER_PADDING * 4.
                                                    + IMAGE_SIZE * 3.
                                                    + PADDING_BETWEEN_GATES * 7.,
                                            ),
                                            left: Val::Px(
                                                ADDER_PADDING
                                                    + IMAGE_SIZE * 2.
                                                    + PADDING_BETWEEN_GATES * 5.,
                                            ),
                                            min_width: Val::Px(IMAGE_SIZE),
                                            min_height: Val::Px(IMAGE_SIZE),
                                            position_type: PositionType::Absolute,
                                            justify_content: JustifyContent::Center,
                                            align_content: AlignContent::Center,
                                            ..Default::default()
                                        },
                                        Gate {
                                            left: zkey,
                                            right: [0; 3],
                                            out: [0; 3],
                                        },
                                    ))
                                    .with_child((
                                        Text::new(String::from_utf8_lossy(&zkey)),
                                        TextColor(Color::BLACK),
                                    ));
                            });
                    }
                    [and, xor] => {
                        assert_eq!(and.operator, Operator::And);
                        assert_eq!(xor.operator, Operator::Xor);

                        parent
                            .spawn((
                                Node {
                                    min_width: Val::Px(
                                        ADDER_PADDING * 2.
                                            + IMAGE_SIZE * 3.
                                            + PADDING_BETWEEN_GATES * 6.,
                                    ),
                                    max_height: Val::Px(
                                        ADDER_PADDING * 5.
                                            + IMAGE_SIZE * 4.
                                            + PADDING_BETWEEN_GATES * 8.,
                                    ),
                                    ..Default::default()
                                },
                                BackgroundColor(palettes::tailwind::BLUE_100.into()),
                            ))
                            .with_children(|parent| {
                                parent
                                    .spawn((
                                        Node {
                                            top: Val::Px(ADDER_PADDING),
                                            left: Val::Px(ADDER_PADDING + PADDING_BETWEEN_GATES),
                                            min_width: Val::Px(IMAGE_SIZE),
                                            min_height: Val::Px(IMAGE_SIZE),
                                            position_type: PositionType::Absolute,
                                            justify_content: JustifyContent::Center,
                                            align_content: AlignContent::Center,
                                            ..Default::default()
                                        },
                                        Gate {
                                            left: [0; 3],
                                            right: [0; 3],
                                            out: xkey,
                                        },
                                    ))
                                    .with_child((
                                        Text::new(String::from_utf8_lossy(&xkey)),
                                        TextColor(Color::BLACK),
                                    ));
                                parent
                                    .spawn((
                                        Node {
                                            top: Val::Px(ADDER_PADDING),
                                            left: Val::Px(
                                                ADDER_PADDING
                                                    + IMAGE_SIZE
                                                    + PADDING_BETWEEN_GATES * 3.,
                                            ),
                                            min_width: Val::Px(IMAGE_SIZE),
                                            min_height: Val::Px(IMAGE_SIZE),
                                            position_type: PositionType::Absolute,
                                            justify_content: JustifyContent::Center,
                                            align_content: AlignContent::Center,
                                            ..Default::default()
                                        },
                                        Gate {
                                            left: [0; 3],
                                            right: [0; 3],
                                            out: ykey,
                                        },
                                    ))
                                    .with_child((
                                        Text::new(String::from_utf8_lossy(&ykey)),
                                        TextColor(Color::BLACK),
                                    ));
                                parent
                                    .spawn((
                                        Node {
                                            top: Val::Px(
                                                ADDER_PADDING * 2.
                                                    + IMAGE_SIZE
                                                    + PADDING_BETWEEN_GATES * 2.,
                                            ),
                                            left: Val::Px(ADDER_PADDING),
                                            min_width: Val::Px(
                                                IMAGE_SIZE * 2. + PADDING_BETWEEN_GATES * 4.,
                                            ),
                                            min_height: Val::Px(
                                                IMAGE_SIZE + PADDING_BETWEEN_GATES * 2.,
                                            ),
                                            position_type: PositionType::Absolute,
                                            ..Default::default()
                                        },
                                        BackgroundColor(palettes::tailwind::BLUE_300.into()),
                                    ))
                                    .with_children(|parent| {
                                        parent.spawn((
                                            Node {
                                                top: Val::Px(PADDING_BETWEEN_GATES),
                                                left: Val::Px(PADDING_BETWEEN_GATES),
                                                position_type: PositionType::Absolute,
                                                ..Default::default()
                                            },
                                            ImageNode {
                                                image: gates.clone(),
                                                texture_atlas: Some(TextureAtlas {
                                                    layout: gates_atlas_layout.clone(),
                                                    index: 0,
                                                }),
                                                ..Default::default()
                                            },
                                            Gate {
                                                left: and.l,
                                                right: and.r,
                                                out: and.out,
                                            },
                                        ));
                                        parent.spawn((
                                            Node {
                                                top: Val::Px(PADDING_BETWEEN_GATES),
                                                left: Val::Px(
                                                    IMAGE_SIZE + PADDING_BETWEEN_GATES * 3.,
                                                ),
                                                position_type: PositionType::Absolute,
                                                ..Default::default()
                                            },
                                            ImageNode {
                                                image: gates.clone(),
                                                texture_atlas: Some(TextureAtlas {
                                                    layout: gates_atlas_layout.clone(),
                                                    index: 3,
                                                }),
                                                ..Default::default()
                                            },
                                            Gate {
                                                left: xor.l,
                                                right: xor.r,
                                                out: xor.out,
                                            },
                                        ));
                                    });
                                parent
                                    .spawn((
                                        Node {
                                            top: Val::Px(
                                                ADDER_PADDING * 4.
                                                    + IMAGE_SIZE * 3.
                                                    + PADDING_BETWEEN_GATES * 7.,
                                            ),
                                            left: Val::Px(
                                                ADDER_PADDING
                                                    + IMAGE_SIZE
                                                    + PADDING_BETWEEN_GATES * 3.,
                                            ),
                                            min_width: Val::Px(IMAGE_SIZE),
                                            min_height: Val::Px(IMAGE_SIZE),
                                            position_type: PositionType::Absolute,
                                            justify_content: JustifyContent::Center,
                                            align_content: AlignContent::Center,
                                            ..Default::default()
                                        },
                                        Gate {
                                            left: zkey,
                                            right: [0; 3],
                                            out: [0; 3],
                                        },
                                    ))
                                    .with_child((
                                        Text::new(String::from_utf8_lossy(&zkey)),
                                        TextColor(Color::BLACK),
                                    ));
                            });
                    }
                    _ => unreachable!("Should be N * 5 + 2."),
                };
            }
        })
        .id()
}

fn build_control_buttons(parent: &mut ChildBuilder) {
    let font = FONT_SYMBOLS_2_HANDLE
        .get()
        .expect("Font should be initialized.");
    parent
        .spawn((
            button_node(),
            Controls::Reset,
            BackgroundColor(BUTTON_BACKGROUND_COLOR),
        ))
        .with_child((
            Text::new("⏮"),
            TextFont {
                font: font.clone(),
                ..Default::default()
            },
            TextColor(Color::BLACK),
        ));
    parent
        .spawn((
            button_node(),
            Controls::FastForward,
            BackgroundColor(BUTTON_BACKGROUND_COLOR),
        ))
        .with_child((
            Text::new("⏭"),
            TextFont {
                font: font.clone(),
                ..Default::default()
            },
            TextColor(Color::BLACK),
        ));
}
