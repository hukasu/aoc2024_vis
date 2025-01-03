mod components;
mod operation;
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
        IntoSystemConfigs, OnEnter, OnExit, Res,
    },
    ui::{FlexDirection, Node, Val},
};

use crate::{loader::Input as InputAsset, scenes::states::States as SceneStates};

use super::state_button_interactions;

use self::resources::{Day24, Input};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((part1::Plugin, part2::Plugin));

        app.add_sub_state::<states::States>();

        app.add_systems(OnEnter(SceneStates::Day(24)), build_day_24);
        app.add_systems(OnExit(SceneStates::Day(24)), destroy_day_24);
        app.add_systems(
            Update,
            state_button_interactions::<states::States>.run_if(in_state(SceneStates::Day(24))),
        );
    }
}

fn build_day_24(mut commands: Commands, asset_server: Res<AssetServer>) {
    let day24_resource = resources::Day24 {
        input: asset_server.load("inputs/day24.txt"),
        camera: commands.spawn((Name::new("day24_camera"), Camera2d)).id(),
        ui: commands
            .spawn((
                Name::new("day24_ui"),
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
            ))
            .id(),
    };

    commands.insert_resource(ClearColor(Color::srgb_u8(0x0f, 0x0f, 0x23)));
    commands.insert_resource(day24_resource);
}

fn destroy_day_24(mut commands: Commands, day24_resource: Res<resources::Day24>) {
    commands.entity(day24_resource.camera).despawn_recursive();
    commands.entity(day24_resource.ui).despawn_recursive();

    commands.remove_resource::<resources::Day24>();
}

fn process_input(mut commands: Commands, day24: Res<Day24>, inputs: Res<Assets<InputAsset>>) {
    if let Some(input) = inputs.get(day24.input.id()) {
        commands.insert_resource(Input::parse(input));
    }
}
