use bevy::{
   prelude::*,
   sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() {
   App::new()
       .add_plugins(DefaultPlugins)
       .add_systems(Startup, setup)
       .run();
}

fn setup(mut commands: Commands,
   mut meshes: ResMut<Assets<Mesh>>,
   mut materials: ResMut<Assets<ColorMaterial>>,)
   {
      commands.spawn(Camera2dBundle::default());   

      let shape = Mesh2dHandle(meshes.add(Triangle2d::new(
         Vec2::Y * 50.0,
         Vec2::new(-50.0, -50.0),
         Vec2::new(50.0, -50.0),
     )));

     let color = Color::rgb_u8(255, 17, 0);

     commands.spawn(MaterialMesh2dBundle {
      mesh: shape,
      material: materials.add(color),
      transform: Transform::from_xyz(
          // Distribute shapes from -X_EXTENT to +X_EXTENT.
          0.0,
          0.0,
          0.0,
      ),
      ..default()
  });
}
