//! Query OpenStreetMap nodes by attribute
//!
//! **Use with absolute caution.** Querying OSM can hog down
//! an Overpass server easily. I am not responsible for any damage this
//! tool may cause.
//!
//! # Library Example
//! ```rust
//! use osm_rs::overpass::{BoundingBox, Config};
//!
//! #[tokio::main]
//! async fn main() {
//!   let c: Config = Config {
//!       url: "https://overpass-api.de/api/interpreter".to_string(),
//!       timeout: 25,
//!       key: "amenity".to_string(),
//!       val: "cafe".to_string(),
//!   };
//!
//!   let b: BoundingBox = BoundingBox {
//!       xmin: 51.305219521963295,
//!       ymin: -0.7690429687500001,
//!       xmax: 51.82219818336938,
//!       ymax: 0.5273437500000064,
//!   };
//!
//!   let resp = b.search(&c).await.expect("failed query");
//! }
//! ```
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;
use std::collections::HashMap;

/// Major semiaxis of WGS-84 geoidal reference
const WGS84A: f64 = 6378137.0;

/// Minor semiaxis of WGS-84 geoidal reference
const WGS84B: f64 = 6356752.3;

/// Query configuration
#[derive(Debug)]
pub struct Config {
    pub url: String,
    pub timeout: u8,
    pub key: String,
    pub val: String,
}

/// Defines a bounding box by its coordinate boundaries (in radians)
#[derive(Debug, Clone, Deserialize)]
pub struct BoundingBox {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub xmin: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub ymin: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub xmax: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub ymax: f64,
}

/// Metadata returned by the Overpass API
#[derive(Serialize, Deserialize, Debug)]
pub struct OSMMetaData {
    pub timestamp_osm_base: String,
    pub copyright: String,
}

/// Node data returned by the Overpass API
#[derive(Serialize, Deserialize, Debug)]
pub struct OverpassResponse {
    pub version: f64,
    pub generator: String,
    pub osm3s: OSMMetaData,
    pub elements: Vec<Node>,
}

/// Defines an OSM node
#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
    pub tags: HashMap<String, String>,
}

/// Earth radius at a given latitude according to the WGS-84 ellipsoid
fn wgs84_earth_radius(lat: f64) -> f64 {
    let an = WGS84A * WGS84A * lat.cos();
    let bn = WGS84B * WGS84B * lat.sin();
    let ad = WGS84A * lat.cos();
    let bd = WGS84B * lat.sin();
    ((an * an + bn * bn) / (ad * ad + bd * bd)).sqrt()
}

impl<'a> BoundingBox {
    /// Construct a bounding box dist dkm away from point
    pub fn from_point(lat: f64, lon: f64, dkm: f64) -> Self {
        let dm = dkm * 1000.0;
        let erad = wgs84_earth_radius(lat);
        let prad = erad * lat.cos();
        let dx = dm / prad;
        let dy = dm / erad;

        Self {
            xmin: lon - dx,
            ymin: lat - dy,
            xmax: lon + dx,
            ymax: lat + dy,
        }
    }

    /// Asynchronously search for nodes within the bounding box by tag
    ///
    /// # Example
    ///
    /// ```rust
    /// use osm_rs::overpass::{BoundingBox, Config};
    /// #[tokio::main]
    /// async fn main() {
    ///   let c: Config = Config {
    ///       url: "https://overpass-api.de/api/interpreter".to_string(),
    ///       timeout: 25,
    ///       key: "amenity".to_string(),
    ///       val: "cafe".to_string(),
    ///   };
    ///
    ///   let b: BoundingBox = BoundingBox {
    ///       xmin: 51.305219521963295,
    ///       ymin: -0.7690429687500001,
    ///       xmax: 51.82219818336938,
    ///       ymax: 0.5273437500000064,
    ///   };
    ///
    ///   let resp = b.search(&c).await.expect("failed query");
    /// }
    /// ```
    pub async fn search(&self, config: &Config) -> Result<OverpassResponse, Error> {
        let query = format!(
            "[out:json];node[\"{}\"=\"{}\"]({},{},{},{});out center;",
            config.key, config.val, self.xmin, self.ymin, self.xmax, self.ymax
        );

        let client = Client::new();
        let resp: OverpassResponse = client
            .post(&config.url)
            .body(query)
            .send()
            .await?
            .json::<OverpassResponse>()
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_bounding_box() {
        let c: Config = Config {
            url: "https://overpass-api.de/api/interpreter".to_string(),
            timeout: 25,
            key: "amenity".to_string(),
            val: "cafe".to_string(),
        };
        let b: BoundingBox = BoundingBox {
            xmin: 51.305219521963295,
            ymin: -0.7690429687500001,
            xmax: 51.82219818336938,
            ymax: 0.5273437500000064,
        };
        let resp = b.search(&c).await.unwrap();
        assert!(resp.elements.len() > 0);
    }

    #[test]
    fn test_bounding_box_from_point() {
        let bbox = BoundingBox::from_point(42.361145, -71.057083, 10.0);
        println!(
            "({}, {}, {}, {})",
            bbox.xmin, bbox.ymin, bbox.xmax, bbox.ymax
        );
    }
}
