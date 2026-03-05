use bevy::prelude::*;

use crate::AppState;
use crate::resources::world_bounds::WorldBounds;
use crate::resources::debug::DebugConfig;
use crate::resources::experimental::wind::Wind;

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

    // Load Wind config from RON file using std::fs
    let wind = load_wind_config();
    commands.insert_resource(wind);
}

fn check_loading(mut next_state: ResMut<NextState<AppState>>) {
    // Simple version: transition to Menu immediately.
    // In a full implementation, this would wait for asset loading to complete.
    next_state.set(AppState::Menu);
}

/// Load the Wind resource from the RON configuration file.
/// Falls back to default values if the file cannot be read or parsed.
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
