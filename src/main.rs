use bevy::prelude::*;

use geo_quizz::{GamePlugin, SetupPlugin};

use geo_quizz::types::{City, Location};

fn main() {
    App::new()
        .add_plugins(SetupPlugin)
        .add_plugins(GamePlugin)
        .run();
}




#[derive(Resource)]
struct LocTimer(Timer);


fn find_city(time: Res<Time>, mut timer: ResMut<LocTimer>, query: Query<&Location, With<City>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for loc in &query {
            println!("At {:?}", loc.0);
        }
    }
}





