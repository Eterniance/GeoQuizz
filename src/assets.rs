use bevy::prelude::*;

use crate::{
    types::{CityAssets, GuessAssets},
};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_guess_assets, setup_city_assets));
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
