use serde::{Deserialize, Serialize};

use crate::models::geo_coords::GeoCoords;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GeoDistance {
    pub distance: f64,
    pub geo_tag: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub country: Option<String>,
    pub geo_coordinates: Option<GeoCoords>,
    pub locality: Option<String>,
    pub postal_code: String,
    pub region: Option<String>,
    pub street: Option<String>,
    pub geo_distances: Option<Vec<GeoDistance>>,
    pub geo_tags: Option<Vec<String>>,
}
