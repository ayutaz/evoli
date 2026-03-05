use bevy::prelude::*;

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
