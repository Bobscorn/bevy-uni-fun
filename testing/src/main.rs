use bevy::prelude::*;

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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
