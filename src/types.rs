use bevy::{
    ecs::{bundle::Bundle, component::Component},
    math::Vec2,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GeoError {
    #[error("Error while loading database {0}")]
    DataLoading(String),
}

#[derive(Component, Debug)]
pub struct Name(pub String);

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
