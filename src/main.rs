use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture, window::WindowResolution
};
use serde::Deserialize;
use std::collections::HashMap;
use toml;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution {
                            physical_height: 600,
                            physical_width: 900,
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            )
        )
        .add_asset::<TomlAsset>()
        .init_asset_loader::<TomlLoader>()
        .add_startup_system(load_assets)
        .add_system(load_data)
        .run();
}

fn load_data(
    keys: Res<Input<KeyCode>>,
    toml_assets: Res<Assets<TomlAsset>>,
    data_assets: Res<DataAssets>
) {
    if keys.just_pressed(KeyCode::Space){
        let data_str = toml_assets.get(&data_assets.handle).expect("Not a valid asset!");
        let birds: HashMap<&str, Bird> = toml::from_str(&data_str.0).expect("Could not parse the data!");
        info!("{:?}", birds);
    }
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let bird_data: Handle<TomlAsset> = asset_server.load("birds.toml");
    info!("{:?}", bird_data);

    commands.insert_resource(DataAssets { handle: bird_data} );
}

#[derive(Debug, Deserialize)]
struct Bird {
    mass: f32,
    speed: u32
}

#[derive(Debug, Resource)]
struct DataAssets {
    pub handle: Handle<TomlAsset>
}

#[derive(Debug, TypeUuid)]
#[uuid = "ff866d71-0c0e-4af0-8437-a4177ed03f2c"]
struct TomlAsset(pub String);


#[derive(Default)]
struct TomlLoader;

impl AssetLoader for TomlLoader {
    fn load<'a>(
            &'a self,
            bytes: &'a [u8],
            load_context: &'a mut LoadContext,
        ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let data_str = std::str::from_utf8(bytes)?;
            let asset = TomlAsset( data_str.into() );
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }
}