extern crate password;

fn main() {
    let save_file_path = ".playbar";
    password::retrieve_credentials(save_file_path);
}
