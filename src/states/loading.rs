use bevy::prelude::*;
use std::collections::HashMap;

use crate::AppState;
use crate::components::creatures::CreatureDefinition;
use crate::resources::world_bounds::WorldBounds;
use crate::resources::debug::DebugConfig;
use crate::resources::experimental::wind::Wind;
use crate::resources::prefabs::CreaturePrefabs;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), setup_loading)
           .add_systems(Update, check_loading.run_if(in_state(AppState::Loading)));
    }
}

fn setup_loading(mut commands: Commands) {
    // Initialize core resources
    commands.insert_resource(WorldBounds::new(-10.0, 10.0, -10.0, 10.0));
    commands.insert_resource(DebugConfig::default());

    // Load Wind config
    let wind = load_wind_config();
    commands.insert_resource(wind);

    // Load creature prefabs from RON files
    let prefabs = load_creature_prefabs();
    commands.insert_resource(prefabs);
}

fn check_loading(
    mut next_state: ResMut<NextState<AppState>>,
) {
    // Faction entities are created in main_game::setup_faction_entities
    next_state.set(AppState::Menu);
}

/// Load the Wind resource from the RON configuration file.
fn load_wind_config() -> Wind {
    match std::fs::read_to_string("resources/wind.ron") {
        Ok(contents) => {
            match ron::de::from_str::<Wind>(&contents) {
                Ok(wind) => wind,
                Err(e) => {
                    warn!("Failed to parse wind config: {:?}. Using default.", e);
                    Wind::default()
                }
            }
        }
        Err(e) => {
            warn!("Failed to read wind config file: {:?}. Using default.", e);
            Wind::default()
        }
    }
}

/// Load all creature definitions from RON files in resources/prefabs/creatures/
fn load_creature_prefabs() -> CreaturePrefabs {
    let mut prefabs = HashMap::new();

    let creature_files = [
        ("Plant", "resources/prefabs/creatures/plant.ron"),
        ("Herbivore", "resources/prefabs/creatures/herbivore.ron"),
        ("Carnivore", "resources/prefabs/creatures/carnivore.ron"),
        ("HerbivoreCarcass", "resources/prefabs/creatures/herbivore_carcass.ron"),
        ("Ground", "resources/prefabs/creatures/ground.ron"),
        ("Ixie", "resources/prefabs/creatures/ixie.ron"),
        ("Nushi", "resources/prefabs/creatures/nushi.ron"),
        ("Topplegrass", "resources/prefabs/creatures/topplegrass.ron"),
    ];

    let ron_options = ron::Options::default()
        .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME);

    for (key, path) in &creature_files {
        match std::fs::read_to_string(path) {
            Ok(contents) => {
                match ron_options.from_str::<CreatureDefinition>(&contents) {
                    Ok(def) => {
                        info!("Loaded creature definition: {}", key);
                        prefabs.insert(key.to_string(), def);
                    }
                    Err(e) => {
                        warn!("Failed to parse {}: {:?}", path, e);
                    }
                }
            }
            Err(e) => {
                warn!("Failed to read {}: {:?}", path, e);
            }
        }
    }

    info!("Loaded {} creature prefabs", prefabs.len());
    CreaturePrefabs { prefabs }
}

