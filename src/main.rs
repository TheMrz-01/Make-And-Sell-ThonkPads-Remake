use bevy::{
   prelude::*,
   sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() {
   App::new()
       .add_plugins(DefaultPlugins)
       .add_systems(Startup, setup)
       .add_systems(Update, movement)
       .run();
}

#[derive(Component)]
struct speed(u8);

fn setup(mut commands: Commands,
    asset_server: Res<AssetServer>)
   {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("Gokhan Hitler.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        speed(8),
    ));

    commands.spawn(Camera2dBundle::default());   
}

fn movement(keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,mut player: Query<(&mut speed, &mut Transform)>
){
    
}
