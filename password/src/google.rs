use super::password::EncryptedPw;
use http_req::{request, uri::Uri};
const GOOGLE_AUTH_ENDPOINT: &str = "https://android.clients.google.com/auth";

pub struct GoogleAuth {
    auth: String,
    device_id: String,
}

pub mod AccountType {
    pub const HOSTED: &str = "HOSTED";
    pub const GOOGLE: &str = "GOOGLE";
    pub const EITHER: &str = "HOSTED_OR_GOOGLE";
}

pub fn google_login(
    account_type: &str,
    email: &str,
    pw: &EncryptedPw,
    android_id: &str,
) -> Option<GoogleAuth> {
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

    Some(GoogleAuth {
        auth: auth.to_string(),
        device_id: android_id.to_string(),
    })
}

fn percent_encoded(p: &str) -> String {
    use percent_encoding::{utf8_percent_encode, USERINFO_ENCODE_SET};
    utf8_percent_encode(p, USERINFO_ENCODE_SET).to_string()
}

const HASH_LEN: usize = 12;
const DEVICE_ID_LEN: usize = 16;

impl GoogleAuth {
    /// "Authorization"
    pub fn authorization_value(&self) -> String {
        format!("GoogleLogin auth={}", self.auth)
    }
    /// "X-Device-ID"
    pub fn x_device_id_value(&self) -> String {
        self.device_id.clone()
    }

    /// From file contents
    pub fn from_save(contents: String) -> Result<Self, &'static str> {
        let (version, tail) = contents.split_at(1);
        match version {
            "0" => {
                let (hash, tail) = tail.split_at(HASH_LEN);
                let (device_id, auth) = tail.split_at(DEVICE_ID_LEN);
                let data_hash = GoogleAuth::hash(device_id, auth);
                if hash == data_hash {
                    Ok(GoogleAuth {
                        device_id: device_id.to_string(),
                        auth: auth.to_string(),
                    })
                } else {
                    Err("corrupt data")
                }
            }
            _ => Err("unknown version"),
        }
    }

    /// To file contents
    pub fn to_save(&self) -> Result<String, &'static str> {
        // version 0
        if self.device_id.len() != DEVICE_ID_LEN {
            return Err("device id is invalid length");
        }
        let hash = GoogleAuth::hash(&self.device_id, &self.auth);
        Ok(format!("0{}{}{}", hash, self.device_id, self.auth))
    }

    fn hash(device_id: &str, token: &str) -> String {
        use openssl::sha;
        let mut context = sha::Sha256::new();
        context.update(device_id.as_bytes());
        context.update(token.as_bytes());
        let digest = context.finish();
        base64::encode(&digest).split_at(HASH_LEN).0.to_string()
    }
}

pub fn generate_device_id() -> String {
    let mut buf = vec![0; DEVICE_ID_LEN / 2];
    openssl::rand::rand_bytes(&mut buf).expect("OpenSSl generates secure random bytes");
    hex(&buf)
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::new();
    for &byte in bytes {
        write!(&mut s, "{:02x}", byte).expect("Unable to write");
    }
    s
}
