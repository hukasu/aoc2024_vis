mod input;
mod part1;
mod part2;

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
        app.add_plugins((part1::Plugin, part2::Plugin));

        app.add_computed_state::<VisualizationState<18>>();

        app.add_systems(OnEnter(Scene::Day(18)), build_day_18)
            .add_systems(OnExit(Scene::Day(18)), destroy_day_18)
            .add_systems(
                Update,
                process_input.run_if(in_state(VisualizationState::<18>::WaitingInput)),
            )
            .add_systems(
                Update,
                state_button_interactions.run_if(in_state(Scene::Day(18))),
            );
    }
}

fn build_day_18(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = commands.spawn((Name::new("day18_camera"), Camera2d)).id();
    let day18_resource = GenericDay {
        input: asset_server.load("inputs/day18.txt"),
        camera,
        ui: commands
            .spawn((
                Name::new("day18_ui"),
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
    commands.insert_resource(day18_resource);
}

fn destroy_day_18(mut commands: Commands, day18_resource: Res<GenericDay>) {
    commands.entity(day18_resource.camera).despawn_recursive();
    commands.entity(day18_resource.ui).despawn_recursive();

    commands.remove_resource::<GenericDay>();
}

fn process_input(
    mut commands: Commands,
    day18_resource: Res<GenericDay>,
    inputs: Res<Assets<InputAsset>>,
    mut next_state: ResMut<NextState<InputState>>,
) {
    if let Some(input) = inputs.get(day18_resource.input.id()) {
        commands.insert_resource(Input::parse(input));
        next_state.set(InputState::Loaded);
    }
}
