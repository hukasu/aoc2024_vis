mod input;
mod states;
mod ui;

use bevy::{
    app::Update,
    asset::{AssetServer, Assets},
    color::Color,
    core::Name,
    prelude::{
        in_state, AppExtStates, Button, Camera2d, Changed, ClearColor, Commands,
        DespawnRecursiveExt, Entity, IntoSystemConfigs, NextState, OnEnter, OnExit, Query, Res,
        ResMut, With,
    },
    ui::{BackgroundColor, FlexDirection, Interaction, Node, TargetCamera, Val},
};

use crate::{
    loader::RawInput,
    scroll_controls::{BUTTON_BACKGROUND_COLOR, BUTTON_HOVERED_BACKGROUND_COLOR},
};

use input::Input;

use super::{components::StateChange, resources::GenericDay};

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(ui::Plugin);

        app.add_sub_state::<states::InputState>()
            .add_sub_state::<states::UiState>()
            .add_computed_state::<states::VisualizationState>();

        app.add_systems(OnEnter(super::states::States::Day(2)), build_day_2)
            .add_systems(OnExit(super::states::States::Day(2)), destroy_day_2)
            .add_systems(
                Update,
                process_input.run_if(in_state(states::VisualizationState::WaitingInput)),
            )
            .add_systems(
                Update,
                state_button_interactions.run_if(in_state(super::states::States::Day(2))),
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
    mut next_state: ResMut<NextState<states::InputState>>,
) {
    if let Some(input) = inputs.get(day2_resource.input.id()) {
        commands.insert_resource(Input::parse(input));
        next_state.set(states::InputState::Loaded);
    }
}

type ButtonWithChangedInteractionQuery<'a, 'b> = Query<
    'a,
    'b,
    (Entity, &'static mut BackgroundColor, &'static Interaction),
    (With<Button>, Changed<Interaction>),
>;

fn state_button_interactions(
    mut buttons: ButtonWithChangedInteractionQuery,
    state_changes: Query<&StateChange>,
    mut next_state: ResMut<NextState<super::states::States>>,
) {
    for (button, mut background_color, interaction) in buttons.iter_mut() {
        match interaction {
            Interaction::None => background_color.0 = BUTTON_BACKGROUND_COLOR,
            Interaction::Hovered => background_color.0 = BUTTON_HOVERED_BACKGROUND_COLOR,
            Interaction::Pressed => {
                if let Ok(state_change) = state_changes.get(button) {
                    next_state.set(state_change.0);
                }
            }
        }
    }
}
