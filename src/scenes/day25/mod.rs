mod components;
mod input;
mod resources;
mod states;

use bevy::{
    app::Update,
    asset::{AssetServer, Assets, RenderAssetUsages},
    color::{palettes, Color},
    core::Name,
    image::{Image, ImageSampler},
    prelude::{
        in_state, resource_exists, resource_removed, AppExtStates, BuildChildren, Button, Camera2d,
        Changed, ChildBuild, ChildBuilder, ClearColor, Commands, DespawnRecursiveExt, Entity,
        ImageNode, IntoSystemConfigs, NextState, OnEnter, OnExit, Query, Res, ResMut, Text, With,
        Without,
    },
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    text::TextColor,
    ui::{
        BackgroundColor, BorderColor, BorderRadius, FlexDirection, FlexWrap, Interaction, Node,
        PositionType, TargetCamera, UiRect, Val,
    },
};
use components::{usable_key_on_lock, Key, Lock};
use resources::Hovered;

use crate::{
    loader::RawInput,
    scenes::states::States as SceneStates,
    scroll_controls::{BUTTON_BACKGROUND_COLOR, BUTTON_HOVERED_BACKGROUND_COLOR},
};

use super::{
    components::StateChange,
    days::{build_content, build_footer, build_header},
    resources::GenericDay,
};

const PIXEL_PER_UNIT: u32 = 1;
const LOCK_WIDTH: u32 = 5;
const LOCK_HEIGHT: u32 = 7;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_sub_state::<states::InputState>()
            .add_sub_state::<states::UiState>()
            .add_computed_state::<states::VisualizationState>();

        app.add_systems(OnEnter(SceneStates::Day(25)), build_day_25)
            .add_systems(OnExit(SceneStates::Day(25)), destroy_day_25)
            .add_systems(
                Update,
                process_input.run_if(in_state(states::VisualizationState::WaitingInput)),
            )
            .add_systems(
                Update,
                build_ui.run_if(in_state(states::VisualizationState::WaitingUi)),
            )
            .add_systems(
                Update,
                state_button_interactions.run_if(in_state(states::VisualizationState::Ready)),
            )
            .add_systems(
                Update,
                check_hovered.run_if(in_state(states::VisualizationState::Ready)),
            )
            .add_systems(
                Update,
                update_keys_and_locks
                    .after(check_hovered)
                    .run_if(in_state(states::VisualizationState::Ready))
                    .run_if(resource_exists::<Hovered>),
            )
            .add_systems(
                Update,
                (clear_locks, clear_keys)
                    .after(check_hovered)
                    .run_if(in_state(states::VisualizationState::Ready))
                    .run_if(resource_removed::<Hovered>),
            );
    }
}

type ButtonWithChangedInteractionQuery<'a, 'b> = Query<
    'a,
    'b,
    (Entity, &'static mut BackgroundColor, &'static Interaction),
    (With<Button>, Changed<Interaction>),
>;

fn check_hovered(
    mut commands: Commands,
    hovered: Option<Res<Hovered>>,
    buttons: ButtonWithChangedInteractionQuery,
    locks: Query<&Lock>,
    keys: Query<&Key>,
) {
    let mut hovered_changed = false;
    for (button, _, interaction) in buttons.iter() {
        match interaction {
            Interaction::None => match hovered.as_deref() {
                None => (),
                Some(Hovered::Lock(lock)) => {
                    if locks
                        .get(button)
                        .ok()
                        .filter(|button| *button == lock)
                        .is_some()
                        && !hovered_changed
                    {
                        bevy::log::trace!("Removing Lock hovered");
                        commands.remove_resource::<Hovered>();
                    }
                }
                Some(Hovered::Key(key)) => {
                    if keys
                        .get(button)
                        .ok()
                        .filter(|button| *button == key)
                        .is_some()
                        && !hovered_changed
                    {
                        bevy::log::trace!("Removing Key hovered");
                        commands.remove_resource::<Hovered>();
                    }
                }
            },
            Interaction::Hovered | Interaction::Pressed => {
                if let Ok(lock) = locks.get(button) {
                    if !hovered_changed {
                        bevy::log::trace!("Adding Lock hovered");
                        commands.insert_resource(Hovered::Lock(*lock));
                        hovered_changed = true;
                    }
                } else if let Ok(key) = keys.get(button) {
                    if !hovered_changed {
                        bevy::log::trace!("Adding Key hovered");
                        commands.insert_resource(Hovered::Key(*key));
                        hovered_changed = true;
                    }
                } else {
                    bevy::log::trace!("Hovering over an button that is not a Key or a Lock");
                }
            }
        }
    }
}

