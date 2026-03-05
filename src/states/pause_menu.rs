use bevy::prelude::*;

use crate::AppState;
use crate::GamePlayState;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        // Toggle pause with Escape while in-game
        app.add_systems(Update, pause_toggle.run_if(in_state(AppState::InGame)));
        // Show/hide the pause menu overlay based on GamePlayState
        app.add_systems(OnEnter(GamePlayState::Paused), show_pause_menu);
        app.add_systems(OnExit(GamePlayState::Paused), hide_pause_menu);
        // Handle pause menu button interactions
        app.add_systems(
            Update,
            pause_menu_interaction.run_if(in_state(GamePlayState::Paused)),
        );
    }
}

/// Marker component for the pause menu UI root, used for cleanup.
#[derive(Component)]
struct PauseMenuRoot;

/// Marker for the Resume button.
#[derive(Component)]
struct ResumeButton;

/// Marker for the Main Menu button.
#[derive(Component)]
struct MainMenuButton;

fn pause_toggle(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GamePlayState>>,
    mut next_state: ResMut<NextState<GamePlayState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GamePlayState::Running => next_state.set(GamePlayState::Paused),
            GamePlayState::Paused => next_state.set(GamePlayState::Running),
        }
    }
}

fn show_pause_menu(mut commands: Commands) {
    // Semi-transparent overlay with pause menu buttons
    commands
        .spawn((
            PauseMenuRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            // High z-index to render above game UI
            GlobalZIndex(100),
        ))
        .with_children(|parent| {
            // "Paused" title
            parent.spawn((
                Text::new("Paused"),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 1.0)),
            ));

            // Resume button
            parent
                .spawn((
                    ResumeButton,
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.6, 0.2)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Resume"),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 1.0, 1.0)),
                    ));
                });

            // Main Menu button
            parent
                .spawn((
                    MainMenuButton,
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.6, 0.2, 0.2)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Main Menu"),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 1.0, 1.0)),
                    ));
                });
        });
}

fn hide_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn pause_menu_interaction(
    mut next_gameplay_state: ResMut<NextState<GamePlayState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    resume_query: Query<&Interaction, (Changed<Interaction>, With<ResumeButton>)>,
    menu_query: Query<&Interaction, (Changed<Interaction>, With<MainMenuButton>)>,
) {
    for interaction in &resume_query {
        if *interaction == Interaction::Pressed {
            next_gameplay_state.set(GamePlayState::Running);
        }
    }
    for interaction in &menu_query {
        if *interaction == Interaction::Pressed {
            // Return to Running first (so OnExit(Paused) fires and cleans up pause UI),
            // then switch to Menu (which triggers OnExit(InGame) to clean up game entities).
            next_gameplay_state.set(GamePlayState::Running);
            next_app_state.set(AppState::Menu);
        }
    }
}
