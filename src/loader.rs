use std::path::Path;

use crate::types::GeoError;

#[derive(Debug, serde::Deserialize)]
struct OsmCity {
    #[serde(rename = "name:default")]
    name_default: String,
    #[serde(rename = "name:fr")]
    name_fr: Option<String>,
    #[serde(rename = "name:nl")]
    name_nl: Option<String>,
    lat: f64,
    lon: f64,
}

pub fn load_database<P: AsRef<Path>>(path: P) -> Result<String, GeoError> {
    let file = std::fs::read_to_string(path).map_err(|e| GeoError::DataLoading(e.to_string()))?;

    let cities: Vec<OsmCity> =
        serde_json::from_str(&file).map_err(|e| GeoError::DataLoading(e.to_string()))?;
    for city in &cities {
        println!("City from osm: {city:?}");
    }
    Ok(cities[0].name_default.to_string())
}

const fn oms_to_local_x(x_oms: f32) -> f32 {
    let a = 218.975_57;
    let b = -973.915_5;
    a * x_oms + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oms_to_local_x() {
        // Soignies
        let x_oms_soignies = 4.0685604_f32;
        let expected_soignies = -83.0_f32;
        let result_soignies = oms_to_local_x(x_oms_soignies);
        assert!(
            (result_soignies - expected_soignies).abs() < 1.0,
            "Soignies: expected ≈ {}, obtained {}",
            expected_soignies,
            result_soignies
        );

        // Liège
        let x_oms_liege = 5.5736112_f32;
        let expected_liege = 216.0_f32;
        let result_liege = oms_to_local_x(x_oms_liege);
        assert!(
            (result_liege - expected_liege).abs() < 1.0,
            "Liège: expected ≈ {}, obtained {}",
            expected_liege,
            result_liege
        );
    }
}
