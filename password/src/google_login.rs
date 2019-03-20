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
) -> SJToken {
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

    let mut auth = None;
    let mut token = None;
    resp_str
        .split_whitespace()
        .filter(|line| line.len() > 5)
        .for_each(|line| {
            if let ("Auth=", value) = line.split_at(5) {
                auth = Some(value);
            }
            if let ("Token=", value) = line.split_at(6) {
                token = Some(value);
            }
        });

    if auth.is_none() || token.is_none() {
        eprintln!("Failed to login: {}\nPerhaps you need to log in using an App password, especially if you have Two Factor authentication turned on", resp_str);
        std::process::exit(1);
    }

    SJToken::new(token.unwrap(), auth.unwrap(), email, android_id)
}

pub fn google_refresh(
    token: SJToken,
) -> SJToken {
    let auth_uri: Uri = GOOGLE_AUTH_ENDPOINT.parse().unwrap();

    let params: Vec<String> = [
        ("accountType", "GOOGLE_OR_HOSTED"),
        ("Token", &token.mt),
        ("has_permission", "1"),
        ("service", "sj"), // only need "skyjam" service
        ("source", "android"),
        ("androidId", &token.device_id),
        ("device_country", "us"),
        ("operatorCountry", "us"),
        ("lang", "en"),
        ("sdk_version", "17"),
        ("app", "com.google.android.music"),
        ("client_sig", "38918a453d07199354f8b19af05ec6562ced5788"),
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

    let mut auth = None;
    resp_str
        .split_whitespace()
        .filter(|line| line.len() > 5)
        .for_each(|line| {
            if let ("Auth=", value) = line.split_at(5) {
                auth = Some(value);
            }
        });

    if auth.is_none() {
        eprintln!("Failed to refresh: {}", resp_str);
        std::process::exit(1);
    }

    SJToken::new(&token.mt, auth.unwrap(), &token.email, &token.device_id)
}

fn percent_encoded(p: &str) -> String {
    use percent_encoding::{utf8_percent_encode, USERINFO_ENCODE_SET};
    utf8_percent_encode(p, USERINFO_ENCODE_SET).to_string()
}
