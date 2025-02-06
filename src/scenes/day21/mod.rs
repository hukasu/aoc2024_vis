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

        app.add_computed_state::<VisualizationState<21>>();

        app.add_systems(OnEnter(Scene::Day(21)), build_day_21)
            .add_systems(OnExit(Scene::Day(21)), destroy_day_21)
            .add_systems(
                Update,
                process_input.run_if(in_state(VisualizationState::<21>::WaitingInput)),
            )
            .add_systems(
                Update,
                state_button_interactions.run_if(in_state(Scene::Day(21))),
            );
    }
}

fn build_day_21(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = commands.spawn((Name::new("day21_camera"), Camera2d)).id();
    let day21_resource = GenericDay {
        input: asset_server.load("inputs/day21.txt"),
        camera,
        ui: commands
            .spawn((
                Name::new("day21_ui"),
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
    commands.insert_resource(day21_resource);
}

fn destroy_day_21(mut commands: Commands, day21_resource: Res<GenericDay>) {
    commands.entity(day21_resource.camera).despawn_recursive();
    commands.entity(day21_resource.ui).despawn_recursive();

    commands.remove_resource::<GenericDay>();
}

fn process_input(
    mut commands: Commands,
    day21_resource: Res<GenericDay>,
    inputs: Res<Assets<InputAsset>>,
    mut next_state: ResMut<NextState<InputState>>,
) {
    if let Some(input) = inputs.get(day21_resource.input.id()) {
        commands.insert_resource(Input::parse(input));
        next_state.set(InputState::Loaded);
    }
}
