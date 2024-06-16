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
struct player;

#[derive(Component)]
struct position {x: f32,y: f32,z: f32}

#[derive(Component)]
struct speed(u8);

fn setup(mut commands: Commands,
   mut meshes: ResMut<Assets<Mesh>>,
   mut materials: ResMut<Assets<ColorMaterial>>,)
   {
    commands.spawn((player,speed(8)));
    commands.spawn((player,position{x: 0.0,y: 0.0,z: 0.0}));

    commands.spawn(Camera2dBundle::default());   

    let shape: Mesh2dHandle = Mesh2dHandle(meshes.add(Triangle2d::new(
        Vec2::Y * 50.0,
        Vec2::new(-50.0, -50.0),
        Vec2::new(50.0, -50.0),
    )));

    let color = Color::rgb_u8(255, 17, 0);

    commands.spawn(MaterialMesh2dBundle {
        mesh: shape,
        material: materials.add(color),
        transform: Transform::from_xyz(
            0.0,
            0.0,
            0.0,
        ),
        ..default()
    });

}

fn movement(keyboard_input: Res<ButtonInput<KeyCode>>,
    p_position: Query<&mut position, With<player>>,
    p_speed: Query<&speed, With<player>>
){
    if keyboard_input.pressed(KeyCode::KeyW){
        p_position = 150. * time.delta_seconds()
    }
    if keyboard_input.pressed(KeyCode::KeyS){

    }
    if keyboard_input.pressed(KeyCode::KeyD){

    }
    if keyboard_input.pressed(KeyCode::KeyA){

    }
}
