use bevy::{
    app::Update,
    prelude::{
        in_state, not, resource_added, resource_exists, BuildChildren, Commands,
        DespawnRecursiveExt, IntoSystemConfigs, OnExit, Res,
    },
};

use crate::scenes::days::{build_content, build_footer, build_header};

use super::{resources::Input, states::States};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            build_ui
                .run_if(in_state(States::Part2))
                .run_if(resource_added::<Input>),
        )
        .add_systems(OnExit(super::states::States::Part2), destroy_ui)
        .add_systems(
            Update,
            super::process_input
                .run_if(not(resource_exists::<Input>))
                .run_if(in_state(super::states::States::Part2)),
        );
    }
}

fn build_ui(
    mut commands: Commands,
    day24_resource: Res<super::resources::Day24>,
    input: Res<Input>,
) {
    bevy::log::trace!("Day 24 Part 2");

    let header = build_header(&mut commands, "day24");
    let content = build_content(&mut commands, "day24");
    let footer = build_footer(&mut commands, "day24");

    commands
        .entity(day24_resource.ui)
        .add_children(&[header, content, footer]);
}

fn destroy_ui(mut commands: Commands, day24_resource: Res<super::resources::Day24>) {
    commands.remove_resource::<Input>();
    commands.entity(day24_resource.ui).despawn_descendants();
}
