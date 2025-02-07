//! # osm-rs
//!
//! Query OSM objects with Overpass and Nominatim.
//!
//! **Use with absolute caution.** Querying OSM can hog down
//! an Overpass server easily. I am not responsible for any damage this
//! tool may cause.
//!
//! # Bounding Boxes
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

pub mod nominatim;
pub mod overpass;
