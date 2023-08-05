// https://youtu.be/4TjEo-gDgAg?list=PLVnntJRoP85JHGX7rGDu6LaF3fmDDbqyd

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub const N_ENEMIES: usize = 4;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(spawn_player)
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_enemies)
    .add_system(player_movememnt)
    .add_system(confine_player_movement)
    .run();
}

#[derive(Component)]
pub struct Player{}

#[derive(Component)]
pub struct Enemy{}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(window.width()/2.0, window.height()/2.0 , 0.0),
                texture: asset_server.load("sprites/ball_blue_large.png"),
                ..default()
            },
            Player {}
        )
    );
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
            ..default()
        }
    );
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();
    for _enemy_pos in 0..N_ENEMIES {
        let rand_x = random::<f32>() *window.width();
        let rand_y = random::<f32>() *window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
                Enemy {}
        ));
    }
}


pub fn player_movememnt(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction += Vec3::new(0.0, -1.0, 0.0)
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        player_transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

// move player to else bound ie. y_min -> y_max, x_min -> x_max etc.
pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    mut window_query: Query<&Window, With<PrimaryWindow>>
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single_mut().unwrap();

        // center of player         
        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = half_player_size;
        let y_max = window.height() - half_player_size;
        
        let mut translation = player_transform.translation;
        // x axis 
        if translation.x < x_min {
            translation.x = x_max
        } else if translation.x > x_max {
            translation.x = x_min
        }
        // y axis
        if translation.y < y_min {
            translation.y = y_max
        } else if translation.y > y_max {
            translation.y = y_min
        }

        player_transform.translation = translation;
    }
}