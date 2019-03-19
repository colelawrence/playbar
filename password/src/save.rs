use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind;

use sj_token::SJToken;

pub enum SaveState {
    NoneFound,
    Corrupt,
    Found(SJToken),
}

pub fn read_save_file(config_file_path: &str) -> SaveState {
    match File::open(config_file_path) {
        Err(err) => match err.kind() {
            ErrorKind::NotFound => SaveState::NoneFound,
            _ => panic!("Error opening \"{}\": {:?}", config_file_path, err),
        },
        Ok(file) => {
            let contents =
                String::from_utf8(file.bytes().map(Result::unwrap).collect()).expect(&format!(
                    "Failed to read \"{}\", might need to be reset",
                    config_file_path
                ));
            match SJToken::from_save(contents) {
                Ok(save_state) => SaveState::Found(save_state),
                Err(err) => {
                    eprintln!("Error reading save file: {}", err);
                    SaveState::Corrupt
                }
            }
        }
    }
}

pub fn save_file(config_file_path: &str, auth: &SJToken) {
    let save_file_file: Option<File> = match File::create(config_file_path) {
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create(config_file_path) {
                Ok(file) => Some(file),
                Err(err) => {
                    eprintln!(
                        "Unable to create credentials at \"{}\": {:?}",
                        config_file_path, err
                    );
                    None
                }
            },
            _ => {
                eprintln!(
                    "Unable to save credentials to \"{}\": {:?}",
                    config_file_path, err
                );
                None
            }
        },
        Ok(file) => Some(file),
    };

    save_file_file
        .map(|mut file| write!(file, "{}", auth.to_save().unwrap()).expect("able to write"));
}
