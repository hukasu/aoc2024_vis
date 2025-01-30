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

        app.add_computed_state::<VisualizationState<19>>();

        app.add_systems(OnEnter(Scene::Day(19)), build_day_19)
            .add_systems(OnExit(Scene::Day(19)), destroy_day_19)
            .add_systems(
                Update,
                process_input.run_if(in_state(VisualizationState::<19>::WaitingInput)),
            )
            .add_systems(
                Update,
                state_button_interactions.run_if(in_state(Scene::Day(19))),
            );
    }
}

fn build_day_19(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = commands.spawn((Name::new("day19_camera"), Camera2d)).id();
    let day19_resource = GenericDay {
        input: asset_server.load("inputs/day19.txt"),
        camera,
        ui: commands
            .spawn((
                Name::new("day19_ui"),
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
    commands.insert_resource(day19_resource);
}

fn destroy_day_19(mut commands: Commands, day19_resource: Res<GenericDay>) {
    commands.entity(day19_resource.camera).despawn_recursive();
    commands.entity(day19_resource.ui).despawn_recursive();

    commands.remove_resource::<GenericDay>();
    commands.remove_resource::<ui::SelectedPattern>();
}

fn process_input(
    mut commands: Commands,
    day19_resource: Res<GenericDay>,
    inputs: Res<Assets<InputAsset>>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<InputState>>,
) {
    if let Some(input) = inputs.get(day19_resource.input.id()) {
        commands.insert_resource(Input::parse(input, &asset_server));
        next_state.set(InputState::Loaded);
    }
}
