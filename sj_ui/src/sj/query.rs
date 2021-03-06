#![allow(non_snake_case)]

use super::prelude::*;

pub struct SearchParams<'a> {
    pub max_results: usize,
    pub query: &'a str,
    pub categories: &'a [u8],
}

pub fn query(t: &SJAccess, params: SearchParams) -> FutureResponse<SearchResponse> {
    let cts = params
        .categories
        .into_iter()
        .map(|c| format!("{}", c))
        .collect::<Vec<String>>()
        .join(",");

    Box::new(sj_req(
        t,
        client::get(format!(
            "https://www.googleapis.com/sj/v2.5/query?{}",
            query_string(&[
                ("max-results", format!("{}", params.max_results).as_str()),
                ("q", params.query),
                ("ic", "true"), // in clusters
                ("ct", &cts),
                ("dv", "0"), // device version
                ("hl", "en"), // language
                ("tier", "aa"), // subscribed or not
            ])
        )),
    ))
}

/// sj#searchresponse
#[derive(Serialize, Deserialize)]
pub struct SearchResponse {
    pub kind: String,
    pub clusterDetail: Vec<ClusterDetail>,
}

#[derive(Serialize, Deserialize)]
pub struct Cluster {
    #[serde(rename = "type")]
    pub _type: String,
    pub category: String,
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClusterDetail {
    pub cluster: Cluster,
    pub displayName: Option<String>,
    pub resultToken: String,
    pub entries: Option<Vec<ClusterEntry>>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClusterEntry {
    #[serde(rename="1")]
    Track(TrackEntry),
    #[serde(rename="2")]
    Artist(ArtistEntry),
    #[serde(rename="3")]
    Album(AlbumEntry),
    #[serde(rename="4")]
    Playlist(serde_json::Value),
    #[serde(rename="5")]
    Genre(serde_json::Value),
    #[serde(rename="6")]
    Station(StationEntry),
    #[serde(rename="7")]
    Situation(serde_json::Value),
    #[serde(rename="8")]
    Video(VideoEntry),
}

// Tracks

/// type: 1
#[derive(Serialize, Deserialize)]
pub struct TrackEntry {
    pub track: Track,
    pub cluster: Vec<Cluster>,
}

/// sj#imageRef
#[derive(Serialize, Deserialize)]
pub struct ImageRef {
    pub kind: String,
    pub url: String,
    pub aspectRatio: String,
    pub autogen: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Track {
    pub kind: String,
    pub title: String,
    pub artist: String,
    pub composer: String,
    pub album: String,
    pub albumArtist: String,
    pub year: i64,
    pub trackNumber: i64,
    pub genre: String,
    pub durationMillis: String,
    pub albumArtRef: Vec<ImageRef>,
    pub discNumber: i64,
    pub estimatedSize: String,
    pub trackType: String,
    pub storeId: String,
    pub albumId: String,
    pub artistId: Vec<String>,
    pub nid: String,
    pub trackAvailableForSubscription: bool,
    pub trackAvailableForPurchase: bool,
    pub albumAvailableForPurchase: bool,
    pub explicitType: String,
}

// Artists

/// type: 2
#[derive(Serialize, Deserialize)]
pub struct ArtistEntry {
    pub artist: Artist,
    pub cluster: Vec<Cluster>,
}

#[derive(Serialize, Deserialize)]
pub struct Artist {
    pub kind: String,
    pub name: String,
    pub artistArtRef: String,
    pub artistArtRefs: Vec<ImageRef>,
    pub artistId: String,
}

// Albums

/// type: 3
#[derive(Serialize, Deserialize)]
pub struct AlbumEntry {
    pub album: Album,
    pub cluster: Vec<Cluster>,
}

#[derive(Serialize, Deserialize)]
pub struct Album {
    pub kind: String,
    pub name: String,
    pub albumArtist: String,
    pub albumArtRef: String,
    pub albumId: String,
    pub artist: String,
    pub artistId: Vec<String>,
    pub year: i64,
    pub explicitType: String,
}

// Stations

/// type: 6
#[derive(Serialize, Deserialize)]
pub struct StationEntry {
    pub station: Station,
    pub cluster: Vec<Cluster>,
}

#[derive(Serialize, Deserialize)]
pub struct CompositeArtRefs {
    pub kind: String,
    pub url: String,
    pub aspectRatio: String,
}

/// sj#radioSeed
#[derive(Serialize, Deserialize)]
pub struct RadioSeed {
    pub kind: String,
    pub curatedStationId: Option<String>,
    pub artistId: Option<String>,
    pub trackId: Option<String>,
    pub seedType: String,
}

#[derive(Serialize, Deserialize)]
pub struct Station {
    pub kind: String,
    pub name: String,
    pub description: Option<String>,
    pub seed: RadioSeed,
    pub stationSeeds: Vec<RadioSeed>,
    pub imageUrls: Vec<ImageRef>,
    pub compositeArtRefs: Option<Vec<CompositeArtRefs>>,
    pub skipEventHistory: Vec<serde_json::Value>,
    pub contentTypes: Option<Vec<String>>,
    pub byline: Option<String>,
}

// Videos

/// type: 8
#[derive(Serialize, Deserialize)]
pub struct VideoEntry {
    pub youtube_video: YoutubeVideo,
    pub score: f64,
    pub cluster: Vec<Cluster>,
}

#[derive(Serialize, Deserialize)]
pub struct Thumbnails {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Serialize, Deserialize)]
pub struct YoutubeVideo {
    pub kind: String,
    pub id: String,
    pub title: String,
    pub thumbnails: Vec<Thumbnails>,
}
