use bevy::{color::palettes::basic::{BLACK, RED}, prelude::*};

use crate::types::{CityAssets, GuessAssets};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (setup_guess_assets, setup_city_assets, setup_texts),
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

fn setup_texts(mut commands: Commands) {
    commands
        .spawn((
            // `Text` or `Text2d` are needed, and will provide default instances
            // of the following components.
            Text::new("Find "),
            TextColor(BLACK.into()),
        ))
        .with_child((
            // Children must be `TextSpan`, not `Text` or `Text2d`.
            TextSpan::default(),
            CityNameToGuess,
            TextColor(RED.into()),
        ));
}

#[derive(Component)]
pub struct CityNameToGuess;
