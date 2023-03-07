use bevy::{
    prelude::*,
    window::{PresentMode},
};

mod arrow;
mod consts;
mod types;
use arrow::ArrowsPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa{ samples: 4 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {    
                title: "Rhythm!".to_string(),
                width: 800.,
                height: 600.,
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_plugin(ArrowsPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    let song_config = types::load_config();
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(song_config);
}
