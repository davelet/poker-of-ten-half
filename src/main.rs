use bevy::prelude::*;
use components::*;
use resources::*;
use systems::*;
use plugins::*;

mod components;
mod systems;
mod resources;
mod plugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .insert_resource(MatchPlayerCount::One)
        .init_state::<GameState>()
        .add_systems(Startup, setup_game)
        .add_plugins((splash_plugin, menu_plugin, game_plugin))
        .run();
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<(&CardRank, &CardType, &CardPoint)>) {
    let t = timer.0.tick(time.delta());
    if t.just_finished() {
        for name in &query {
            info!("hello {} {:?} {}", name.0.rank, name.1.suite, name.2.point_type);
        }
        
        
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}
#[derive(Resource)]
struct GreetTimer(Timer);

