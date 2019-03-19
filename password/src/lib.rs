use sj_token::generate_device_id;

mod google_login;
mod input;
mod password;
mod save;

use input::input;
use save::SaveState;

pub use sj_token::SJToken;

/// Ask for credentials via commandline either by reading the credentials from the `save_file_path`
/// or by requesting the user's email and password so we can fetch their Google Play music auth token
pub fn retrieve_credentials(save_file_path: &str) -> SJToken {
    let result = save::read_save_file(save_file_path);

    match result {
        SaveState::Found(sj_token) => {
            eprintln!("Successfully loaded credentials file");
            sj_token
        }
        _ => {
            eprint!("Google Play email: ");
            let email = input();
            let enc_password = password::input_password_and_encrypt(&email);
            let android_id = generate_device_id();

            let auth = google_login::google_login(
                google_login::account_type::EITHER,
                &email,
                &enc_password,
                &android_id,
            )
            .expect("successful login");

            save::save_file(save_file_path, &auth);

            println!("Successfully created new credentials file");

            auth
        }
    }
}
