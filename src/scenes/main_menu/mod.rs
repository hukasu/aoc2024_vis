mod resources;
mod ui;

use bevy::{
    color::Color,
    core::Name,
    prelude::{Camera2d, ClearColor, Commands, DespawnRecursiveExt, OnEnter, OnExit, Res},
};

use crate::scenes::states::Scene;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(ui::Plugin);

        app.add_systems(OnEnter(Scene::MainMenu), build_main_menu);
        app.add_systems(OnExit(Scene::MainMenu), destroy_main_menu);
    }
}

fn build_main_menu(mut commands: Commands) {
    let camera = commands
        .spawn((Name::new("main_menu_camera"), Camera2d))
        .id();

    let main_menu_resource = resources::MainMenu {
        camera,
        ..Default::default()
    };

    commands.insert_resource(ClearColor(Color::srgb_u8(0x0f, 0x0f, 0x23)));
    commands.insert_resource(main_menu_resource);
}

fn destroy_main_menu(mut commands: Commands, main_menu_resource: Res<resources::MainMenu>) {
    commands
        .entity(main_menu_resource.camera)
        .despawn_recursive();
    commands.entity(main_menu_resource.ui).despawn_recursive();

    commands.remove_resource::<resources::MainMenu>();
}