fn update_keys_and_locks(
    hovered: Res<Hovered>,
    mut keys: Query<(&Key, &mut BorderColor), Without<Lock>>,
    mut locks: Query<(&Lock, &mut BorderColor), Without<Key>>,
) {
    match hovered.as_ref() {
        Hovered::Lock(lock) => {
            for (key, mut border) in keys.iter_mut() {
                if usable_key_on_lock(key, lock) {
                    border.0 = palettes::tailwind::GREEN_500.into();
                } else {
                    border.0 = Color::WHITE;
                }
            }
        }
        Hovered::Key(key) => {
            for (lock, mut border) in locks.iter_mut() {
                if usable_key_on_lock(key, lock) {
                    border.0 = palettes::tailwind::GREEN_500.into();
                } else {
                    border.0 = Color::WHITE;
                }
            }
        }
    }
}

fn clear_locks(mut locks: Query<&mut BorderColor, (With<Lock>, Without<Key>)>) {
    for mut lock in locks.iter_mut() {
        lock.0 = Color::WHITE;
    }
}

fn clear_keys(mut keys: Query<&mut BorderColor, (With<Key>, Without<Lock>)>) {
    for mut lock in keys.iter_mut() {
        lock.0 = Color::WHITE;
    }
}

fn build_day_25(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = commands.spawn((Name::new("day25_camera"), Camera2d)).id();
    let day25_resource = GenericDay {
        input: asset_server.load("inputs/day25.txt"),
        camera,
        ui: commands
            .spawn((
                Name::new("day25_ui"),
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                TargetCamera(camera),
            ))
            .id(),
    };

    commands.insert_resource(ClearColor(Color::srgb_u8(0x0f, 0x0f, 0x23)));
    commands.insert_resource(day25_resource);
}

fn destroy_day_25(mut commands: Commands, day25_resource: Res<GenericDay>) {
    commands.entity(day25_resource.camera).despawn_recursive();
    commands.entity(day25_resource.ui).despawn_recursive();

    commands.remove_resource::<GenericDay>();
}

fn state_button_interactions(
    mut buttons: ButtonWithChangedInteractionQuery,
    state_changes: Query<&StateChange>,
    mut next_state: ResMut<NextState<SceneStates>>,
) {
    for (button, mut background_color, interaction) in buttons.iter_mut() {
        match interaction {
            Interaction::None => background_color.0 = BUTTON_BACKGROUND_COLOR,
            Interaction::Hovered => background_color.0 = BUTTON_HOVERED_BACKGROUND_COLOR,
            Interaction::Pressed => {
                if let Ok(state_change) = state_changes.get(button) {
                    next_state.set(state_change.0);
                }
            }
        }
    }
}

fn process_input(
    mut commands: Commands,
    day25: Res<GenericDay>,
    inputs: Res<Assets<RawInput>>,
    mut next_state: ResMut<NextState<states::InputState>>,
) {
    if let Some(input) = inputs.get(day25.input.id()) {
        commands.insert_resource(input::Input::parse(input));
        next_state.set(states::InputState::Loaded);
    }
}

fn build_ui(
    mut commands: Commands,
    day25_resource: Res<GenericDay>,
    input: Res<input::Input>,
    mut images: ResMut<Assets<Image>>,
    mut next_state: ResMut<NextState<states::UiState>>,
) {
    let header = build_header(&mut commands, "day25", false);
    let content = build_content(&mut commands, "day25");
    let footer = build_footer(&mut commands, "day25");

    commands
        .entity(content)
        .with_children(|parent| build_visualization(parent, &input, &mut images));
    commands
        .entity(day25_resource.ui)
        .add_children(&[header, content, footer]);

    next_state.set(states::UiState::Loaded);
}

fn build_visualization(
    parent: &mut ChildBuilder,
    input: &input::Input,
    images: &mut Assets<Image>,
) {
    parent
        .spawn((Node {
            top: Val::Px(50.),
            bottom: Val::Px(10.),
            left: Val::Px(10.),
            right: Val::Px(10.),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },))
        .with_children(|parent| {
            build_locks(parent, input, images);
            build_keys(parent, input, images);
        });
}

