use std::io::{stdin, stdout, Write};

pub fn input() -> String {
    let mut s = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter something I could understand...");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

mod google;
mod password;
mod save;

use save::SaveState;

fn main() {
    let save_file_path = ".playbar";
    let result = save::read_save_file(save_file_path);

    match result {
        SaveState::Found(google_auth) => {
            eprintln!("Successfully loaded credentials file");
            google_auth
        }
        _ => {
            eprint!("Google Play email: ");
            let email = input();
            let enc_password = password::input_password_and_encrypt(&email);
            let android_id = google::generate_device_id();

            let auth = google::google_login(
                google::AccountType::EITHER,
                &email,
                &enc_password,
                &android_id,
            )
            .expect("successful login");

            save::save_file(save_file_path, &auth);

            println!("Successfully created new credentials file");

            auth
        }
    };
}
