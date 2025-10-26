use std::path::PathBuf;

use bevy::{prelude::*};

use crate::{load_database, types::{BundleCity, City, CityAssets, GuessAssets, Location}};



pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_guess_assets, setup_city_assets, add_city));
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

fn add_city(mut commands: Commands) {
    let path: PathBuf = ["database", "belgium_cities.json"].iter().collect();
    let db = load_database(path).unwrap();
    commands.spawn(BundleCity {
        city: City,
        name: Name::new("Soignies".to_string()),
        loc: Location(Vec2::new(-89.0, 13.0)),
    });
}




