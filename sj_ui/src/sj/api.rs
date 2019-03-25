use super::prelude::*;
use super::query;
use crate::SJAccess;
use actix::*;

pub struct SJApi {
    token: SJAccess,
}

impl SJApi {
    pub fn new(access: SJAccess) -> Self {
        SJApi { token: access }
    }
}

impl Actor for SJApi {
    type Context = Context<Self>;
}

pub struct SearchRadioStations {
    pub query: String,
}

pub struct RadioStation {
    pub id: RadioStationId,
    pub name: String,
    pub description: Option<String>,
    pub byline: Option<String>,
}

pub struct RadioStationId {
    inner: RadioSeed,
}

pub struct AlbumId {
    inner: String,
}

enum RadioSeed {
    // 9
    CuratedStationId(String),
    // 3
    ArtistId(String),
    // 2
    TrackId(String),

    // VideoId(String),
    // SituationId(String),
    // PlaylistId(String),
    // GenreId(String),
    // AlbumId(String),
}

impl From<query::RadioSeed> for RadioStationId {
    fn from(seed: query::RadioSeed) -> Self {
        let inner = if let Some(artist_id) = seed.artistId {
            RadioSeed::ArtistId(artist_id.clone())
        } else if let Some(station_id) = seed.curatedStationId  {
            RadioSeed::CuratedStationId(station_id.clone())
        } else if let Some(track_id) = seed.trackId  {
            RadioSeed::TrackId(track_id.clone())
        } else {
            unimplemented!("Unknown radio seed kind: {}", seed.seedType)
        };
        RadioStationId {
            inner: inner,
        }
    }
}

impl Message for SearchRadioStations {
    type Result = Result<Vec<RadioStation>, actix_web::Error>;
}

impl Handler<SearchRadioStations> for SJApi {
    type Result = FutureResponse<Vec<RadioStation>>;

    fn handle(&mut self, msg: SearchRadioStations, _ctx: &mut Context<Self>) -> Self::Result {
        Box::new(
            query::query(
                &self.token,
                query::SearchParams {
                    categories: &[6],
                    max_results: 10,
                    query: &msg.query,
                },
            )
            .map(|result: query::SearchResponse| {
                result
                    .clusterDetail
                    .into_iter()
                    .filter_map(|a: query::ClusterDetail| {
                        if a.cluster._type == "6" {
                            a.entries
                        } else {
                            None
                        }
                    })
                    .flatten()
                    .filter_map(|a: query::ClusterEntry| match a {
                        query::ClusterEntry::RadioStation(station) => Some(station.station),
                        _ => None,
                    })
                    .map(|qs: query::RadioStation| RadioStation {
                        id: qs.seed.into(),
                        name: qs.name,
                        description: qs.description,
                        byline: qs.byline,
                    })
                    .collect::<Vec<RadioStation>>()
            }),
        )
    }
}
