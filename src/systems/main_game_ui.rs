use bevy::prelude::*;

// ---------------------------------------------------------------------------
// Marker components for UI buttons
// ---------------------------------------------------------------------------

#[derive(Component)]
pub struct PauseButton;

#[derive(Component)]
pub struct SpeedUpButton;

#[derive(Component)]
pub struct SlowDownButton;

#[derive(Component)]
pub struct MenuButton;

/// Root node of the in-game UI – used for cleanup when leaving the game state.
#[derive(Component)]
pub struct GameUiRoot;

/// Marker placed on the `Text` child of the pause button so we can toggle
/// its label between "Pause" and "Play".
#[derive(Component)]
pub struct PauseButtonText;

// ---------------------------------------------------------------------------
// Events emitted by UI buttons (consumed by other systems)
// ---------------------------------------------------------------------------

/// Fired when the player presses the Speed-Up button.
#[derive(Event)]
pub struct SpeedUpEvent;

/// Fired when the player presses the Slow-Down button.
#[derive(Event)]
pub struct SlowDownEvent;

/// Fired when the player presses the Pause / Play button.
#[derive(Event)]
pub struct TogglePauseEvent;

/// Fired when the player presses the Menu button.
#[derive(Event)]
pub struct MenuEvent;

// ---------------------------------------------------------------------------
// Setup / teardown
// ---------------------------------------------------------------------------

/// Spawns the bottom-bar game UI with Pause, >>, <<, and Menu buttons.
pub fn setup_game_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Auto,
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(10.0),
                ..default()
            },
            GameUiRoot,
        ))
        .with_children(|parent| {
            spawn_button(parent, "Pause", PauseButton, true);
            spawn_button(parent, ">>", SpeedUpButton, false);
            spawn_button(parent, "<<", SlowDownButton, false);
            spawn_button(parent, "Menu", MenuButton, false);
        });
}

/// Helper – spawns a single button entity with a text child.
///
/// If `mark_text` is `true` the text entity also receives `PauseButtonText` so
/// we can find it later to toggle the label.
fn spawn_button(parent: &mut ChildBuilder, label: &str, marker: impl Component, mark_text: bool) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(120.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            marker,
        ))
        .with_children(|btn| {
            let mut text_cmd = btn.spawn((
                Text::new(label.to_string()),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            if mark_text {
                text_cmd.insert(PauseButtonText);
            }
        });
}

/// Despawns the entire game UI hierarchy.
pub fn cleanup_game_ui(mut commands: Commands, query: Query<Entity, With<GameUiRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

// ---------------------------------------------------------------------------
// Interaction handling
// ---------------------------------------------------------------------------

/// Reacts to button clicks and emits the corresponding events.
///
/// * Pause  – sends `TogglePauseEvent` and toggles the button label.
/// * >>     – sends `SpeedUpEvent`.
/// * <<     – sends `SlowDownEvent`.
/// * Menu   – sends `MenuEvent`.
pub fn game_ui_interaction(
    pause_query: Query<&Interaction, (Changed<Interaction>, With<PauseButton>)>,
    speed_up_query: Query<&Interaction, (Changed<Interaction>, With<SpeedUpButton>)>,
    slow_down_query: Query<&Interaction, (Changed<Interaction>, With<SlowDownButton>)>,
    menu_query: Query<&Interaction, (Changed<Interaction>, With<MenuButton>)>,
    mut pause_text_query: Query<&mut Text, With<PauseButtonText>>,
    mut ev_toggle_pause: EventWriter<TogglePauseEvent>,
    mut ev_speed_up: EventWriter<SpeedUpEvent>,
    mut ev_slow_down: EventWriter<SlowDownEvent>,
    mut ev_menu: EventWriter<MenuEvent>,
) {
    // Pause / Play
    for interaction in &pause_query {
        if *interaction == Interaction::Pressed {
            ev_toggle_pause.send(TogglePauseEvent);
            // Toggle button label
            for mut text in &mut pause_text_query {
                let current = text.0.clone();
                if current == "Pause" {
                    text.0 = "Play".to_string();
                } else {
                    text.0 = "Pause".to_string();
                }
            }
        }
    }

    // Speed up
    for interaction in &speed_up_query {
        if *interaction == Interaction::Pressed {
            ev_speed_up.send(SpeedUpEvent);
        }
    }

    // Slow down
    for interaction in &slow_down_query {
        if *interaction == Interaction::Pressed {
            ev_slow_down.send(SlowDownEvent);
        }
    }

    // Menu
    for interaction in &menu_query {
        if *interaction == Interaction::Pressed {
            ev_menu.send(MenuEvent);
        }
    }
}

/// Visual feedback – changes button colour on hover / press.
pub fn button_visual_feedback(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut bg) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *bg = BackgroundColor(Color::srgb(0.4, 0.4, 0.4));
            }
            Interaction::Hovered => {
                *bg = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
            }
            Interaction::None => {
                *bg = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
            }
        }
    }
}
