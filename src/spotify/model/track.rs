
use serde_json;

use std::collections::HashMap;

use super::artist::Artist;
use super::image::Image;
use super::album::Item;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Track {
    pub album: Item,
    pub artists: Vec<Artist>,
    pub available_markets: Vec<String>,
    pub disc_number: i32,
    pub duration_ms: u32,
    pub external_ids: HashMap<String, String>,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub name: String,
    pub popularity: i32,
    pub preview_url: String,
    pub track_number: u32,
    #[serde(rename = "type")]
    pub _type: String,
    pub uri: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tracks {
    pub tracks: Vec<Track>,
}
