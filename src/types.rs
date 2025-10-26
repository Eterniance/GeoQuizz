use bevy::{
    ecs::{bundle::Bundle, component::Component, resource::Resource},
    math::Vec2,
    prelude::*,
};
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
    pub cities: Vec<BundleCity>,
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
