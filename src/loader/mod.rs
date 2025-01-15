use bevy::{asset::Asset, prelude::Deref, reflect::Reflect};

#[derive(Asset, Reflect, Deref)]
pub struct RawInput(pub Vec<u8>);

pub struct AssetLoader;

impl bevy::asset::AssetLoader for AssetLoader {
    type Asset = RawInput;
    type Settings = ();
    type Error = std::io::Error;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        _load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data).await?;

        Ok(RawInput(data))
    }

    fn extensions(&self) -> &[&str] {
        &["txt"]
    }
}
