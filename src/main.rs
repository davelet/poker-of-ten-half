use bevy::prelude::*;
use components::{CardRank, CardType};
use entities::PokerCard;
use systems::create_deck;

mod entities;
mod components;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_systems(Startup, create_deck)
        .add_systems(Update, (update_people, greet_people).chain())
        .run();
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<(&CardRank, &CardType), With<CardType>>) {
    let t = timer.0.tick(time.delta());
    if t.just_finished() {
        for name in &query {
            info!("hello {} {:?}", name.0.rank, name.1.suite);
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

