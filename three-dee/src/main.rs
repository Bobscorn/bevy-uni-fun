use std::ops::Mul;

use bevy::{prelude::*, render::render_resource::{TextureDimension, TextureFormat, Extent3d}};

fn say_hi() {
    println!("Hello, World!");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands)
{
    commands.spawn((Person, Name("Bob".to_string())));
    commands.spawn((Person, Name("Steve".to_string())));
    commands.spawn((Person, Name("Nijkie".to_string())));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>)
{
    if timer.0.tick(time.delta()).just_finished()
    {
        for name in query.iter()
        {
            println!("Wassup {}", name.0);
        }
    }
}



pub struct HelloPlugin;

impl Plugin for HelloPlugin
{
    fn build(&self, app: &mut App)
    {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(say_hi)
            .add_startup_system(add_people)
            .add_system(greet_people);
    }
}


pub struct ThreeDTestPlugin;

impl Plugin for ThreeDTestPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_startup_system(add_3d);
    }
}

fn add_3d(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut textures: ResMut<Assets<Image>>) {
    
    let tex = textures.add(uv_debug_texture());

    // spawn a cube
    commands.spawn(PbrBundle{ 
        mesh: meshes.add(shape::Cube{ size: 1.5 }.into()),
        material: materials.add(StandardMaterial{ 
            base_color: Color::rgb(0.3, 5.0, 0.3),
            base_color_texture: Some(tex.clone()),
            ..default()
        }),
        ..default()
    });

    // spawn a plane
    commands.spawn(PbrBundle{
        mesh: meshes.add(shape::Plane{ size: 0.5 }.into()),
        material: materials.add(StandardMaterial{ 
            base_color: Color::rgb(0.3, 5.0, 0.3),
            base_color_texture: Some(tex.clone()),
            ..default()
        }),
        ..default()
    });

    // Add a light
    commands.spawn(PointLightBundle{
        point_light: PointLight {
            intensity: 9000.,
            range: 100.,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    commands.spawn(Camera3dBundle{ 
        transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1.0, 0.), Vec3::Y),
        ..default()
    });
}

fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}

fn move_camera(mut query: Query<(&mut Transform), With<Camera>>, input: Res<Input<KeyCode>>) {
    for mut trans in query.iter_mut() {
        let mut movement: Vec2 = Vec2::new(0.0, 0.0); 
        let mut moved = false;
        if input.pressed(KeyCode::W) {
            movement.y += 1.;
            moved = true;
        }
        if input.pressed(KeyCode::S) {
            movement.y -= 1.;
            moved = true;
        }
        if input.pressed(KeyCode::A) {
            movement.x -= 1.;
            moved = true;
        }
        if input.pressed(KeyCode::D) {
            movement.x += 1.;
            moved = true;
        }
        if !moved {
            continue;
        }
        movement.normalize();
        *trans = trans.mul_transform(Transform::from_translation(Vec3::new(movement.x, 0., movement.y)));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .add_plugin(ThreeDTestPlugin)
        .add_system(move_camera)
        .run();
}
