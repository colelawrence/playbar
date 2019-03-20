#![allow(non_snake_case)]

use super::prelude::*;

pub enum ItemKind {
    Track,
    Station,
    Album,
}

pub struct SearchParams<'a> {
    pub max_results: usize,
    pub query: &'a str,
    pub categories: &'a [u8],
}

pub fn query(t: &SJAccess, params: SearchParams) -> FutureResponse<SearchResponse> {
    let cts = params.categories.into_iter().map(|c| format!("{}", c))
        .collect::<Vec<String>>()
        .join(",");

    Box::new(
        sj_req(t,
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
        )))
    )
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

#[derive(Serialize, Deserialize)]
pub struct CompositeArtRefs {
    pub kind: String,
    pub url: String,
    pub aspectRatio: String,
}

#[derive(Serialize, Deserialize)]
pub struct Entries {
    #[serde(rename = "type")]
    pub _type: String,
    pub artist: Option<Artist>,
    pub playlist: Option<Playlist>,
    pub album: Option<Album>,
    pub series: Option<Series>,
    pub youtube_video: Option<YoutubeVideo>,
    pub score: Option<f64>,
    pub track: Option<Track>,
    pub station: Option<Station>,
}

#[derive(Serialize, Deserialize)]
pub struct Playlist {
    pub kind: String,
    pub name: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub shareToken: String,
    pub ownerProfilePhotoUrl: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResponse {
    pub kind: String,
    pub entries: Vec<Entries>,
    pub clusterOrder: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Seed {
    pub kind: String,
    pub trackId: String,
    pub seedType: String,
}

#[derive(Serialize, Deserialize)]
pub struct Series {
    pub seriesId: String,
    pub title: String,
    pub author: String,
    pub description: String,
    pub art: Vec<ArtistArtRefs>,
    pub copyright: String,
    pub explicitType: String,
    pub link: String,
    pub continuationToken: String,
    pub totalNumEpisodes: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Station {
    pub kind: String,
    pub name: String,
    pub seed: Seed,
    pub stationSeeds: Vec<Seed>,
    pub imageUrls: Vec<ArtistArtRefs>,
    pub compositeArtRefs: Vec<CompositeArtRefs>,
    pub skipEventHistory: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Thumbnails {
    pub url: String,
    pub width: i64,
    pub height: i64,
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
    pub albumArtRef: Vec<ArtistArtRefs>,
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

#[derive(Serialize, Deserialize)]
pub struct YoutubeVideo {
    pub kind: String,
    pub id: String,
    pub title: String,
    pub thumbnails: Vec<Thumbnails>,
}
