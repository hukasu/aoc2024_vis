mod input;
mod ui;

use bevy::{
    app::Update,
    asset::{AssetServer, Assets},
    color::Color,
    core::Name,
    prelude::{
        in_state, AppExtStates, BuildChildren, Camera2d, ClearColor, Commands, DespawnRecursiveExt,
        IntoSystemConfigs, NextState, OnEnter, OnExit, Res, ResMut, Text,
    },
    text::TextFont,
    ui::{FlexDirection, Node, TargetCamera, Val},
};
use input::Input;

use crate::loader::RawInput as InputAsset;

use super::{
    resources::GenericDay,
    state_button_interactions,
    states::{InputState, Part, Scene, VisualizationState},
};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(ui::Plugin);

        app.add_computed_state::<VisualizationState<6>>();

        app.add_systems(OnEnter(Scene::Day(6)), build_day_6)
            .add_systems(OnExit(Scene::Day(6)), destroy_day_6)
            .add_systems(
                Update,
                process_input.run_if(in_state(VisualizationState::<6>::WaitingInput)),
            )
            .add_systems(
                Update,
                state_button_interactions::<Part>.run_if(in_state(Scene::Day(6))),
            );
    }
}

fn build_day_6(mut commands: Commands, asset_server: Res<AssetServer>) {
    let camera = commands.spawn((Name::new("day6_camera"), Camera2d)).id();
    let day6_resource = GenericDay {
        input: asset_server.load("inputs/day6.txt"),
        camera,
        ui: commands
            .spawn((
                Name::new("day6_ui"),
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

    commands.entity(day6_resource.ui).with_child((
        Text::new("Loading"),
        TextFont {
            font_size: 64.,
            ..Default::default()
        },
    ));

    commands.insert_resource(ClearColor(Color::srgb_u8(0x0f, 0x0f, 0x23)));
    commands.insert_resource(day6_resource);
}

fn destroy_day_6(mut commands: Commands, day6_resource: Res<GenericDay>) {
    commands.entity(day6_resource.camera).despawn_recursive();
    commands.entity(day6_resource.ui).despawn_recursive();

    commands.remove_resource::<GenericDay>();
}

fn process_input(
    mut commands: Commands,
    day6_resource: Res<GenericDay>,
    inputs: Res<Assets<InputAsset>>,
    mut next_state: ResMut<NextState<InputState>>,
) {
    if let Some(input) = inputs.get(day6_resource.input.id()) {
        commands.insert_resource(Input::parse(input));
        next_state.set(InputState::Loaded);
    }
}
