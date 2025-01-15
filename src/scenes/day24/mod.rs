mod components;
mod input;
mod operation;
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

use crate::{loader::RawInput, scenes::states::Scene};

use super::{
    resources::GenericDay,
    state_button_interactions,
    states::{InputState, Part, VisualizationState},
};

use self::input::Input;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((part1::Plugin, part2::Plugin));

        app.add_computed_state::<VisualizationState<24>>();

        app.add_systems(OnEnter(Scene::Day(24)), build_day_24)
            .add_systems(OnExit(Scene::Day(24)), destroy_day_24)
            .add_systems(
                Update,
                state_button_interactions::<Part>.run_if(in_state(Scene::Day(24))),
            )
            .add_systems(
                Update,
                process_input.run_if(in_state(VisualizationState::<24>::WaitingInput)),
            );
    }
}

fn build_day_24(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = commands.spawn((Name::new("day24_camera"), Camera2d)).id();
    let day24_resource = GenericDay {
        input: asset_server.load("inputs/day24.txt"),
        camera,
        ui: commands
            .spawn((
                Name::new("day24_ui"),
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
    commands.insert_resource(day24_resource);
}

fn destroy_day_24(mut commands: Commands, day24_resource: Res<GenericDay>) {
    commands.entity(day24_resource.camera).despawn_recursive();
    commands.entity(day24_resource.ui).despawn_recursive();

    commands.remove_resource::<GenericDay>();
}

fn process_input(
    mut commands: Commands,
    day24: Res<GenericDay>,
    inputs: Res<Assets<RawInput>>,
    mut next_state: ResMut<NextState<InputState>>,
) {
    if let Some(input) = inputs.get(day24.input.id()) {
        commands.insert_resource(Input::parse(input));
        next_state.set(InputState::Loaded);
    }
}
