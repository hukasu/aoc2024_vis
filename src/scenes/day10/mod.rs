mod components;
mod input;
mod part1;
mod part2;
mod resources;

use bevy::{
    app::Update,
    asset::{AssetServer, Assets},
    color::Color,
    core::Name,
    prelude::{
        apply_deferred, in_state, resource_exists_and_changed, resource_removed, AppExtStates,
        BuildChildren, Camera2d, Changed, ClearColor, Commands, Condition, DespawnRecursiveExt,
        Entity, IntoSystemConfigs, NextState, OnEnter, OnExit, Query, Res, ResMut, Text, With,
    },
    text::{TextColor, TextFont},
    ui::{FlexDirection, Interaction, Node, TargetCamera, Val},
};
use components::{PartOfTrail, Start};
use input::Input;
use resources::HoveredTile;

use crate::loader::RawInput as InputAsset;

use super::{
    resources::GenericDay,
    state_button_interactions,
    states::{InputState, Scene, VisualizationState},
};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((part1::Plugin, part2::Plugin));

        app.add_computed_state::<VisualizationState<10>>();

        app.add_systems(OnEnter(Scene::Day(10)), build_day_10)
            .add_systems(OnExit(Scene::Day(10)), destroy_day_10)
            .add_systems(
                Update,
                process_input.run_if(in_state(VisualizationState::<10>::WaitingInput)),
            )
            .add_systems(
                Update,
                state_button_interactions.run_if(in_state(Scene::Day(10))),
            )
            .add_systems(
                Update,
                (hovered_start, apply_deferred)
                    .chain()
                    .run_if(in_state(VisualizationState::<10>::Ready)),
            )
            .add_systems(
                Update,
                clear_trails.after(hovered_start).run_if(
                    in_state(VisualizationState::<10>::Ready).and(resource_removed::<HoveredTile>),
                ),
            )
            .add_systems(
                Update,
                update_trails.after(clear_trails).run_if(
                    in_state(VisualizationState::<10>::Ready)
                        .and(resource_exists_and_changed::<HoveredTile>),
                ),
            );
    }
}

fn build_day_10(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = commands.spawn((Name::new("day10_camera"), Camera2d)).id();
    let day10_resource = GenericDay {
        input: asset_server.load("inputs/day10.txt"),
        camera,
        ui: commands
            .spawn((
                Name::new("day10_ui"),
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
    commands.insert_resource(day10_resource);
}

fn destroy_day_10(mut commands: Commands, day10_resource: Res<GenericDay>) {
    commands.entity(day10_resource.camera).despawn_recursive();
    commands.entity(day10_resource.ui).despawn_recursive();

    commands.remove_resource::<GenericDay>();
}

fn process_input(
    mut commands: Commands,
    day10_resource: Res<GenericDay>,
    inputs: Res<Assets<InputAsset>>,
    mut next_state: ResMut<NextState<InputState>>,
) {
    if let Some(input) = inputs.get(day10_resource.input.id()) {
        commands.insert_resource(Input::parse(input));
        next_state.set(InputState::Loaded);
    }
}

fn hovered_start(
    mut commands: Commands,
    starts: Query<(&Interaction, &Start), Changed<Interaction>>,
) {
    if starts.is_empty() {
        return;
    }

    let mut new_start = None;
    for (interaction, start) in starts.iter() {
        match interaction {
            Interaction::None => (),
            Interaction::Hovered | Interaction::Pressed => {
                new_start.replace(**start);
            }
        }
    }

    if let Some(new_start) = new_start {
        commands.insert_resource(HoveredTile(new_start));
    } else {
        commands.remove_resource::<HoveredTile>();
    }
}

fn clear_trails(mut commands: Commands, part_of_trails: Query<Entity, With<PartOfTrail>>) {
    for entity in part_of_trails.iter() {
        commands.entity(entity).despawn_descendants();
    }
}

fn update_trails(
    mut commands: Commands,
    part_of_trails: Query<(Entity, &PartOfTrail)>,
    hovered_trail_head: Res<HoveredTile>,
    input: Res<Input>,
) {
    for (entity, part_of_trail) in part_of_trails.iter() {
        commands.entity(entity).despawn_descendants();
        if part_of_trail.starts.contains(&**hovered_trail_head) {
            if part_of_trail.is_end {
                commands.entity(entity).with_child((
                    Node {
                        align_self: bevy::ui::AlignSelf::Center,
                        justify_self: bevy::ui::JustifySelf::Center,
                        ..Default::default()
                    },
                    Text::new("E"),
                    TextColor(Color::BLACK),
                    TextFont {
                        font_size: 12.,
                        ..Default::default()
                    },
                ));
            } else {
                commands.entity(entity).despawn_descendants().with_child((
                    Node {
                        align_self: bevy::ui::AlignSelf::Center,
                        justify_self: bevy::ui::JustifySelf::Center,
                        ..Default::default()
                    },
                    Text::new(
                        input.tiles[part_of_trail.coord.1][part_of_trail.coord.0].to_string(),
                    ),
                    TextColor(Color::BLACK),
                    TextFont {
                        font_size: 12.,
                        ..Default::default()
                    },
                ));
            }
        }
    }
}
