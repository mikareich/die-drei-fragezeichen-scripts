use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    #[serde(rename = "titel")]
    pub title: String,
    pub start: u64,
    pub end: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    #[serde(rename = "rolle")]
    pub role: String,
    #[serde(rename = "sprecher")]
    pub speaker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Links {
    pub json: String,
    pub ffmetadata: String,
    pub cover: String,
    pub cover_itunes: Option<String>,
    pub cover_kosmos: Option<String>,
    pub dreifragezeichen: String,
    #[serde(rename = "appleMusic")]
    pub apple_music: String,
    pub spotify: String,
    pub bookbeat: String,
    #[serde(rename = "amazonMusic")]
    pub amazon_music: String,
    pub amazon: String,
    #[serde(rename = "youTubeMusic")]
    pub youtube_music: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ids {
    #[serde(rename = "dreimetadaten")]
    pub three_metadata: u32,
    #[serde(rename = "appleMusic")]
    pub apple_music: String,
    pub spotify: String,
    pub bookbeat: String,
    #[serde(rename = "amazonMusic")]
    pub amazon_music: String,
    pub amazon: String,
    #[serde(rename = "youTubeMusic")]
    pub youtube_music: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    #[serde(rename = "titel")]
    pub title: String,
    pub start: u64,
    pub end: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    pub tracks: Vec<Track>,
    #[serde(rename = "ripLog")]
    pub rip_log: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeMetadata {
    #[serde(rename = "nummer")]
    pub number: u32,
    #[serde(rename = "titel")]
    pub title: String,
    #[serde(rename = "autor")]
    pub author: String,
    #[serde(rename = "hörspielskriptautor")]
    pub script_author: String,
    #[serde(rename = "kurzbeschreibung")]
    pub short_description: Option<String>,
    #[serde(rename = "beschreibung")]
    pub description: String,
    #[serde(rename = "veröffentlichungsdatum")]
    pub release_date: String,
    #[serde(rename = "kapitel")]
    pub chapters: Vec<Chapter>,
    #[serde(rename = "sprechrollen")]
    pub roles: Vec<Role>,
    pub links: Links,
    pub ids: Ids,
    #[serde(rename = "medien")]
    pub media: Vec<Media>,
}
