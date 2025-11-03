use std::path::PathBuf;

use crate::loader::load_database;
use bevy::{
    ecs::{bundle::Bundle, component::Component, resource::Resource},
    math::Vec2,
    prelude::*,
};
use rand::seq::IteratorRandom;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GeoError {
    #[error("Error while loading database {0}")]
    DataLoading(String),
}

#[derive(Component, Debug, Clone)]
pub struct Location(pub Vec2);

impl From<Vec2> for Location {
    fn from(value: Vec2) -> Self {
        Self(value)
    }
}

#[derive(Component, Debug, Clone)]
pub struct City;

#[derive(Bundle, Debug, Clone)]
pub struct BundleCity {
    pub city: City,
    pub name: Name,
    pub loc: Location,
}

#[derive(Debug, Resource)]
pub struct GuessSet {
    all_cities: Vec<BundleCity>,
    pub to_guess: Vec<BundleCity>,
}

impl Default for GuessSet {
    fn default() -> Self {
        let path: PathBuf = ["database", "belgium_cities.json"].iter().collect();
        let all_cities = load_database(path).expect("Should exists");
        let mut rng = rand::rng();

        let to_guess = all_cities
            .iter()
            .choose_multiple(&mut rng, 10)
            .into_iter()
            .cloned()
            .collect();
        Self {
            all_cities,
            to_guess,
        }
    }
}

#[derive(Resource)]
pub struct GuessAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}

#[derive(Resource)]
pub struct CityAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}

#[derive(Component, Debug)]
pub enum GuessType {
    Name(String),
    Location(Vec2),
}

#[derive(Event)]
pub struct ValidatedGuess;

#[derive(Event)]
pub struct SpawnCity;

#[derive(Component)]
pub struct CityNameToGuess;
