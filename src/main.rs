use std::u8;

use bevy::{math::f32, prelude::*};

fn main() {
   App::new()
       .add_plugins(DefaultPlugins)
       .add_systems(Startup, setup)
       .add_systems(Update, movement)
       .run();
}

#[derive(Component)]
struct Speed(f32);
#[derive(Component)]
struct sprintSpeed(f32);

fn setup(mut commands: Commands,
    asset_server: Res<AssetServer>)
   {
    commands.spawn(Camera2dBundle::default());  

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("GokhanHitler.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Speed(100.),
        sprintSpeed(200.),
    ));


}

fn movement(keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,mut player: Query<(&mut sprintSpeed,&mut Speed, &mut Transform)>
){
    for (mut sprintspeed,mut speed,mut transform) in &mut player{
        if keyboard_input.pressed(KeyCode::ShiftLeft) {speed.0 = sprintspeed.0;}
        
        if keyboard_input.pressed(KeyCode::KeyW) {
            transform.translation.y +=  speed.0 * time.delta_seconds()
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            transform.translation.y -=  speed.0 * time.delta_seconds()
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.translation.x +=  speed.0 * time.delta_seconds()
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.translation.x -=  speed.0 * time.delta_seconds()
        }
    }
}
