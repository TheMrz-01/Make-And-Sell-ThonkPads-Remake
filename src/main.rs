use std::f32::consts::E;

use bevy::{ecs::component, prelude::*};

fn main() {
   App::new()
       .add_plugins(DefaultPlugins)
       .add_systems(Startup, setup)
       .add_systems(Update, move_player)
       .run();
}

#[derive(Component)]
struct Speed{
    normalSpeed: f32,
    sprintSpeed: f32,
}


#[derive(Bundle)]
struct PlayerBundle{
    speed: Speed,
    p_sprite: SpriteBundle,
}


fn setup(mut commands: Commands,
    asset_server: Res<AssetServer>)
   {
    commands.spawn(Camera2dBundle::default());  

    commands.spawn(
        PlayerBundle{
            speed: Speed{
                normalSpeed: 150.,
                sprintSpeed: 200.,
            },
            p_sprite: SpriteBundle {
                texture: asset_server.load("GokhanHitler.png"),
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()                
            }
        }
    );


}

fn move_player(keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Speed, &mut Transform)>, time: Res<Time>,
){
    for (speed, mut transform) in query.iter_mut(){
        let mut movement_speed = speed.normalSpeed;
        if keyboard_input.pressed(KeyCode::ShiftLeft) {movement_speed = speed.sprintSpeed;}
        
        if keyboard_input.pressed(KeyCode::KeyW) {
            transform.translation.y +=  movement_speed * time.delta_seconds()
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            transform.translation.y -=  movement_speed * time.delta_seconds()
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.translation.x +=  movement_speed * time.delta_seconds()
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.translation.x -=  movement_speed * time.delta_seconds()
        }
    }
}
