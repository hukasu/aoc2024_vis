mod claw_machine;
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
use claw_machine::SelectedClawMachine;

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
        app.add_plugins((part1::Plugin, part2::Plugin, claw_machine::Plugin));

        app.add_computed_state::<VisualizationState<13>>();

        app.add_systems(OnEnter(Scene::Day(13)), build_day_13)
            .add_systems(OnExit(Scene::Day(13)), destroy_day_13)
            .add_systems(
                Update,
                process_input.run_if(in_state(VisualizationState::<13>::WaitingInput)),
            )
            .add_systems(
                Update,
                state_button_interactions.run_if(in_state(Scene::Day(13))),
            );
    }
}

fn build_day_13(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = commands.spawn((Name::new("day13_camera"), Camera2d)).id();
    let day13_resource = GenericDay {
        input: asset_server.load("inputs/day13.txt"),
        camera,
        ui: commands
            .spawn((
                Name::new("day13_ui"),
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
    commands.insert_resource(day13_resource);
    commands.init_resource::<SelectedClawMachine>();
}

fn destroy_day_13(mut commands: Commands, day13_resource: Res<GenericDay>) {
    commands.entity(day13_resource.camera).despawn_recursive();
    commands.entity(day13_resource.ui).despawn_recursive();

    commands.remove_resource::<GenericDay>();
    commands.remove_resource::<SelectedClawMachine>();
}

fn process_input(
    mut commands: Commands,
    day13_resource: Res<GenericDay>,
    inputs: Res<Assets<InputAsset>>,
    mut next_state: ResMut<NextState<InputState>>,
) {
    if let Some(input) = inputs.get(day13_resource.input.id()) {
        commands.insert_resource(Input::parse(input));
        next_state.set(InputState::Loaded);
    }
}
