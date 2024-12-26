mod scenes;

#[cfg(feature = "with_inspector")]
use bevy::remote::{http::RemoteHttpPlugin, RemotePlugin};
use bevy::{app::App, DefaultPlugins};

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, scenes::Plugin));
    #[cfg(feature = "with_inspector")]
    app.add_plugins((RemotePlugin::default(), RemoteHttpPlugin::default()));

    app.run();
}
