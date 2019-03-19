use http_req::request;
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

mod password;

fn main() {
    // eprint!("Google Play username: ");
    // let username = input();
    let enc_password = password::input_password_and_encrypt("cole@reaktor.com");

    println!("{}", enc_password.to_url_safe_base64());
}
