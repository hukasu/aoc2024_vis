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

use crate::{loader::RawInput, scenes::states::Scene};

use super::{
    resources::GenericDay,
    state_button_interactions,
    states::{InputState, VisualizationState},
};

use self::input::Input;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((part1::Plugin, part2::Plugin));

        app.add_computed_state::<VisualizationState<1>>();

        app.add_systems(OnEnter(Scene::Day(1)), build_day_1);
        app.add_systems(OnExit(Scene::Day(1)), destroy_day_1);
        app.add_systems(
            Update,
            state_button_interactions.run_if(in_state(Scene::Day(1))),
        )
        .add_systems(
            Update,
            process_input.run_if(in_state(VisualizationState::<1>::WaitingInput)),
        );
    }
}

fn build_day_1(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = commands.spawn((Name::new("day1_camera"), Camera2d)).id();
    let day1_resource = GenericDay {
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

fn destroy_day_1(mut commands: Commands, day1_resource: Res<GenericDay>) {
    commands.entity(day1_resource.camera).despawn_recursive();
    commands.entity(day1_resource.ui).despawn_recursive();

    commands.remove_resource::<GenericDay>();
}

fn process_input(
    mut commands: Commands,
    day1_resource: Res<GenericDay>,
    inputs: Res<Assets<RawInput>>,
    mut next_state: ResMut<NextState<InputState>>,
) {
    if let Some(input) = inputs.get(day1_resource.input.id()) {
        commands.insert_resource(Input::parse(input));
        next_state.set(InputState::Loaded);
    }
}
