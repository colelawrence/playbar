use sj_token::{generate_device_id, read_save_file, save_file, SJToken, SJAccess, SaveState};
use std::path::Path;

mod google_login;
mod input;
mod password;

use input::input;

pub enum Operation {
    ResetAll,
    Login,
    Silent,
}

/// Ask for credentials via commandline either by reading the credentials from the `save_file_path`
/// or by requesting the user's email and password so we can fetch their Google Play music auth token
pub fn retrieve_credentials(save_file_path: &Path, operation: Operation) -> SJAccess {
    let result = read_save_file(save_file_path);

    // refresh the token before returning
    match result {
        SaveState::Found(sj_token) => {
            let sj_token = match operation {
                Operation::Login => {
                    eprintln!("Successfully loaded credentials file");
                    eprintln!("Updating login credentials");
                    login(Some(sj_token.email), Some(sj_token.device_id))
                }
                Operation::ResetAll => login(None, None),
                Operation::Silent => {
                    eprintln!("Successfully loaded credentials file");
                    // google_login::google_refresh(sj_token)
                    sj_token
                }
            };

            save_file(save_file_path, &sj_token);

            sj_token
        }
        _ => {
            println!("Save file not found at \"{:?}\"", save_file_path);
            let sj_token = login(None, None);
            save_file(save_file_path, &sj_token);

            println!("Successfully created new credentials file");

            sj_token
        }
    }.access_only()
}

fn login(email: Option<String>, device_id: Option<String>) -> SJToken {
    let email = match email {
        None => {
            eprint!("Google Play email: ");
            input()
        }
        Some(email) => {
            eprintln!("Google Play email: {}", email);
            email
        }
    };

    let enc_password = password::input_password_and_encrypt(&email);
    let android_id = device_id.unwrap_or_else(|| generate_device_id());

    google_login::google_login(
        google_login::account_type::EITHER,
        &email,
        &enc_password,
        &android_id,
    )
}
