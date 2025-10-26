use std::path::PathBuf;

use bevy::prelude::*;

use crate::{
    load_database,
    types::{CityAssets, GuessAssets, GuessSet},
};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (setup_guess_assets, setup_city_assets, init_guess_set),
        );
    }
}

fn setup_guess_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Circle::new(2.5));
    let material = materials.add(Color::WHITE);

    commands.insert_resource(GuessAssets { mesh, material });
}

fn setup_city_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Circle::new(2.5));
    let material = materials.add(Color::srgb(255., 0., 0.));
    commands.insert_resource(CityAssets { mesh, material });
}

fn init_guess_set(mut commands: Commands) {
    let path: PathBuf = ["database", "belgium_cities.json"].iter().collect();
    let cities = load_database(path).expect("database should exist");
    let city = cities[0].clone();
    commands.spawn(city);

    let guess_set = GuessSet { cities };
    commands.insert_resource(guess_set);
}
