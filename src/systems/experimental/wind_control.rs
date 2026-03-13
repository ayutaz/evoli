use bevy::prelude::*;

use crate::resources::experimental::wind::Wind;
use std::f32;

/// Wind speed cannot decrease below this number.
const MIN_WIND_SPEED: f32 = 0.0;
/// Wind speed cannot increase above this number.
const MAX_WIND_SPEED: f32 = 5.0;
/// Speed with which to rotate wind direction in radians per second.
const WIND_TURN_SPEED: f32 = f32::consts::FRAC_PI_4;
/// Speed with which to increase or decrease wind speed in units per second per second.
const WIND_ACCELERATION: f32 = 2.0;

/// DebugWindControlSystem allows players to change the wind speed and direction at runtime.
pub fn debug_wind_control_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut wind: ResMut<Wind>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    let change_direction: Option<f32> = if keyboard.pressed(KeyCode::ArrowLeft) {
        Some(1.0)
    } else if keyboard.pressed(KeyCode::ArrowRight) {
        Some(-1.0)
    } else {
        None
    };

    let change_speed: Option<f32> = if keyboard.pressed(KeyCode::ArrowUp) {
        Some(1.0)
    } else if keyboard.pressed(KeyCode::ArrowDown) {
        Some(-1.0)
    } else {
        None
    };

    if change_direction.is_none() && change_speed.is_none() {
        return;
    }

    let new_angle = calc_wind_angle(change_direction, &wind, dt);
    let new_speed = calc_wind_speed(change_speed, &wind, dt);
    wind.wind = Vec2::new(new_speed * new_angle.cos(), new_speed * new_angle.sin());
    info!(
        "Changed wind vector to: ({:?},{:?}) angle={:?} speed={:?}",
        wind.wind.x, wind.wind.y, new_angle, new_speed
    );
}

fn calc_wind_angle(input_signum: Option<f32>, wind: &Wind, dt: f32) -> f32 {
    let old_wind_angle = wind.wind.y.atan2(wind.wind.x);
    if let Some(signum) = input_signum {
        old_wind_angle + signum * WIND_TURN_SPEED * dt
    } else {
        old_wind_angle
    }
}

fn calc_wind_speed(input_signum: Option<f32>, wind: &Wind, dt: f32) -> f32 {
    let magnitude = wind.wind.length();
    if let Some(signum) = input_signum {
        (magnitude + signum * WIND_ACCELERATION * dt).clamp(MIN_WIND_SPEED, MAX_WIND_SPEED)
    } else {
        magnitude
    }
}
