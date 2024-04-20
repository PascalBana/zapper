use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_player))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Option::Some(Vec2::new(100.0, 100.0)),
            ..Default::default()
        },
        ..default()
    });
}