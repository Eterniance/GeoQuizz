use bevy::math::Vec2;
use std::f32::consts::PI;
use std::path::Path;

use crate::types::{BundleCity, City, GeoError};

#[derive(Debug, serde::Deserialize)]
struct OsmCity {
    #[serde(rename = "name:default")]
    name_default: String,
    #[serde(rename = "name:fr")]
    name_fr: Option<String>,
    #[serde(rename = "name:nl")]
    #[allow(unused)]
    name_nl: Option<String>,
    lat: f32,
    lon: f32,
}

pub fn load_database<P: AsRef<Path>>(path: P) -> Result<Vec<BundleCity>, GeoError> {
    let file = std::fs::read_to_string(path).map_err(|e| GeoError::DataLoading(e.to_string()))?;

    let oms_cities: Vec<OsmCity> =
        serde_json::from_str(&file).map_err(|e| GeoError::DataLoading(e.to_string()))?;
    let cities: Vec<BundleCity> = oms_cities.into_iter().map(|city| city.into()).collect();
    Ok(cities)
}

/// Convert geographic coordinates (longitude, latitude) in degrees
/// into Mercator projection coordinates (radians-based, unscaled).
fn mercator(long: f32, lat: f32) -> (f32, f32) {
    // Constant: central longitude (4.5° in radians)
    const X0: f32 = 4.5_f32.to_radians();

    let x = long.to_radians();
    let y = lat.to_radians();

    let new_x = x - X0;
    let new_y = (0.25 * PI + 0.5 * y).tan().ln();

    (new_x, new_y)
}

// linear transformation of the x coordinates
const fn x_transformation(x: f32) -> f32 {
    let a = 11_173.297;
    let b = 1.215_384_8;
    a * x + b
}

impl From<OsmCity> for BundleCity {
    fn from(value: OsmCity) -> Self {
        let (x, y) = oms_to_local(value.lon, value.lat);
        let loc = Vec2::new(x, y);
        let name;
        if let Some(name_fr) = value.name_fr {
            name = name_fr;
        } else {
            name = value.name_default;
        }
        BundleCity {
            city: City,
            name: name.into(),
            loc: loc.into(),
        }
    }
}

// linear transformation of the y coordinates
const fn y_transformation(y: f32) -> f32 {
    let a = 10_760.031;
    let b = -11_030.805;
    a * y + b
}

fn oms_to_local(long: f32, lat: f32) -> (f32, f32) {
    let (x, y) = mercator(long, lat);
    (x_transformation(x), y_transformation(y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oms_to_local_x() {
        // Soignies
        let x_oms_soignies = 4.0685604_f32;
        let expected_soignies_x = -83.0_f32;
        let y_oms_soignies = 50.579203_f32;
        let expected_soignies_y = 14_f32;
        let (x, y) = oms_to_local(x_oms_soignies, y_oms_soignies);
        assert!(
            (x - expected_soignies_x).abs() < 1.0,
            "Soignies X: expected ≈ {}, obtained {}",
            expected_soignies_x,
            x
        );

        assert!(
            (y - expected_soignies_y).abs() < 1.0,
            "Soignies Y: expected ≈ {}, obtained {}",
            expected_soignies_y,
            y
        );

        // Liège
        // let x_oms_liege = 5.5736112_f32;
        // let expected_liege = 216.0_f32;
        // let result_liege = oms_to_local_x(x_oms_liege);
        // assert!(
        //     (result_liege - expected_liege).abs() < 1.0,
        //     "Liège: expected ≈ {}, obtained {}",
        //     expected_liege,
        //     result_liege
        // );
    }
}
