// Most of this code was taken from the bevy_common_assets crate:
// https://github.com/NiklasEi/bevy_common_assets/blob/main/src/json.rs
// There are also examples of the same with JSON, RON, TOML and more if we want to switch later.

use bevy::app::{App, Plugin};
use bevy::asset::io::Reader;
use bevy::asset::*;
use bevy::prelude::*;
use serde::Deserialize;
use serde_json::from_slice;
use thiserror::Error;

/// Plugin to load your asset type [`FactionAsset`] from json files.
pub struct FactionLoaderPlugin;

impl Plugin for FactionLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader::<FactionAssetLoader>(FactionAssetLoader)
            .init_asset::<FactionAsset>()
            .insert_resource(Factions(vec![]))
            .add_systems(Startup, load_factions)
            .add_systems(
                Update,
                (
                    setup_factions_resource.after(load_factions),
                    display_content.after(setup_factions_resource), // TODO: Remove display_content, when actually using the factions resource.
                ),
            );
    }
}

// --- Custom Asset Loader ---
// Read more on custom asset loaders under `Custom asset loader` here:
// https://taintedcoders.com/bevy/assets/

/// Loads your asset type [`FactionAsset`] from json files.
pub struct FactionAssetLoader;

/// Possible errors that can be produced by [`FactionAssetLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum FactionLoaderError {
    /// An [IO Error](std::io::Error)
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),
    /// A [JSON Error](serde_json::Error)
    #[error("Could not parse JSON: {0}")]
    JsonError(#[from] serde_json::Error),
}

impl AssetLoader for FactionAssetLoader {
    type Asset = FactionAsset;
    type Settings = ();
    type Error = FactionLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let asset = from_slice::<FactionAsset>(&bytes)?;
            Ok(asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        // Use this loader for files ending with faction.json
        &["faction.json"]
    }
}

// --- Asset types ---

#[derive(Deserialize, Debug, Clone, TypePath, Asset)]
pub struct FactionAsset {
    id: String,
    name: String,
    buildings: Vec<BuildingData>,
    units: Vec<UnitData>,
}

#[derive(Deserialize, Debug, Clone)]
struct BuildingData {
    id: String,
    name: String,
    sprite: String,
    components: Vec<ComponentBlueprint>,
}

#[derive(Deserialize, Debug, Clone)]
struct UnitData {
    id: String,
    name: String,
    sprite: String,
    components: Vec<ComponentBlueprint>,
}

// --- Resource types ---

#[derive(Resource, Debug)]
pub struct FactionBlueprint {
    pub id: String,
    pub name: String,
    pub buildings: Vec<BuildingBlueprint>,
    pub units: Vec<UnitBlueprint>,
}

#[derive(Debug)]
pub struct BuildingBlueprint {
    pub id: String,
    pub name: String,
    pub sprite: Handle<Image>,
    pub components: Vec<ComponentBlueprint>,
}

#[derive(Debug)]
pub struct UnitBlueprint {
    pub id: String,
    pub name: String,
    pub sprite: Handle<Image>,
    pub components: Vec<ComponentBlueprint>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum ComponentBlueprint {
    UnitSpawner {
        unit_id: String,
        spawn_time: f32,
    },
    Health {
        max_health: i32,
        health: i32,
    },
    AttackStats {
        damage: i32,
        attack_speed: f32,
        attack_range: i32,
    },
    OpponentFollower,
    MovementSpeed(i32),
    Visible,
}

// --- Resources ---

#[derive(Resource)]
struct FactionsFolderHandle(Handle<LoadedFolder>);

#[derive(Resource)]
struct Factions(Vec<FactionBlueprint>);

// --- Systems ---

/// Starts loading all the faction files in the factions folder.
fn load_factions(mut commands: Commands, asset_server: Res<AssetServer>) {
    let folder_handle = asset_server.load_folder("factions");
    commands.insert_resource(FactionsFolderHandle(folder_handle));
}

/// Converts the loaded faction files to blueprints and adds them to resources.
fn setup_factions_resource(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    factions_folder: Res<FactionsFolderHandle>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
    loaded_folder: Res<Assets<LoadedFolder>>,
    factions: Res<Assets<FactionAsset>>,
) {
    // Waits for the event telling that the files were loaded.
    for event in events.read() {
        if event.is_loaded_with_dependencies(&factions_folder.0) {
            if let Some(loaded_factions) = loaded_folder.get(&factions_folder.0) {
                let mut faction_blueprints: Vec<FactionBlueprint> = vec![];

                for handle in loaded_factions.handles.iter() {
                    let id = handle.id().typed_unchecked::<FactionAsset>();
                    let Some(faction) = factions.get(id) else {
                        warn!(
                            "{:?} did not resolve to an `Faction` asset.",
                            handle.path().unwrap()
                        );
                        continue;
                    };

                    let faction_blueprint = FactionBlueprint {
                        id: faction.id.clone(),
                        name: faction.name.clone(),
                        buildings: faction
                            .buildings
                            .iter()
                            .map(|building_asset| BuildingBlueprint {
                                id: building_asset.id.clone(),
                                name: building_asset.name.clone(),
                                sprite: asset_server.load(&building_asset.sprite),
                                components: building_asset.components.clone(),
                            })
                            .collect(),
                        units: faction
                            .units
                            .iter()
                            .map(|unit_asset| UnitBlueprint {
                                id: unit_asset.id.clone(),
                                name: unit_asset.name.clone(),
                                sprite: asset_server.load(&unit_asset.sprite),
                                components: unit_asset.components.clone(),
                            })
                            .collect(),
                    };

                    faction_blueprints.push(faction_blueprint);
                }

                commands.insert_resource(Factions(faction_blueprints))
            }
        }
    }
}

/// This is a test function to see that it works. When factions are in use, this should be removed.
fn display_content(mut commands: Commands, factions_res: Option<Res<Factions>>) {
    let Some(factions) = factions_res else {
        return;
    };

    info!("Printing all factions!");
    for faction in factions.0.iter() {
        info!("FACTION: {:?}", faction);
    }
    commands.remove_resource::<Factions>();
}
