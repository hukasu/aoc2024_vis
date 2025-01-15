mod loader;
mod scenes;
pub mod scroll_controls;

#[cfg(feature = "with_inspector")]
use bevy::remote::{http::RemoteHttpPlugin, RemotePlugin};
use bevy::{
    app::App,
    asset::AssetApp,
    prelude::{DefaultGizmoConfigGroup, GizmoConfigStore},
    render::view::RenderLayers,
    DefaultPlugins,
};

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, scenes::Plugin, scroll_controls::Plugin));
    #[cfg(feature = "with_inspector")]
    app.add_plugins((RemotePlugin::default(), RemoteHttpPlugin::default()));

    app.register_asset_loader(loader::AssetLoader)
        .init_asset::<loader::RawInput>()
        .register_type::<loader::RawInput>();

    let mut gizmos_config_store = app
        .world_mut()
        .get_resource_mut::<GizmoConfigStore>()
        .unwrap();
    let (default, _) = gizmos_config_store.config_mut::<DefaultGizmoConfigGroup>();
    default.render_layers = RenderLayers::from_layers(&[1]);

    app.run();
}
