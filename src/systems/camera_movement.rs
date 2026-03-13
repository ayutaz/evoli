use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

/// Camera zoom system that uses mouse wheel to zoom in/out
/// by adjusting the orthographic projection scale.
pub fn camera_zoom_system(
    mut scroll_events: EventReader<MouseWheel>,
    mut query: Query<&mut Projection, With<Camera>>,
) {
    for event in scroll_events.read() {
        let scroll = match event.unit {
            MouseScrollUnit::Line => event.y * 0.1,
            MouseScrollUnit::Pixel => event.y * 0.001,
        };
        for mut projection in &mut query {
            if let Projection::Orthographic(ref mut ortho) = *projection {
                ortho.scale = (ortho.scale * (1.0 - scroll)).clamp(0.05, 2.0);
            }
        }
    }
}

/// Camera movement system that uses arrow keys to move the camera,
/// matching the original Amethyst key bindings from input.ron:
///   - Arrow Up/Down/Left/Right for panning
///   - LShift + Arrow Up/Down for forward/backward (zoom via Z-axis)
pub fn camera_movement_system(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let delta_time = time.delta_secs();
    let move_factor = 12.0 * delta_time;

    for mut transform in &mut query {
        // Pan up/down (Y axis) — Arrow Up/Down without Shift
        if keyboard.pressed(KeyCode::ArrowUp) && !keyboard.pressed(KeyCode::ShiftLeft) {
            transform.translation.y += move_factor;
        }
        if keyboard.pressed(KeyCode::ArrowDown) && !keyboard.pressed(KeyCode::ShiftLeft) {
            transform.translation.y -= move_factor;
        }

        // Pan left/right (X axis) — Arrow Left/Right
        if keyboard.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= move_factor;
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            transform.translation.x += move_factor;
        }

        // Move forward/backward (Z axis) — LShift + Arrow Up/Down
        if keyboard.pressed(KeyCode::ShiftLeft) && keyboard.pressed(KeyCode::ArrowUp) {
            transform.translation.z -= move_factor;
        }
        if keyboard.pressed(KeyCode::ShiftLeft) && keyboard.pressed(KeyCode::ArrowDown) {
            transform.translation.z += move_factor;
        }
    }
}
