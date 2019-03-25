pub use crate::prelude::*;
pub use actix_web::{client, FutureResponse, HttpMessage};
pub use futures::future::{self, Either, Future};
use serde::de::DeserializeOwned;

pub fn query_string<'a, T: std::iter::IntoIterator<Item = &'a (&'a str, &'a str)>>(
    params: T,
) -> String {
    params
        .into_iter()
        .filter_map(|(k, v)| {
            if v.len() > 0 {
                Some(format!("{}={}", k, percent_encoded(v)))
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .join("&")
}

pub fn sj_req<T: DeserializeOwned + 'static>(
    access: &SJAccess,
    req: client::ClientRequestBuilder,
) -> impl Future<Item = T, Error = actix_web::Error> {
    sj_http(access, req).and_then(|res: client::ClientResponse| {
        res.json::<serde_json::Value>()
            .from_err()
            .and_then(move |json_value| {
                if res.status().is_success() {
                    Either::A(future::result(serde_json::from_value::<T>(json_value)).from_err())
                } else {
                    Either::B(future::err(actix_web::error::ErrorBadRequest(
                        "Failed SJ request",
                    )))
                }
            })
    })
}

pub fn sj_http(
    access: &SJAccess,
    mut req: client::ClientRequestBuilder,
) -> impl Future<Item = client::ClientResponse, Error = actix_web::Error> {
    future::result(
        req.header("Authorization", access.authorization_value())
            .header("X-Device-ID", access.x_device_id_value())
            .timeout(std::time::Duration::from_secs(10))
            // Errors when trying to accept gzip https://github.com/actix/actix-web/issues/674
            .header("Accept-Encoding", "br, deflate")
            .finish(),
    )
    .and_then(|req: client::ClientRequest| {
        req.send()
            .map_err(|err| {
                eprintln!("Sending error: {:?}", err);
                err
            })
            .from_err()
    })
}

fn percent_encoded(p: &str) -> String {
    use percent_encoding::{utf8_percent_encode, USERINFO_ENCODE_SET};
    utf8_percent_encode(p, USERINFO_ENCODE_SET).to_string()
}
