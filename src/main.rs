use bevy::prelude::*;

use geo_quizz::{GamePlugin, SetupPlugin};

fn main() {
    App::new()
        .add_plugins(SetupPlugin)
        .add_plugins(GamePlugin)
        .run();
}
