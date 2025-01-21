use bevy::{
    app::Update,
    asset::Handle,
    color::Color,
    prelude::{
        in_state, AppExtStates, BuildChildren, Button, Changed, ChildBuild, ChildBuilder,
        Component, IntoSystemConfigs, NextState, Query, ResMut, StateSet, SubStates, Text, With,
    },
    text::{Font, TextColor, TextFont},
    ui::{BackgroundColor, Interaction},
};

use crate::{
    scenes::{
        days::button_node,
        states::{Scene, VisualizationState},
    },
    scroll_controls::{BUTTON_BACKGROUND_COLOR, BUTTON_HOVERED_BACKGROUND_COLOR},
};

use super::input::Input;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_sub_state::<ControlState>();

        app.add_systems(
            Update,
            controls_interaction.run_if(in_state(VisualizationState::<9>::Ready)),
        );
    }
}

type ControlWithChangedInteractionQuery<'a, 'b> = Query<
    'a,
    'b,
    (
        &'static mut BackgroundColor,
        &'static Interaction,
        &'static Control,
    ),
    (With<Button>, Changed<Interaction>),
>;

fn controls_interaction(
    mut controls: ControlWithChangedInteractionQuery,
    mut input: ResMut<Input>,
    mut next_state: ResMut<NextState<ControlState>>,
) {
    for (mut background_color, interaction, control) in controls.iter_mut() {
        match interaction {
            Interaction::None => background_color.0 = BUTTON_BACKGROUND_COLOR,
            Interaction::Hovered => background_color.0 = BUTTON_HOVERED_BACKGROUND_COLOR,
            Interaction::Pressed => match control {
                Control::Reset => input.reset(),
                Control::Play => {
                    next_state.set(ControlState::Playing);
                }
                Control::Pause => {
                    next_state.set(ControlState::Paused);
                }
            },
        }
    }
}

#[derive(Debug, Component)]
#[require(Button)]
enum Control {
    Reset,
    Play,
    Pause,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SubStates)]
#[source(Scene = Scene::Day(9))]
pub enum ControlState {
    Playing,
    #[default]
    Paused,
}

pub fn build_control(parent: &mut ChildBuilder, symbol: Handle<Font>) {
    parent
        .spawn((
            button_node(),
            Control::Reset,
            BackgroundColor(BUTTON_BACKGROUND_COLOR),
        ))
        .with_child((
            Text::new("⏮"),
            TextFont {
                font: symbol.clone(),
                ..Default::default()
            },
            TextColor(Color::BLACK),
        ));
    parent
        .spawn((
            button_node(),
            Control::Play,
            BackgroundColor(BUTTON_BACKGROUND_COLOR),
        ))
        .with_child((
            Text::new("⏵"),
            TextFont {
                font: symbol.clone(),

                ..Default::default()
            },
            TextColor(Color::BLACK),
        ));
    parent
        .spawn((
            button_node(),
            Control::Pause,
            BackgroundColor(BUTTON_BACKGROUND_COLOR),
        ))
        .with_child((
            Text::new("⏸"),
            TextFont {
                font: symbol.clone(),
                ..Default::default()
            },
            TextColor(Color::BLACK),
        ));
}
