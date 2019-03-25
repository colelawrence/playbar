#![allow(non_snake_case)]

use super::prelude::*;

pub enum StreamParams<'a> {
    SongId(&'a str)
}

pub fn stream(t: &SJAccess, params: StreamParams) -> FutureResponse<StreamResponse> {

    Box::new(sj_http(
        t,
        client::get(format!(
            "https://mclients.googleapis.com/music/mplay?{}",
            query_string(&[
                ("max-results", format!("{}", params.max_results).as_str()),
                ("q", params.query),
                ("ic", "true"), // in clusters
                ("ct", &cts),
                ("dv", "17"), // device version
                ("hl", "en"), // language
                ("tier", "aa"), // subscribed or not
            ])
        )),
    ).and_then(|res: client::ClientResponse| {
        match res.get_header::<actix_web::http::header::LOCATION>() {
            Some(location) => Ok(StreamResponse { url: location.into() }),
            None => Err(actix_web::error::ErrorInternalServerError("Unknown"))
        }
    }))
}

#[derive(Serialize, Deserialize)]
pub struct StreamResponse {
    pub url: String,
}


fn sign_id(id: String) -> (String, String) {
    use crypto::hmac::Hmac;
    use crypto::sha1::Sha1;
    use crypto::mac::Mac;
    let salt = format!("{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() * 1000);
    let mut ctx = Hmac::new(Sha1::new(), STREAM_SIGNING_KEY);
    ctx.input(id.as_bytes());
    ctx.input(salt.as_bytes());
    let sig = base64::encode(ctx.result().code());
    (salt, sig)
}
