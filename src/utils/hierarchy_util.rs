// In Bevy 0.15, hierarchy deletion is handled natively by `Commands::entity(e).despawn_recursive()`.
// This module is kept for backwards compatibility but the function simply wraps Bevy's built-in
// recursive despawn.

use bevy::prelude::*;

/// Despawn the given entity and all of its descendants.
///
/// In Bevy 0.15, this is equivalent to `commands.entity(root).despawn_recursive()`.
/// This helper is provided for call-site compatibility with the old Amethyst codebase.
pub fn delete_hierarchy(root: Entity, commands: &mut Commands) {
    commands.entity(root).despawn_recursive();
}
