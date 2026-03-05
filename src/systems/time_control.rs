use bevy::prelude::*;

/// Controls the virtual time speed using keyboard input.
///
/// - `]` (BracketRight): speed up the simulation
/// - `[` (BracketLeft): slow down the simulation
/// - `P`: toggle pause
///
/// Speed is clamped between 0.25x and 4.0x.
pub fn time_control_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut time: ResMut<Time<Virtual>>,
) {
    if keyboard.just_pressed(KeyCode::BracketRight) {
        let current = time.relative_speed();
        let new_speed = (current * 2.0).min(4.0);
        time.set_relative_speed(new_speed);
        info!("Game speed: {:.2}x", new_speed);
    }

    if keyboard.just_pressed(KeyCode::BracketLeft) {
        let current = time.relative_speed();
        let new_speed = (current * 0.5).max(0.25);
        time.set_relative_speed(new_speed);
        info!("Game speed: {:.2}x", new_speed);
    }

    if keyboard.just_pressed(KeyCode::KeyP) {
        if time.is_paused() {
            time.unpause();
            info!("Game unpaused");
        } else {
            time.pause();
            info!("Game paused");
        }
    }
}
