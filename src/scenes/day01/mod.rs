mod input;
mod part1;
mod part2;
mod resources;
mod states;

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

use crate::{loader::RawInput, scenes::states::States as SceneStates};

use super::state_button_interactions;

use self::{input::Input, resources::Day01};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((part1::Plugin, part2::Plugin));

        app.add_sub_state::<states::States>()
            .add_sub_state::<states::InputState>()
            .add_sub_state::<states::UiState>()
            .add_computed_state::<states::VisualizationState>();

        app.add_systems(OnEnter(SceneStates::Day(1)), build_day_1);
        app.add_systems(OnExit(SceneStates::Day(1)), destroy_day_1);
        app.add_systems(
            Update,
            state_button_interactions::<states::States>.run_if(in_state(SceneStates::Day(1))),
        );
    }
}

fn build_day_1(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = commands.spawn((Name::new("day1_camera"), Camera2d)).id();
    let day1_resource = resources::Day01 {
        input: asset_server.load("inputs/day1.txt"),
        camera,
        ui: commands
            .spawn((
                Name::new("day1_ui"),
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
    commands.insert_resource(day1_resource);
}

fn destroy_day_1(mut commands: Commands, day1_resource: Res<resources::Day01>) {
    commands.entity(day1_resource.camera).despawn_recursive();
    commands.entity(day1_resource.ui).despawn_recursive();

    commands.remove_resource::<resources::Day01>();
}

fn process_input(
    mut commands: Commands,
    day1_resource: Res<Day01>,
    inputs: Res<Assets<RawInput>>,
    mut next_state: ResMut<NextState<states::InputState>>,
) {
    if let Some(input) = inputs.get(day1_resource.input.id()) {
        commands.insert_resource(Input::parse(input));
        next_state.set(states::InputState::Loaded);
    }
}
