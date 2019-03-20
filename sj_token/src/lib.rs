//! `SJToken` encode/save and read/decode operations.
//!
//! SJ stands for "SkyJam" which is the internal service name for querying the Google Play Music service.
//! As such, your SJ Token only has access to your SJ service, and does not have any access to your other Google services.
use base64;
use openssl::sha;
use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;

const HASH_LEN: usize = 12;
const DEVICE_ID_LEN: usize = 16;

pub struct SJToken {
    auth: String,
    device_id: String,
}

impl SJToken {
    /// "Authorization"
    pub fn authorization_value(&self) -> String {
        format!("GoogleLogin auth={}", self.auth)
    }
    /// "X-Device-ID"
    pub fn x_device_id_value(&self) -> String {
        self.device_id.clone()
    }

    pub fn new(auth: &str, device_id: &str) -> Self {
        SJToken {
            auth: auth.to_string(),
            device_id: device_id.to_string(),
        }
    }

    /// From file contents
    pub fn from_string(contents: String) -> Result<Self, &'static str> {
        let (version, tail) = contents.split_at(1);
        match version {
            "0" => {
                let (hash, tail) = tail.split_at(HASH_LEN);
                let (device_id, auth) = tail.split_at(DEVICE_ID_LEN);
                let data_hash = SJToken::hash(device_id, auth);
                if hash == data_hash {
                    Ok(SJToken {
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
    pub fn to_string(&self) -> Result<String, &'static str> {
        // version 0
        if self.device_id.len() != DEVICE_ID_LEN {
            return Err("device id is invalid length");
        }
        let hash = SJToken::hash(&self.device_id, &self.auth);
        Ok(format!("0{}{}{}", hash, self.device_id, self.auth))
    }

    fn hash(device_id: &str, token: &str) -> String {
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

pub enum SaveState {
    NoneFound,
    Corrupt,
    Found(SJToken),
}

pub fn read_save_file(config_file_path: &Path) -> SaveState {
    match File::open(config_file_path) {
        Err(err) => match err.kind() {
            ErrorKind::NotFound => SaveState::NoneFound,
            _ => panic!("Error opening \"{:?}\": {:?}", config_file_path, err),
        },
        Ok(file) => {
            let contents =
                String::from_utf8(file.bytes().map(Result::unwrap).collect()).expect(&format!(
                    "Failed to read \"{:?}\", might need to be reset",
                    config_file_path
                ));
            match SJToken::from_string(contents) {
                Ok(save_state) => SaveState::Found(save_state),
                Err(err) => {
                    eprintln!("Error reading save file: {}", err);
                    SaveState::Corrupt
                }
            }
        }
    }
}

pub fn save_file(config_file_path: &Path, auth: &SJToken) {
    let save_file_file: Option<File> = match File::create(config_file_path) {
        Err(err) => {
            eprintln!(
                "Unable to save credentials to \"{:?}\": {:?}",
                config_file_path, err
            );
            None
        }
        Ok(file) => Some(file),
    };

    save_file_file
        .map(|mut file| write!(file, "{}", auth.to_string().unwrap()).expect("able to write"));
}
