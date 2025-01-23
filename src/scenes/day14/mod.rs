mod input;
mod ui;

use bevy::{
    app::Update,
    asset::{AssetServer, Assets},
    color::Color,
    core::Name,
    prelude::{
        in_state, AppExtStates, Camera2d, ClearColor, Commands, DespawnRecursiveExt,
        IntoSystemConfigs, NextState, OnEnter, OnExit, Res, ResMut,
    },
    ui::{FlexDirection, Node, TargetCamera, Val},
};
use input::Input;

use crate::loader::RawInput as InputAsset;

use super::{
    resources::GenericDay,
    state_button_interactions,
    states::{InputState, Scene, VisualizationState},
};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(ui::Plugin);

        app.add_computed_state::<VisualizationState<14>>();

        app.add_systems(OnEnter(Scene::Day(14)), build_day_14)
            .add_systems(OnExit(Scene::Day(14)), destroy_day_14)
            .add_systems(
                Update,
                process_input.run_if(in_state(VisualizationState::<14>::WaitingInput)),
            )
            .add_systems(
                Update,
                state_button_interactions.run_if(in_state(Scene::Day(14))),
            );
    }
}

fn build_day_14(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = commands.spawn((Name::new("day14_camera"), Camera2d)).id();
    let day14_resource = GenericDay {
        input: asset_server.load("inputs/day14.txt"),
        camera,
        ui: commands
            .spawn((
                Name::new("day14_ui"),
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
    commands.insert_resource(day14_resource);
}

fn destroy_day_14(mut commands: Commands, day14_resource: Res<GenericDay>) {
    commands.entity(day14_resource.camera).despawn_recursive();
    commands.entity(day14_resource.ui).despawn_recursive();

    commands.remove_resource::<GenericDay>();
}

fn process_input(
    mut commands: Commands,
    day14_resource: Res<GenericDay>,
    inputs: Res<Assets<InputAsset>>,
    mut next_state: ResMut<NextState<InputState>>,
) {
    if let Some(input) = inputs.get(day14_resource.input.id()) {
        commands.insert_resource(Input::parse(input));
        next_state.set(InputState::Loaded);
    }
}
