use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::f32::consts::PI;

use crate::components::combat::FactionPrey;
use crate::components::creatures::CreatureTag;
use crate::resources::prefabs::Factions;
use crate::resources::world_bounds::WorldBounds;
use crate::systems::spawner::CreatureSpawnEvent;
use crate::AppState;
use crate::GamePlayState;

pub struct MainGamePlugin;

/// SystemSets that define the execution order within the main game loop.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSet {
    Perception,
    Decision,
    Behavior,
    Movement,
    Metabolism,
    Combat,
    Death,
    Spawn,
    Experimental,
}

impl Plugin for MainGamePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                GameSet::Perception,
                GameSet::Decision,
                GameSet::Behavior,
                GameSet::Movement,
                GameSet::Metabolism,
                GameSet::Combat,
                GameSet::Death,
                GameSet::Spawn,
                GameSet::Experimental,
            )
                .chain()
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GamePlayState::Running)),
        );

        app.add_systems(OnEnter(AppState::InGame), setup_main_game);
        app.add_systems(OnExit(AppState::InGame), cleanup_main_game);
    }
}

/// Marker component for the main game camera.
#[derive(Component)]
struct MainGameCamera;

/// Marker component for entities spawned as part of the main game scene.
#[derive(Component)]
pub struct MainGameEntity;

fn setup_main_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    world_bounds: Res<WorldBounds>,
    mut factions: ResMut<Factions>,
    mut spawn_events: EventWriter<CreatureSpawnEvent>,
) {
    info!("Starting main game");

    // Setup 3D orthographic camera
    let camera_transform = Transform::from_xyz(-10.0, -10.0, 8.0).looking_at(Vec3::ZERO, Vec3::Z);

    commands.spawn((
        MainGameCamera,
        MainGameEntity,
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scale: 0.2,
            near: 0.1,
            far: 1000.0,
            ..OrthographicProjection::default_3d()
        }),
        camera_transform,
    ));

    // Setup directional light (sun)
    commands.spawn((
        MainGameEntity,
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_rotation_x(-PI / 3.0)),
    ));

    // Setup ambient light
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.4, 0.4, 0.5),
        brightness: 300.0,
    });

    // --- Spawn faction entities ---
    setup_faction_entities(&mut commands, &mut factions);

    // --- Spawn ground with glTF scene ---
    commands.spawn((
        MainGameEntity,
        SceneRoot(asset_server.load("assets/ground2.glb#Scene0")),
        Transform::from_scale(Vec3::new(1.05, 1.05, 1.0)),
    ));

    // --- Spawn initial plants via CreatureSpawnEvent ---
    let mut rng = thread_rng();
    for _ in 0..25 {
        let x = rng.gen_range(world_bounds.left..world_bounds.right);
        let y = rng.gen_range(world_bounds.bottom..world_bounds.top);
        let scale = rng.gen_range(0.8f32..1.2f32);
        let rotation = rng.gen_range(0.0f32..PI);

        spawn_events.send(CreatureSpawnEvent {
            creature_type: "Plant".to_string(),
            transform: Transform::from_xyz(x, y, 0.01)
                .with_scale(Vec3::splat(scale))
                .with_rotation(Quat::from_rotation_z(rotation)),
        });
    }

    // Spawn a few herbivores
    for _ in 0..5 {
        let x = rng.gen_range(world_bounds.left..world_bounds.right);
        let y = rng.gen_range(world_bounds.bottom..world_bounds.top);
        spawn_events.send(CreatureSpawnEvent {
            creature_type: "Herbivore".to_string(),
            transform: Transform::from_xyz(x, y, 0.02).with_scale(Vec3::splat(0.4)),
        });
    }

    // Spawn a couple of carnivores
    for _ in 0..2 {
        let x = rng.gen_range(world_bounds.left..world_bounds.right);
        let y = rng.gen_range(world_bounds.bottom..world_bounds.top);
        spawn_events.send(CreatureSpawnEvent {
            creature_type: "Carnivore".to_string(),
            transform: Transform::from_xyz(x, y, 0.02).with_scale(Vec3::splat(0.4)),
        });
    }
}

/// Load faction definitions from RON and spawn faction entities with FactionPrey components.
fn setup_faction_entities(commands: &mut Commands, factions: &mut Factions) {
    #[derive(serde::Deserialize)]
    struct FactionDef {
        prey: Vec<String>,
    }

    let faction_defs: HashMap<String, FactionDef> =
        match std::fs::read_to_string("resources/prefabs/factions.ron") {
            Ok(contents) => ron::de::from_str(&contents).unwrap_or_default(),
            Err(_) => HashMap::new(),
        };

    // First pass: create faction entities and store the name -> Entity mapping
    let mut name_to_entity = HashMap::new();
    for name in faction_defs.keys() {
        let entity = commands.spawn_empty().id();
        name_to_entity.insert(name.clone(), entity);
    }

    // Second pass: resolve prey references and attach FactionPrey components
    for (name, def) in &faction_defs {
        let faction_entity = name_to_entity[name];
        let prey_entities: Vec<Entity> = def
            .prey
            .iter()
            .filter_map(|prey_name| name_to_entity.get(prey_name).copied())
            .collect();
        commands
            .entity(faction_entity)
            .insert(FactionPrey(prey_entities));
    }

    // Update the Factions resource
    factions.0 = name_to_entity;
    info!("Created {} faction entities", factions.0.len());
}

fn cleanup_main_game(
    mut commands: Commands,
    game_entities: Query<Entity, With<MainGameEntity>>,
    creature_entities: Query<Entity, With<CreatureTag>>,
) {
    info!("Stopping main game");

    for entity in &game_entities {
        commands.entity(entity).despawn_recursive();
    }

    for entity in &creature_entities {
        commands.entity(entity).despawn_recursive();
    }
}
