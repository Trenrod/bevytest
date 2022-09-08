use bevy::prelude::*;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_people)
            .add_system(greet_people)
            .insert_resource(GreetTimer {
                time: Timer::from_seconds(2.0, true),
            });
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name {
        name: "Elaina Proctor".to_string(),
    });
    commands.spawn().insert(Person).insert(Name {
        name: "Renzo Hume".to_string(),
    });
    commands.spawn().insert(Person).insert(Name {
        name: "Zayna Nieves".to_string(),
    });
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // Update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.time.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.name);
        }
    }
}

struct GreetTimer {
    time: Timer,
}

#[derive(Component)]
struct Name {
    name: String,
}

#[derive(Component)]
struct Person;
