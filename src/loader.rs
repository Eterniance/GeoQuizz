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
    let file = std::fs::read_to_string(path)
        .map_err(|e| GeoError::DataLoading(e.to_string()))?;

    let cities: Vec<OsmCity> =
        serde_json::from_str(&file).map_err(|e| GeoError::DataLoading(e.to_string()))?;
    for city in &cities {
        println!("City from osm: {city:?}");
    }
    Ok(cities[0].name_default.to_string())
}
