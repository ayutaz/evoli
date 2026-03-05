use bevy::prelude::*;

mod components;
mod resources;
mod states;
mod systems;
mod utils;

/// Top-level application state machine.
/// Mirrors the original Amethyst state flow: Loading -> Menu -> InGame.
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Loading,
    Menu,
    InGame,
}

/// Sub-state active only while `AppState::InGame`.
/// Controls whether the simulation is running or paused.
#[derive(SubStates, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[source(AppState = AppState::InGame)]
pub enum GamePlayState {
    #[default]
    Running,
    Paused,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Evoli".to_string(),
                    resolution: (1024.0, 768.0).into(),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                file_path: "resources".to_string(),
                ..default()
            })
        )
        // States
        .init_state::<AppState>()
        .add_sub_state::<GamePlayState>()
        .enable_state_scoped_entities::<AppState>()
        .enable_state_scoped_entities::<GamePlayState>()
        // Plugins (state lifecycle)
        .add_plugins((
            states::loading::LoadingPlugin,
            states::menu::MenuPlugin,
            states::main_game::MainGamePlugin,
            states::pause_menu::PauseMenuPlugin,
        ))
        // Events
        .add_event::<systems::collision::CollisionEvent>()
        .add_event::<systems::combat::AttackEvent>()
        .add_event::<systems::death::CreatureDeathEvent>()
        .add_event::<systems::spawner::CreatureSpawnEvent>()
        .add_event::<systems::main_game_ui::SpeedUpEvent>()
        .add_event::<systems::main_game_ui::SlowDownEvent>()
        .add_event::<systems::main_game_ui::TogglePauseEvent>()
        .add_event::<systems::main_game_ui::MenuEvent>()
        // Resources
        .init_resource::<systems::behaviors::decision::FactionQueries<systems::behaviors::decision::Prey>>()
        .init_resource::<systems::behaviors::decision::FactionQueries<systems::behaviors::decision::Predator>>()
        .init_resource::<resources::prefabs::CreaturePrefabs>()
        .init_resource::<resources::prefabs::Factions>()
        .init_resource::<systems::swarm_behavior::SwarmSpawnTimer>()
        .init_resource::<systems::experimental::topplegrass::TopplegrassSpawnTimer>()
        .insert_resource(resources::experimental::spatial_grid::SpatialGrid::new(2.0))
        // SeekConfig resources for prey (seek) and predator (flee)
        .insert_resource(systems::behaviors::decision::SeekConfig::<systems::behaviors::decision::Prey>::new(
            Quat::IDENTITY,
            5.0,
        ))
        .insert_resource(systems::behaviors::decision::SeekConfig::<systems::behaviors::decision::Predator>::new(
            Quat::from_rotation_z(std::f32::consts::PI),
            8.0,
        ))
        // Systems - organized by GameSet
        .add_systems(
            Update,
            (
                // Perception
                (
                    systems::experimental::perception::spatial_grid_system,
                    systems::experimental::perception::entity_detection_system,
                )
                    .chain()
                    .in_set(states::main_game::GameSet::Perception),
                // Decision
                (
                    systems::behaviors::decision::query_predators_and_prey_system,
                    systems::behaviors::decision::closest_system::<systems::behaviors::decision::Prey>,
                    systems::behaviors::decision::closest_system::<systems::behaviors::decision::Predator>,
                    systems::behaviors::obstacle::closest_obstacle_system,
                )
                    .in_set(states::main_game::GameSet::Decision),
                // Behavior
                (
                    systems::behaviors::decision::seek_system::<systems::behaviors::decision::Prey>,
                    systems::behaviors::decision::seek_system::<systems::behaviors::decision::Predator>,
                    systems::behaviors::wander::wander_system,
                    systems::behaviors::ricochet::ricochet_system,
                    systems::swarm_behavior::swarm_behavior_system,
                )
                    .in_set(states::main_game::GameSet::Behavior),
                // Movement
                (
                    systems::movement::velocity_cap_system,
                    systems::movement::movement_system,
                    systems::movement::creature_rotation_system,
                    systems::collision::collision_system,
                    systems::collision::enforce_bounds_system,
                )
                    .chain()
                    .in_set(states::main_game::GameSet::Movement),
                // Metabolism
                systems::digestion::digestion_system
                    .in_set(states::main_game::GameSet::Metabolism),
                // Combat
                (
                    systems::combat::cooldown_system,
                    systems::combat::find_attack_system,
                    systems::combat::perform_default_attack_system,
                )
                    .chain()
                    .in_set(states::main_game::GameSet::Combat),
                // Death
                (
                    systems::death::starvation_system,
                    systems::death::death_by_health_system,
                    systems::death::carcass_system,
                )
                    .in_set(states::main_game::GameSet::Death),
                // Spawn
                (
                    systems::spawner::debug_spawn_trigger_system,
                    systems::spawner::creature_spawner_system,
                    systems::swarm_behavior::swarm_spawn_system,
                    systems::swarm_behavior::swarm_center_system,
                )
                    .in_set(states::main_game::GameSet::Spawn),
                // Experimental
                (
                    systems::experimental::gravity::gravity_system,
                    systems::experimental::out_of_bounds::out_of_bounds_despawn_system,
                    systems::experimental::topplegrass::topplegrass_spawn_system,
                    systems::experimental::topplegrass::toppling_system,
                    systems::experimental::wind_control::debug_wind_control_system,
                )
                    .in_set(states::main_game::GameSet::Experimental),
            ),
        )
        // Debug systems (always run in InGame, check DebugConfig internally)
        .add_systems(
            Update,
            (
                systems::debug::toggle_debug_system,
                systems::debug::debug_collider_system,
                systems::debug::debug_wander_system,
                systems::debug::debug_health_system,
                systems::debug::debug_fullness_system,
            )
                .run_if(in_state(AppState::InGame)),
        )
        // Camera and time control (always run in InGame)
        .add_systems(
            Update,
            (
                systems::camera_movement::camera_movement_system,
                systems::time_control::time_control_system,
            )
                .run_if(in_state(AppState::InGame)),
        )
        // Game UI (setup on InGame enter, cleanup on exit)
        .add_systems(OnEnter(AppState::InGame), systems::main_game_ui::setup_game_ui)
        .add_systems(OnExit(AppState::InGame), systems::main_game_ui::cleanup_game_ui)
        .add_systems(
            Update,
            (
                systems::main_game_ui::game_ui_interaction,
                systems::main_game_ui::button_visual_feedback,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .run();
}
