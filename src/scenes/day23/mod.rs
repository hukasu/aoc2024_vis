mod input;
mod ui;

use bevy::{
    app::Update,
    asset::{AssetServer, Assets},
    color::Color,
    core::Name,
    prelude::{
        in_state, AppExtStates, Camera, Camera2d, ClearColor, Commands, Component,
        DespawnRecursiveExt, Entity, IntoSystemConfigs, NextState, OnEnter, OnExit, Query, Res,
        ResMut, With,
    },
    render::view::RenderLayers,
    ui::{FlexDirection, Node, TargetCamera, Val},
};

use crate::loader::RawInput as InputAsset;

use super::{
    resources::GenericDay,
    state_button_interactions,
    states::{InputState, Scene, VisualizationState},
};

use self::input::Input;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(ui::Plugin);

        app.add_computed_state::<VisualizationState<23>>();

        app.add_systems(OnEnter(Scene::Day(23)), build_day_23)
            .add_systems(OnExit(Scene::Day(23)), destroy_day_23)
            .add_systems(
                Update,
                process_input.run_if(in_state(VisualizationState::<23>::WaitingInput)),
            )
            .add_systems(
                Update,
                state_button_interactions.run_if(in_state(Scene::Day(23))),
            )
            .add_systems(OnEnter(Scene::Day(23)), spawn_gizmos_camera)
            .add_systems(OnExit(Scene::Day(23)), despawn_gizmos_camera);
    }
}

fn build_day_23(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = commands.spawn((Name::new("day23_camera"), Camera2d)).id();
    let day23_resource = GenericDay {
        input: asset_server.load("inputs/day23.txt"),
        camera,
        ui: commands
            .spawn((
                Name::new("day23_ui"),
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
    commands.insert_resource(day23_resource);
}

fn destroy_day_23(mut commands: Commands, day23_resource: Res<GenericDay>) {
    commands.entity(day23_resource.camera).despawn_recursive();
    commands.entity(day23_resource.ui).despawn_recursive();

    commands.remove_resource::<GenericDay>();
}

fn spawn_gizmos_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("day23_gizmos_camera"),
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

fn process_input(
    mut commands: Commands,
    day23_resource: Res<GenericDay>,
    inputs: Res<Assets<InputAsset>>,
    mut next_state: ResMut<NextState<InputState>>,
) {
    if let Some(input) = inputs.get(day23_resource.input.id()) {
        commands.insert_resource(Input::parse(input));
        next_state.set(InputState::Loaded);
    }
}

#[derive(Debug, Component)]
pub struct GizmosCamera;
