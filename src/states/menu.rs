use bevy::prelude::*;

use crate::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), setup_menu)
           .add_systems(Update, menu_interaction.run_if(in_state(AppState::Menu)))
           .add_systems(OnExit(AppState::Menu), cleanup_menu);
    }
}

/// Marker component for the menu camera.
#[derive(Component)]
struct MenuCamera;

/// Marker component for the menu UI root entity, used for cleanup.
#[derive(Component)]
struct MenuRoot;

/// Marker component for the Play button.
#[derive(Component)]
struct PlayButton;

/// Marker component for the Quit button.
#[derive(Component)]
struct QuitButton;

fn setup_menu(mut commands: Commands) {
    // Spawn a 2D camera for UI rendering
    commands.spawn((MenuCamera, Camera2d));

    // Root node for the menu UI
    commands
        .spawn((
            MenuRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.95)),
        ))
        .with_children(|parent| {
            // Title: "Evoli"
            parent.spawn((
                Text::new("Evoli"),
                TextFont {
                    font_size: 80.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));

            // Play button
            parent
                .spawn((
                    PlayButton,
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
                        Text::new("Play"),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 1.0, 1.0)),
                    ));
                });

            // Quit button
            parent
                .spawn((
                    QuitButton,
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
                        Text::new("Quit"),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 1.0, 1.0)),
                    ));
                });
        });
}

fn menu_interaction(
    mut next_state: ResMut<NextState<AppState>>,
    mut exit_events: EventWriter<AppExit>,
    play_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    quit_query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
) {
    for interaction in &play_query {
        if *interaction == Interaction::Pressed {
            next_state.set(AppState::InGame);
        }
    }
    for interaction in &quit_query {
        if *interaction == Interaction::Pressed {
            exit_events.send(AppExit::Success);
        }
    }
}

fn cleanup_menu(
    mut commands: Commands,
    root_query: Query<Entity, With<MenuRoot>>,
    camera_query: Query<Entity, With<MenuCamera>>,
) {
    for entity in &root_query {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &camera_query {
        commands.entity(entity).despawn_recursive();
    }
}
