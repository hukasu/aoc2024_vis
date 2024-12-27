mod loader;
mod scenes;

#[cfg(feature = "with_inspector")]
use bevy::remote::{http::RemoteHttpPlugin, RemotePlugin};
use bevy::{app::App, asset::AssetApp, DefaultPlugins};

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, scenes::Plugin));
    #[cfg(feature = "with_inspector")]
    app.add_plugins((RemotePlugin::default(), RemoteHttpPlugin::default()));

    app.register_asset_loader(loader::AssetLoader)
        .init_asset::<loader::Input>()
        .register_type::<loader::Input>();

    app.run();
}
