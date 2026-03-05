use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

use crate::AppState;
use crate::GamePlayState;
use crate::components::creatures::CreatureTag;
use crate::resources::world_bounds::WorldBounds;

pub struct MainGamePlugin;

/// SystemSets that define the execution order within the main game loop.
/// These replace the three Amethyst Dispatchers (main, debug, ui).
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
        // Define SystemSet ordering (chain = sequential execution)
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

        // OnEnter: set up the game scene (camera, lights, initial entities)
        app.add_systems(OnEnter(AppState::InGame), setup_main_game);
        // OnExit: clean up all game entities
        app.add_systems(OnExit(AppState::InGame), cleanup_main_game);
    }
}

/// Marker component for the main game camera.
#[derive(Component)]
struct MainGameCamera;

/// Marker component for entities spawned as part of the main game scene
/// (lights, ground, etc.) that need cleanup on exit.
#[derive(Component)]
struct MainGameEntity;

fn setup_main_game(
    mut commands: Commands,
    world_bounds: Res<WorldBounds>,
) {
    info!("Starting main game");

    // Setup 3D orthographic camera
    // Original: position (-10, -10, 8), rotation (pi/3, 0, -pi/4), zoom_factor = 95
    let camera_transform = Transform::from_xyz(-10.0, -10.0, 8.0)
        .looking_at(Vec3::ZERO, Vec3::Z);

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
        color: Color::srgb(0.2, 0.2, 0.2),
        brightness: 200.0,
    });

    // Spawn initial plants
    let mut rng = thread_rng();
    for _ in 0..25 {
        let x = rng.gen_range(world_bounds.left..world_bounds.right);
        let y = rng.gen_range(world_bounds.bottom..world_bounds.top);
        let scale = rng.gen_range(0.8f32..1.2f32);
        let rotation = rng.gen_range(0.0f32..PI);

        commands.spawn((
            MainGameEntity,
            CreatureTag,
            Transform::from_xyz(x, y, 0.01)
                .with_scale(Vec3::new(scale, scale, 1.0))
                .with_rotation(Quat::from_rotation_z(rotation)),
        ));
        // NOTE: In a full implementation, a CreatureSpawnEvent would be sent here
        // to attach the Plant prefab data. For now we just spawn the transform + tag.
    }

    // Spawn ground entity
    commands.spawn((
        MainGameEntity,
        CreatureTag,
        Transform::from_scale(Vec3::new(1.05, 1.05, 1.0)),
    ));
}

fn cleanup_main_game(
    mut commands: Commands,
    game_entities: Query<Entity, With<MainGameEntity>>,
    creature_entities: Query<Entity, With<CreatureTag>>,
) {
    info!("Stopping main game");

    // Despawn all main game scene entities (camera, lights, etc.)
    for entity in &game_entities {
        commands.entity(entity).despawn_recursive();
    }

    // Despawn all creatures/organisms that don't have MainGameEntity
    for entity in &creature_entities {
        // despawn_recursive is safe to call on already-despawned entities in Bevy
        commands.entity(entity).despawn_recursive();
    }
}
