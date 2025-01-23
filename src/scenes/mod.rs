mod components;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day24;
mod day25;
mod days;
mod main_menu;
mod resources;
mod states;

use bevy::{
    app::{PluginGroup, Startup},
    asset::AssetServer,
    prelude::{
        AppExtStates, Button, Changed, Commands, Entity, NextState, Or, Query, Res, ResMut, State,
        With,
    },
    ui::{BackgroundColor, Interaction},
};
use states::Part;

use crate::scroll_controls::{
    BUTTON_BACKGROUND_COLOR, BUTTON_HOVERED_BACKGROUND_COLOR, BUTTON_SELECTED_BACKGROUND_COLOR,
};

use self::{
    components::{PartChange, SceneChange},
    resources::FontHandles,
};

type ButtonWithChangedInteractionQuery<'a, 'b> = Query<
    'a,
    'b,
    (Entity, &'static mut BackgroundColor, &'static Interaction),
    (
        With<Button>,
        Changed<Interaction>,
        Or<(With<PartChange>, With<SceneChange>)>,
    ),
>;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(ScenesPluginGroup);

        app.init_state::<states::Scene>()
            .add_sub_state::<states::Part>()
            .add_sub_state::<states::InputState>()
            .add_sub_state::<states::UiState>();

        app.add_systems(Startup, load_font);
    }
}

fn load_font(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(FontHandles {
        font: asset_server.load("NotoSans-VariableFont_wdth,wght.ttf"),
        symbol1: asset_server.load("NotoSansSymbols-VariableFont_wght.ttf"),
        symbol2: asset_server.load("NotoSansSymbols2-Regular.ttf"),
    });
}

fn state_button_interactions(
    mut buttons: ButtonWithChangedInteractionQuery,
    state_changes: Query<&SceneChange>,
    part_changes: Query<&PartChange>,
    current_part: Res<State<Part>>,
    mut next_state: ResMut<NextState<states::Scene>>,
    mut part_next_state: ResMut<NextState<Part>>,
) {
    for (button, mut background_color, interaction) in buttons.iter_mut() {
        match interaction {
            Interaction::None => {
                let mut color = BUTTON_BACKGROUND_COLOR;
                if let Ok(part_change) = part_changes.get(button) {
                    if Part::from(*part_change) == *current_part.get() {
                        color = BUTTON_SELECTED_BACKGROUND_COLOR;
                    }
                }
                background_color.0 = color;
            }
            Interaction::Hovered => background_color.0 = BUTTON_HOVERED_BACKGROUND_COLOR,
            Interaction::Pressed => {
                if let Ok(state_change) = state_changes.get(button) {
                    next_state.set(state_change.0);
                } else if let Ok(part_change) = part_changes.get(button) {
                    part_next_state.set(Part::from(*part_change));
                }
            }
        }
    }
}

pub struct ScenesPluginGroup;

impl PluginGroup for ScenesPluginGroup {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
            .add(main_menu::Plugin)
            .add(day01::Plugin)
            .add(day02::Plugin)
            .add(day03::Plugin)
            .add(day04::Plugin)
            .add(day05::Plugin)
            .add(day06::Plugin)
            .add(day07::Plugin)
            .add(day08::Plugin)
            .add(day09::Plugin)
            .add(day10::Plugin)
            .add(day11::Plugin)
            .add(day12::Plugin)
            .add(day13::Plugin)
            .add(day14::Plugin)
            .add(day24::Plugin)
            .add(day25::Plugin)
    }
}
