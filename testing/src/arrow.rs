use bevy::prelude::*;
use crate::{consts::*, types::{SongConfig, Speed, ArrowDirection}};

pub struct ArrowsPlugin;
impl Plugin for ArrowsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ArrowMaterialResource>()
            .insert_resource(SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_startup_system(load_arrows)
            .add_startup_system(setup_target_arrows)
            .add_system(spawn_arrows)
            .add_system(move_arrows)
            .add_system(despawn_arrows);
    }
}

#[derive(Resource, Default)]
struct ArrowMaterialResource {
    red_texture: Handle<Image>,
    blue_texture: Handle<Image>,
    green_texture: Handle<Image>,
    border_texture: Handle<Image>,
}
fn load_arrows(mut arrows: ResMut<ArrowMaterialResource>, asset_server: Res<AssetServer>) {
    arrows.red_texture = asset_server.load("images/arrow_red.png");
    println!("Loaded red arrow texture: {:?}", arrows.red_texture);
    arrows.blue_texture = asset_server.load("images/arrow_blue.png");
    arrows.green_texture = asset_server.load("images/arrow_green.png");
    arrows.border_texture = asset_server.load("images/arrow_border.png");
}

#[derive(Default, Component)]
struct Arrow {
    speed: Speed,
    direction: ArrowDirection
}

#[derive(Default, Resource)]
struct SpawnTimer(Timer);

fn spawn_arrows(
    mut commands: Commands,
    mut song_config: ResMut<SongConfig>,
    time: Res<Time>,
    textures: Res<ArrowMaterialResource>
) {
    let secs = time.elapsed_seconds_f64() - 3.;
    let secs_last = secs - time.delta_seconds_f64();

    let mut remove_counter = 0;
    for arrow in &song_config.arrows {
        println!("Elapsed time minus delta: {} and elapsed: {}", secs_last, secs);
        println!("Arrow spawn time: {}", arrow.spawn_time);

        if secs_last < arrow.spawn_time && arrow.spawn_time < secs {
            remove_counter += 1;

            let tex = match arrow.speed {
                Speed::Slow => textures.red_texture.clone(),
                Speed::Medium => textures.blue_texture.clone(),
                Speed::Fast => textures.green_texture.clone()
            };

            let mut transform = Transform::from_translation(Vec3::new(ARROW_JOURNEY_START_POSITION, arrow.direction.y(), 1.));

            transform.rotate(Quat::from_rotation_z(arrow.direction.rotation()));
            commands.spawn((SpriteBundle{ 
                sprite: Sprite { custom_size: Some(Vec2::new(140., 140.)), ..default() },  
                transform,
                texture: tex,
                ..default()
            }, Arrow {
                speed: arrow.speed,
                direction: arrow.direction
            }));
            println!("Spawned arrow with speed '{:?}' and direction '{:?}'", arrow.speed, arrow.direction);
        }
        else {
            break;
        }

    }
    
    for _ in 0..remove_counter {
        song_config.arrows.remove(0);
    }
}

fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, arrow) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * arrow.speed.value();
    }
}

#[derive(Default, Component)]
struct TargetArrow;

fn setup_target_arrows(mut commands: Commands, textures: Res<ArrowMaterialResource>) {
    use ArrowDirection::*;
    let directions = [Up, Down, Left, Right];

    for direction in directions.iter() {
        let mut transform = Transform::from_translation(Vec3::new(ARROW_JOURNEY_END_POSITION, direction.y(), 1.));
        transform.rotate(Quat::from_rotation_z(direction.rotation()));
        commands.spawn(
    (SpriteBundle{
                texture: textures.border_texture.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(140., 140.)),
                    ..default()
                },
                transform,
                ..default()
            }, TargetArrow)
        );
        println!("Spawned target arrow");
    }
}

fn despawn_arrows(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Arrow)>,
    keyboard_input: Res<Input<KeyCode>>
) {
    for (entity, transform, arrow) in query.iter() {
        let pos = transform.translation.x;

        if (ARROW_JOURNEY_END_POSITION - ARROW_CLICK_THRESHOLD..=ARROW_JOURNEY_END_POSITION + ARROW_CLICK_THRESHOLD).contains(&pos)
            && arrow.direction.key_just_pressed(&keyboard_input) 
        {
            if let Some(mut e_coms) = commands.get_entity(entity) { e_coms.despawn(); }
        }

        if pos >= 2. * ARROW_JOURNEY_END_POSITION {
            if let Some(mut e_coms) = commands.get_entity(entity) { e_coms.despawn(); }
        }
    }
}

