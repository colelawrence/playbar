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
                ("ic", "true"),
                ("ct", &cts),
                ("dv", "0"),
                ("hl", "en"),
                ("tier", "aa"),
            ])
        )),
    ))
}

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
    Tracks(TrackEntry),
    #[serde(rename="2")]
    Artists(ArtistEntry),
    #[serde(rename="3")]
    Albums(AlbumEntry),
    #[serde(rename="4")]
    Playlists(serde_json::Value),
    #[serde(rename="5")]
    Genres(serde_json::Value),
    #[serde(rename="6")]
    Stations(StationEntry),
    #[serde(rename="7")]
    Situations(serde_json::Value),
    #[serde(rename="8")]
    Videos(VideoEntry),
}

// Tracks

/// type: 1
#[derive(Serialize, Deserialize)]
pub struct TrackEntry {
    pub track: Track,
    pub cluster: Vec<Cluster>,
}

#[derive(Serialize, Deserialize)]
pub struct AlbumArtRef {
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
    pub albumArtRef: Vec<AlbumArtRef>,
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
    pub artistArtRefs: Vec<ArtistArtRefs>,
    pub artistId: String,
}

#[derive(Serialize, Deserialize)]
pub struct ArtistArtRefs {
    pub kind: String,
    pub url: String,
    pub aspectRatio: String,
    pub autogen: bool,
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

#[derive(Serialize, Deserialize)]
pub struct ImageUrls {
    pub kind: String,
    pub url: String,
    pub aspectRatio: String,
    pub autogen: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Seed {
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
    pub seed: Seed,
    pub stationSeeds: Vec<Seed>,
    pub imageUrls: Vec<ImageUrls>,
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
