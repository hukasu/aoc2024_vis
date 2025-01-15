mod components;
mod day01;
mod day02;
mod day03;
mod day04;
mod day24;
mod day25;
mod days;
mod main_menu;
mod resources;
mod states;

use std::sync::OnceLock;

use bevy::{
    app::Startup,
    asset::{AssetServer, Handle},
    prelude::{AppExtStates, Button, Changed, Entity, NextState, Query, Res, ResMut, With},
    state::state::FreelyMutableState,
    text::Font,
    ui::{BackgroundColor, Interaction},
};
use components::{PartChange, SceneChange};

use crate::scroll_controls::{BUTTON_BACKGROUND_COLOR, BUTTON_HOVERED_BACKGROUND_COLOR};

static FONT_HANDLE: OnceLock<Handle<Font>> = OnceLock::new();
static FONT_SYMBOLS_HANDLE: OnceLock<Handle<Font>> = OnceLock::new();
static FONT_SYMBOLS_2_HANDLE: OnceLock<Handle<Font>> = OnceLock::new();

type ButtonWithChangedInteractionQuery<'a, 'b> = Query<
    'a,
    'b,
    (Entity, &'static mut BackgroundColor, &'static Interaction),
    (With<Button>, Changed<Interaction>),
>;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((
            main_menu::Plugin,
            day01::Plugin,
            day02::Plugin,
            day03::Plugin,
            day04::Plugin,
            day24::Plugin,
            day25::Plugin,
        ));

        app.init_state::<states::Scene>()
            .add_sub_state::<states::Part>()
            .add_sub_state::<states::InputState>()
            .add_sub_state::<states::UiState>();

        app.add_systems(Startup, load_font);
    }
}

fn load_font(asset_server: Res<AssetServer>) {
    if FONT_HANDLE
        .set(asset_server.load("NotoSans-VariableFont_wdth,wght.ttf"))
        .is_err()
    {
        bevy::log::error!("Failed to load font.");
    };
    if FONT_SYMBOLS_HANDLE
        .set(asset_server.load("NotoSansSymbols-VariableFont_wght.ttf"))
        .is_err()
    {
        bevy::log::error!("Failed to load symbols font.");
    };
    if FONT_SYMBOLS_2_HANDLE
        .set(asset_server.load("NotoSansSymbols2-Regular.ttf"))
        .is_err()
    {
        bevy::log::error!("Failed to load symbols font.");
    };
}

fn state_button_interactions<T>(
    mut buttons: ButtonWithChangedInteractionQuery,
    state_changes: Query<&SceneChange>,
    part_changes: Query<&PartChange>,
    mut next_state: ResMut<NextState<states::Scene>>,
    mut part_next_state: ResMut<NextState<T>>,
) where
    T: FreelyMutableState + From<PartChange>,
{
    for (button, mut background_color, interaction) in buttons.iter_mut() {
        match interaction {
            Interaction::None => background_color.0 = BUTTON_BACKGROUND_COLOR,
            Interaction::Hovered => background_color.0 = BUTTON_HOVERED_BACKGROUND_COLOR,
            Interaction::Pressed => {
                if let Ok(state_change) = state_changes.get(button) {
                    next_state.set(state_change.0);
                } else if let Ok(part_change) = part_changes.get(button) {
                    part_next_state.set(T::from(*part_change));
                }
            }
        }
    }
}
