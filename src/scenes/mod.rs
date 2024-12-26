mod components;
mod main_menu;
mod states;

use bevy::prelude::AppExtStates;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(main_menu::Plugin);

        app.init_state::<states::States>();
    }
}
