use super::password::EncryptedPw;
use http_req::{request, uri::Uri};
const GOOGLE_AUTH_ENDPOINT: &str = "https://android.clients.google.com/auth";

use sj_token::SJToken;

pub mod account_type {
    // pub const HOSTED: &str = "HOSTED";
    // pub const GOOGLE: &str = "GOOGLE";
    pub const EITHER: &str = "HOSTED_OR_GOOGLE";
}

pub fn google_login(
    account_type: &str,
    email: &str,
    pw: &EncryptedPw,
    android_id: &str,
) -> Option<SJToken> {
    let auth_uri: Uri = GOOGLE_AUTH_ENDPOINT.parse().unwrap();

    let params: Vec<String> = [
        ("accountType", account_type),
        ("Email", email),
        ("has_permission", "1"),
        ("add_account", "1"),
        ("EncryptedPasswd", &pw.to_url_safe_base64()),
        ("service", "sj"), // only need "skyjam" service
        ("source", "android"),
        ("androidId", android_id),
        ("device_country", "us"),
        ("operatorCountry", "us"),
        ("lang", "en"),
        ("sdk_version", "17"),
    ]
    .into_iter()
    .map(|(k, v)| format!("{}={}", *k, percent_encoded(*v)))
    .collect();

    let body = params.join("&");

    let req = request::Request::new(&auth_uri)
        .method(request::Method::POST)
        .header(
            "Content-Type",
            "application/x-www-form-urlencoded; charset=utf-8",
        )
        .header("Content-Length", &format!("{}", body.len()))
        .body(body.as_bytes())
        .clone();

    let mut response = Vec::new();

    req.send(&mut response).expect("successful call");

    let resp_str = String::from_utf8_lossy(response.as_slice());

    let auth = resp_str
        .split_whitespace()
        .filter(|line| line.len() > 5)
        .find_map(|line| match line.split_at(5) {
            ("Auth=", value) => Some(value),
            _ => None,
        });

    let auth = match auth {
        None => panic!("Failed to login: {}\nPerhaps you need to log in using an App password, especially if you have Two Factor authentication turned on", resp_str),
        Some(auth) => auth,
    };

    Some(SJToken::new(auth, android_id))
}

fn percent_encoded(p: &str) -> String {
    use percent_encoding::{utf8_percent_encode, USERINFO_ENCODE_SET};
    utf8_percent_encode(p, USERINFO_ENCODE_SET).to_string()
}
