//! Location search and reverse geocoding
//!
//! **Use with absolute caution.** Querying OSM can hog down
//! a Nominatim server easily. I am not responsible for any damage this
//! tool may cause.
//!
//! # Geocode
//! ```rust
//! use osm_rs::nominatim::{Config, Geocode};
//!
//! #[tokio::main]
//! async fn main() {
//!     let c: Config = Config {
//!         url: "https://nominatim.openstreetmap.org/search".to_string(),
//!         timeout: 25,
//!     };
//!
//!     let g = Geocode {
//!         q: Some("Boston".to_string()),
//!         street: None,
//!         city: None,
//!         county: None,
//!         state: None,
//!         country: None,
//!         postalcode: None,
//!     };
//!
//!     let resp = g.search(&c).await.unwrap();
//!     assert_eq!(resp[0].lat, 42.3554334);
//!     assert_eq!(resp[0].lon, -71.060511);
//! }
//! ```
//! # Reverse geocode
//! ```rust
//! use osm_rs::nominatim::{Config, ReverseGeocode};
//! #[tokio::main]
//! async fn main() {
//!    let c: Config = Config {
//!        url: "https://nominatim.openstreetmap.org/reverse".to_string(),
//!        timeout: 25,
//!    };
//!
//!    let g = ReverseGeocode {
//!        lat: 42.3554334,
//!        lon: -71.060511,
//!    };
//!
//!    let resp = g.search(&c).await.unwrap();
//!    assert_eq!(resp.osm_id, 10533284);
//! }
//! ```
use crate::overpass::BoundingBox;
use reqwest;
use serde::Deserialize;
use serde_aux::prelude::deserialize_number_from_string;
use std::collections::HashMap;

/// User agent string
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// Query configuration
#[derive(Clone)]
pub struct Config {
    pub url: String,
    pub timeout: u8,
}

/// Defines a search query
#[derive(Debug, Clone, Deserialize)]
pub struct Geocode {
    pub q: Option<String>,
    pub street: Option<String>,
    pub city: Option<String>,
    pub county: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postalcode: Option<String>,
}

/// Defines a reverse geocode query
#[derive(Debug, Clone, Deserialize)]
pub struct ReverseGeocode {
    pub lon: f64,
    pub lat: f64,
}

/// Payload returned by the Nominatim API
#[derive(Debug, Deserialize)]
pub struct GeocodeResponse {
    pub place_id: u64,
    pub license: Option<String>,
    pub osm_type: String,
    pub osm_id: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub lat: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub lon: f64,
    pub class: String,
    #[serde(rename = "type")]
    pub place_type: String,
    pub place_rank: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub importance: f64,
    pub addresstype: String,
    pub name: String,
    pub display_name: String,
    pub boundingbox: BoundingBox,
}

impl Geocode {
    pub fn new(s: String) -> Self {
        Self {
            q: Some(s),
            street: None,
            city: None,
            county: None,
            state: None,
            country: None,
            postalcode: None,
        }
    }

    /// Asynchronously search by location.
    ///
    /// # Example
    ///
    /// ```rust
    /// use osm_rs::nominatim::{Config, Geocode};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let c: Config = Config {
    ///         url: "https://nominatim.openstreetmap.org/search".to_string(),
    ///         timeout: 25,
    ///     };
    ///
    ///     let g = Geocode::new("Boston".to_string());
    ///     let resp = g.search(&c).await.unwrap();
    ///     assert_eq!(resp[0].lat, 42.3554334);
    ///     assert_eq!(resp[0].lon, -71.060511);
    /// }
    /// ```
    pub async fn search(&self, config: &Config) -> Result<Vec<GeocodeResponse>, reqwest::Error> {
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?;
        let params = self.to_params();
        let url = format!("{}?format=json", config.url);
        let res = client.get(url).query(&params).send().await?;
        let resp: Vec<GeocodeResponse> = res.json().await?;
        Ok(resp)
    }

    /// Construct GET request params
    pub fn to_params(&self) -> HashMap<&str, &String> {
        let mut params = HashMap::new();
        if let Some(q) = &self.q {
            params.insert("q", q);
        } else {
            if let Some(street) = &self.street {
                params.insert("street", street);
            }
            if let Some(city) = &self.city {
                params.insert("city", city);
            }
            if let Some(county) = &self.county {
                params.insert("county", county);
            }
            if let Some(state) = &self.state {
                params.insert("state", state);
            }
            if let Some(country) = &self.country {
                params.insert("country", country);
            }
            if let Some(postalcode) = &self.postalcode {
                params.insert("postalcode", postalcode);
            }
        }
        params
    }
}

impl ReverseGeocode {
    /// Asynchronously reverse geocode
    /// # Example
    /// ```rust
    /// use osm_rs::nominatim::{Config, ReverseGeocode};
    /// #[tokio::main]
    /// async fn main() {
    ///    let c: Config = Config {
    ///        url: "https://nominatim.openstreetmap.org/reverse".to_string(),
    ///        timeout: 25,
    ///    };
    ///
    ///    let g = ReverseGeocode {
    ///        lat: 42.3554334,
    ///        lon: -71.060511,
    ///    };
    ///
    ///    let resp = g.search(&c).await.unwrap();
    ///    assert_eq!(resp.osm_id, 10533284);
    /// }
    /// ```
    pub async fn search(&self, config: &Config) -> Result<GeocodeResponse, reqwest::Error> {
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?;

        let mut params = HashMap::new();
        params.insert("lat", self.lat.to_string());
        params.insert("lon", self.lon.to_string());

        let url = format!("{}?format=json", config.url);
        let res = client.get(url).query(&params).send().await?;
        let resp: GeocodeResponse = res.json().await?;
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_geocode() {
        let c: Config = Config {
            url: "https://nominatim.openstreetmap.org/search".to_string(),
            timeout: 25,
        };

        let g = Geocode::new("Boston".to_string());
        let resp = g.search(&c).await.unwrap();
        assert_eq!(resp[0].lat, 42.3554334);
        assert_eq!(resp[0].lon, -71.060511);
    }

    #[tokio::test]
    async fn test_reverse_geocode() {
        let c: Config = Config {
            url: "https://nominatim.openstreetmap.org/reverse".to_string(),
            timeout: 25,
        };

        let g = ReverseGeocode {
            lat: 42.3554334,
            lon: -71.060511,
        };

        let resp = g.search(&c).await.unwrap();
        assert_eq!(resp.osm_id, 10533284);
    }
}
