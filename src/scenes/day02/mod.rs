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

use crate::loader::RawInput;

use input::Input;

use super::{
    resources::GenericDay,
    state_button_interactions,
    states::{InputState, Part, Scene, VisualizationState},
};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(ui::Plugin);

        app.add_computed_state::<VisualizationState<2>>();

        app.add_systems(OnEnter(Scene::Day(2)), build_day_2)
            .add_systems(OnExit(Scene::Day(2)), destroy_day_2)
            .add_systems(
                Update,
                process_input.run_if(in_state(VisualizationState::<2>::WaitingInput)),
            )
            .add_systems(
                Update,
                state_button_interactions::<Part>.run_if(in_state(Scene::Day(2))),
            );
    }
}

fn build_day_2(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = commands.spawn((Name::new("day2_camera"), Camera2d)).id();
    let day2_resource = GenericDay {
        input: asset_server.load("inputs/day2.txt"),
        camera,
        ui: commands
            .spawn((
                Name::new("day2_ui"),
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
    commands.insert_resource(day2_resource);
}

fn destroy_day_2(mut commands: Commands, day2_resource: Res<GenericDay>) {
    commands.entity(day2_resource.camera).despawn_recursive();
    commands.entity(day2_resource.ui).despawn_recursive();

    commands.remove_resource::<GenericDay>();
}

fn process_input(
    mut commands: Commands,
    day2_resource: Res<GenericDay>,
    inputs: Res<Assets<RawInput>>,
    mut next_state: ResMut<NextState<InputState>>,
) {
    if let Some(input) = inputs.get(day2_resource.input.id()) {
        commands.insert_resource(Input::parse(input));
        next_state.set(InputState::Loaded);
    }
}
