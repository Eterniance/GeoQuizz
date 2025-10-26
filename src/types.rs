use bevy::{
    prelude::*,
    ecs::{bundle::Bundle, component::Component, resource::Resource},
    math::Vec2,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GeoError {
    #[error("Error while loading database {0}")]
    DataLoading(String),
}

#[derive(Component, Debug)]
pub struct Location(pub Vec2);

#[derive(Component, Debug)]
pub struct City;

#[derive(Bundle, Debug)]
pub struct BundleCity {
    pub city: City,
    pub name: Name,
    pub loc: Location,
}

#[derive(Debug, Resource)]
pub struct GuessSet {
    pub cities: Vec<BundleCity>
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

