use bevy::prelude::*;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

use crate::components::combat::HasFaction;
use crate::resources::prefabs::{CreaturePrefabs, Factions};

/// Event requesting that a creature be spawned.
#[derive(Event, Debug, Clone)]
pub struct CreatureSpawnEvent {
    pub creature_type: String,
    pub transform: Transform,
}

/// Local timer resource for the debug spawn trigger system.
#[derive(Default)]
pub struct SpawnTimer {
    timer_to_next_spawn: f32,
}

/// For debugging purposes, this system sends `CreatureSpawnEvent`s at regular intervals.
pub fn debug_spawn_trigger_system(
    time: Res<Time>,
    mut spawn_events: EventWriter<CreatureSpawnEvent>,
    mut timer: Local<SpawnTimer>,
) {
    let delta_seconds = time.delta_secs();
    timer.timer_to_next_spawn -= delta_seconds;

    if timer.timer_to_next_spawn <= 0.0 {
        timer.timer_to_next_spawn = 1.5;

        let mut rng = thread_rng();
        let x = rng.gen_range(-5.0f32..5.0f32);
        let y = rng.gen_range(-5.0f32..5.0f32);

        // Randomly pick a creature type
        let creature_type = match rng.gen_range(0..3) {
            0 => "Herbivore".to_string(),
            1 => "Carnivore".to_string(),
            _ => "Plant".to_string(),
        };

        let mut transform = Transform::from_xyz(x, y, 0.02);

        if creature_type == "Carnivore" || creature_type == "Herbivore" {
            transform.scale = Vec3::splat(0.4);
        }

        if creature_type == "Plant" {
            let scale = rng.gen_range(0.8f32..1.2f32);
            let rotation = rng.gen_range(0.0f32..PI);
            transform.translation.z = 0.01;
            transform.scale = Vec3::splat(scale);
            transform.rotation = Quat::from_rotation_z(rotation);
        }

        spawn_events.send(CreatureSpawnEvent {
            creature_type,
            transform,
        });
    }
}

/// Reads `CreatureSpawnEvent`s and spawns entities using `CreaturePrefabs` definitions.
pub fn creature_spawner_system(
    mut commands: Commands,
    mut spawn_events: EventReader<CreatureSpawnEvent>,
    prefabs: Res<CreaturePrefabs>,
    factions: Res<Factions>,
    asset_server: Res<AssetServer>,
) {
    for event in spawn_events.read() {
        if let Some(def) = prefabs.prefabs.get(&event.creature_type) {
            let mut entity_commands = commands.spawn(event.transform);

            // Insert tag components
            if def.creature_tag {
                entity_commands.insert(crate::components::creatures::CreatureTag);
            }
            if def.intelligence_tag {
                entity_commands.insert(crate::components::creatures::IntelligenceTag);
            }
            if def.ricochet_tag {
                entity_commands.insert(crate::components::creatures::RicochetTag);
            }
            if def.avoid_obstacles_tag {
                entity_commands.insert(crate::components::creatures::AvoidObstaclesTag);
            }
            if def.despawn_when_out_of_bounds_tag {
                entity_commands.insert(crate::components::creatures::DespawnWhenOutOfBoundsTag);
            }
            if def.topplegrass_tag {
                entity_commands.insert(crate::components::creatures::TopplegrassTag);
            }
            if def.falling_tag {
                entity_commands.insert(crate::components::creatures::FallingTag);
            }

            // Insert data components (cloned from the definition)
            if let Some(ref movement) = def.movement {
                entity_commands.insert(movement.clone());
            }
            if let Some(ref wander) = def.wander {
                entity_commands.insert(*wander);
            }
            if let Some(ref collider) = def.collider {
                entity_commands.insert(collider.clone());
            }
            if let Some(ref perception) = def.perception {
                entity_commands.insert(perception.clone());
            }
            if let Some(ref carcass) = def.carcass {
                entity_commands.insert(carcass.clone());
            }

            // Digestion components
            if let Some(ref digestion_data) = def.digestion {
                if let Some(ref fullness) = digestion_data.fullness {
                    entity_commands.insert(fullness.clone());
                }
                if let Some(ref digestion) = digestion_data.digestion {
                    entity_commands.insert(digestion.clone());
                }
                if let Some(ref nutrition) = digestion_data.nutrition {
                    entity_commands.insert(nutrition.clone());
                }
            }

            // Combat components
            if let Some(ref combat_data) = def.combat {
                if let Some(ref health) = combat_data.health {
                    entity_commands.insert(health.clone());
                }
                if let Some(ref speed) = combat_data.speed {
                    entity_commands.insert(speed.clone());
                }
                if let Some(ref damage) = combat_data.damage {
                    entity_commands.insert(damage.clone());
                }
                // Resolve faction: String -> Entity
                if let Some(ref has_faction_data) = combat_data.faction {
                    if let Some(&faction_entity) = factions.0.get(&has_faction_data.0) {
                        entity_commands.insert(HasFaction(faction_entity));
                    }
                }
            }

            // Load glTF scene
            if let Some(ref gltf_path) = def.gltf {
                entity_commands.insert(SceneRoot(asset_server.load(gltf_path.clone())));
            }

            // Insert a Name component if provided
            if let Some(ref name) = def.name {
                entity_commands.insert(Name::new(name.clone()));
            }
        }
    }
}
