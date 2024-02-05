use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PeoplePlugin))
        .run();
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (
                print_names,
                people_with_jobs,
                people_ready_for_hire,
                person_does_job),
            );
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(
        (
            Person { name: "Alex".to_string() },
            Employed { job: Job::Programmer },
        ));
    commands.spawn(
        (
            Person { name: "Eve".to_string() },
            Employed { job: Job::Accountant },
        )
    );
    commands.spawn(
        (
            Person { name: "Joe".to_string() },
            Employed { job: Job::Unemployed },
        )
    );
    commands.spawn(
        (
            Person { name: "Claire".to_string() },
            Employed { job: Job::Artist },
        )
    );

    commands.spawn(
        Person { name: "Max".to_string() },
    );
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}

pub fn people_with_jobs(
    person_query: Query<&Person, With<Employed>>) {
    for person in person_query.iter() {
        println!("{} has a job.", person.name);
    }
}

pub fn people_ready_for_hire(
    person_query: Query<&Person, Without<Employed>>) {
    for person in person_query.iter() {
        println!("{} is ready to be hired.", person.name);
    }
}

pub fn person_does_job(
    person_query: Query<(&Person, &Employed)>,
) {
    for (person, employed) in person_query.iter() {
        let job_name: &str = match employed.job {
            Job::Programmer => "Programmer",
            Job::Artist => "Artist",
            Job::Accountant => "Accountant",
            Job::Unemployed => "Unemployed",
        };
        println!("{0} is a {1}", person.name, job_name);
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}

#[derive(Component)]
pub struct Employed {
    pub job: Job,
}

#[derive(Debug)]
pub enum Job {
    Programmer,
    Artist,
    Accountant,
    Unemployed,
}