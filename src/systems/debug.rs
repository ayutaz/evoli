use bevy::prelude::*;

use crate::components::collider::Circle;
use crate::components::combat::Health;
use crate::components::creatures::{CreatureTag, Movement, Wander};
use crate::components::digestion::Fullness;
use crate::resources::debug::DebugConfig;

// ---------------------------------------------------------------------------
// Toggle debug overlay with F1
// ---------------------------------------------------------------------------

pub fn toggle_debug_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut debug_config: ResMut<DebugConfig>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        debug_config.visible = !debug_config.visible;
    }
}

// ---------------------------------------------------------------------------
// Debug: draw collider circles (Gizmos)
// ---------------------------------------------------------------------------

pub fn debug_collider_system(
    debug_config: Res<DebugConfig>,
    mut gizmos: Gizmos,
    query: Query<(&Circle, &Transform), With<CreatureTag>>,
) {
    if !debug_config.visible {
        return;
    }
    for (circle, transform) in &query {
        let pos = transform.translation;
        gizmos.circle(
            Isometry3d::from_translation(Vec3::new(pos.x, pos.y, pos.z + 1.0)),
            circle.radius,
            Color::srgb(1.0, 0.5, 0.5),
        );
    }
}

// ---------------------------------------------------------------------------
// Debug: draw wander direction (Gizmos)
// ---------------------------------------------------------------------------

pub fn debug_wander_system(
    debug_config: Res<DebugConfig>,
    mut gizmos: Gizmos,
    query: Query<(&Wander, &Transform, &Movement), With<CreatureTag>>,
) {
    if !debug_config.visible {
        return;
    }
    for (wander, transform, movement) in &query {
        let pos = transform.translation + Vec3::Z * 0.5;
        let future_pos = pos + Vec3::new(movement.velocity.x, movement.velocity.y, 0.0) * 0.5
            + Vec3::Z * 0.5;
        let direction = wander.get_direction();

        // Line from current position to future position
        gizmos.line(pos, future_pos, Color::srgb(1.0, 0.05, 0.65));

        // Arrow showing wander direction from future position
        let arrow_end = future_pos + Vec3::new(direction.x, direction.y, 0.0);
        gizmos.arrow(future_pos, arrow_end, Color::srgb(1.0, 0.05, 0.65));
    }
}

// ---------------------------------------------------------------------------
// Debug: draw health bar (Gizmos)
// ---------------------------------------------------------------------------

pub fn debug_health_system(
    debug_config: Res<DebugConfig>,
    mut gizmos: Gizmos,
    query: Query<(&Health, &Transform), With<CreatureTag>>,
) {
    if !debug_config.visible {
        return;
    }
    for (health, transform) in &query {
        let pos = transform.translation;
        let start = Vec3::new(pos.x, pos.y + 0.5, pos.z + 0.5);
        let end = Vec3::new(pos.x + health.value / 100.0, pos.y + 0.5, pos.z + 0.5);
        gizmos.line(start, end, Color::srgb(0.0, 1.0, 0.0));
    }
}

// ---------------------------------------------------------------------------
// Debug: draw fullness bar (Gizmos)
// ---------------------------------------------------------------------------

pub fn debug_fullness_system(
    debug_config: Res<DebugConfig>,
    mut gizmos: Gizmos,
    query: Query<(&Fullness, &Transform)>,
) {
    if !debug_config.visible {
        return;
    }
    for (fullness, transform) in &query {
        let pos = transform.translation;
        let start = Vec3::new(pos.x, pos.y, 0.0);
        let end = Vec3::new(pos.x + fullness.value / 100.0, pos.y, 0.0);
        gizmos.line(start, end, Color::srgb(0.0, 1.0, 0.0));
    }
}

// ---------------------------------------------------------------------------
// Debug: draw entity detection lines (Gizmos)
// ---------------------------------------------------------------------------

#[cfg(feature = "perception_debug")]
pub fn debug_entity_detection_system(
    debug_config: Res<DebugConfig>,
    mut gizmos: Gizmos,
    query: Query<(
        &crate::components::experimental::perception::DetectedEntities,
        &Transform,
    )>,
    transform_query: Query<&Transform>,
) {
    if !debug_config.visible {
        return;
    }
    for (detected, transform) in &query {
        let pos = transform.translation + Vec3::Z * 0.3;
        for other_entity in detected.entities.iter() {
            if let Ok(other_transform) = transform_query.get(other_entity) {
                let other_pos = other_transform.translation + Vec3::Z * 0.3;
                gizmos.line(pos, other_pos, Color::srgb(1.0, 1.0, 0.0));
            }
        }
    }
}
