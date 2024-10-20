use bevy::prelude::*;

/// Resource describing the center marker
#[derive(Resource)]
pub struct CenterMarkerResource {
    /// The color of the center marker
    color: Color,

    /// Size of the center marker in pixels
    scale: f32,

    /// Layer (z-height) for the center marker
    layer: f32,
}

impl FromWorld for CenterMarkerResource {
    fn from_world(_world: &mut World) -> Self {
        Self {
            color: Color::srgb(1., 0., 0.),
            scale: 2.,
            layer: 100.,
        }
    }
}

/// Marker struct tagging center entities
#[derive(Component)]
pub struct CenterEntity;

/// Center marker plugin
pub struct CenterMarkerPlugin;

impl Plugin for CenterMarkerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CenterMarkerResource>()
            .add_systems(Startup, spawn_center_system);
    }
}

/// System to spawn a center entity
pub fn spawn_center_system(mut commands: Commands, marker: Res<CenterMarkerResource>) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: marker.color,
                ..default()
            },
            transform: Transform {
                scale: Vec3::splat(marker.scale),
                translation: Vec3::new(0., 0., marker.layer),
                ..default()
            },
            ..default()
        },
        CenterEntity,
    ));
}

/// System to despawn center entities (likely only one)
pub fn despawn_center_system(
    to_despawn: Query<Entity, With<CenterEntity>>,
    mut commands: Commands,
) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
