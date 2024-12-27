use bevy::{
    app::Update,
    core::Name,
    prelude::{
        in_state, not, resource_exists, BuildChildren, ChildBuild, ChildBuilder, Commands,
        DespawnRecursiveExt, IntoSystemConfigs, OnEnter, OnExit, Res,
    },
    ui::{FlexDirection, Node, UiRect, Val},
};

use super::resources::Input;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            OnEnter(super::states::States::Part2),
            build_ui.after(super::build_day_24),
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

fn build_ui(mut commands: Commands, day24_resource: Res<super::resources::Day24>) {
    commands.entity(day24_resource.ui).with_children(build_divs);
}

fn destroy_ui(mut commands: Commands, day24_resource: Res<super::resources::Day24>) {
    commands.remove_resource::<Input>();
    commands.entity(day24_resource.ui).despawn_descendants();
}

fn build_divs(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Name::new("day24_states"),
            Node {
                top: Val::Px(0.),
                left: Val::Px(0.),
                padding: UiRect {
                    left: Val::Px(10.),
                    top: Val::Px(10.),
                    bottom: Val::Px(10.),
                    ..Default::default()
                },
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(5.),
                ..Default::default()
            },
        ))
        .with_children(super::part1::build_state_buttons);
}
