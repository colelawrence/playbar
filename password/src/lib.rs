use std::path::Path;
use sj_token::{generate_device_id, SJToken, read_save_file, save_file, SaveState};

mod google_login;
mod input;
mod password;

use input::input;

/// Ask for credentials via commandline either by reading the credentials from the `save_file_path`
/// or by requesting the user's email and password so we can fetch their Google Play music auth token
pub fn retrieve_credentials(save_file_path: &Path) -> SJToken {
    let result = read_save_file(save_file_path);

    match result {
        SaveState::Found(sj_token) => {
            eprintln!("Successfully loaded credentials file");
            sj_token
        }
        _ => {
            println!("Save file not found at \"{:?}\"", save_file_path);
            eprint!("Google Play email: ");
            let email = input();
            let enc_password = password::input_password_and_encrypt(&email);
            let android_id = generate_device_id();

            let sj_token = google_login::google_login(
                google_login::account_type::EITHER,
                &email,
                &enc_password,
                &android_id,
            )
            .expect("successful login");

            save_file(save_file_path, &sj_token);

            println!("Successfully created new credentials file");

            sj_token
        }
    }
}