fn build_locks(parent: &mut ChildBuilder, input: &input::Input, images: &mut Assets<Image>) {
    parent
        .spawn(Node {
            height: Val::Percent(50.),
            flex_direction: FlexDirection::Column,
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(Node::default())
                .with_child((Text::new("Locks"), TextColor::WHITE));
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(2.),
                    row_gap: Val::Px(2.),
                    flex_wrap: FlexWrap::Wrap,
                    ..Default::default()
                })
                .with_children(|parent| {
                    for lock in &input.locks {
                        let mut lock_image = Image::new(
                            Extent3d {
                                width: LOCK_WIDTH * PIXEL_PER_UNIT,
                                height: LOCK_HEIGHT * PIXEL_PER_UNIT,
                                depth_or_array_layers: 1,
                            },
                            TextureDimension::D2,
                            (0..(LOCK_WIDTH * PIXEL_PER_UNIT * LOCK_HEIGHT * PIXEL_PER_UNIT))
                                .map(|i| {
                                    let index = i / PIXEL_PER_UNIT;
                                    let x = index % LOCK_WIDTH;
                                    let y = index / LOCK_WIDTH;

                                    if match x {
                                        0 => lock.0,
                                        1 => lock.1,
                                        2 => lock.2,
                                        3 => lock.3,
                                        4 => lock.4,
                                        _ => unreachable!("Should only ever be 0..5"),
                                    } >= u8::try_from(y).unwrap()
                                    {
                                        0
                                    } else {
                                        255
                                    }
                                })
                                .collect(),
                            TextureFormat::R8Unorm,
                            RenderAssetUsages::RENDER_WORLD,
                        );
                        lock_image.sampler = ImageSampler::nearest();
                        let image = images.add(lock_image);
                        parent
                            .spawn((
                                Node {
                                    border: UiRect::all(Val::Px(2.)),
                                    ..Default::default()
                                },
                                BorderColor(Color::WHITE),
                                BorderRadius::all(Val::Px(3.)),
                                Button,
                                *lock,
                            ))
                            .with_child(ImageNode {
                                image,
                                ..Default::default()
                            });
                    }
                });
        });
}

fn build_keys(parent: &mut ChildBuilder, input: &input::Input, images: &mut Assets<Image>) {
    parent
        .spawn(Node {
            height: Val::Percent(50.),
            flex_direction: FlexDirection::Column,
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(Node::default())
                .with_child((Text::new("Keys"), TextColor::WHITE));
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(2.),
                    row_gap: Val::Px(2.),
                    flex_wrap: FlexWrap::Wrap,
                    ..Default::default()
                })
                .with_children(|parent| {
                    for key in &input.keys {
                        let mut key_image = Image::new(
                            Extent3d {
                                width: LOCK_WIDTH * PIXEL_PER_UNIT,
                                height: LOCK_HEIGHT * PIXEL_PER_UNIT,
                                depth_or_array_layers: 1,
                            },
                            TextureDimension::D2,
                            (0..(LOCK_WIDTH * PIXEL_PER_UNIT * LOCK_HEIGHT * PIXEL_PER_UNIT))
                                .map(|i| {
                                    let index = i / PIXEL_PER_UNIT;
                                    let x = index % LOCK_WIDTH;
                                    let y = index / LOCK_WIDTH;

                                    if match x {
                                        0 => key.0,
                                        1 => key.1,
                                        2 => key.2,
                                        3 => key.3,
                                        4 => key.4,
                                        _ => unreachable!("Should only ever be 0..5"),
                                    } >= u8::try_from(LOCK_HEIGHT - y).unwrap()
                                    {
                                        0
                                    } else {
                                        255
                                    }
                                })
                                .collect(),
                            TextureFormat::R8Unorm,
                            RenderAssetUsages::RENDER_WORLD,
                        );
                        key_image.sampler = ImageSampler::nearest();
                        let image = images.add(key_image);
                        parent
                            .spawn((
                                Node {
                                    border: UiRect::all(Val::Px(2.)),
                                    ..Default::default()
                                },
                                BorderColor(Color::WHITE),
                                BorderRadius::all(Val::Px(3.)),
                                Button,
                                *key,
                            ))
                            .with_child(ImageNode {
                                image,
                                ..Default::default()
                            });
                    }
                });
        });
}
