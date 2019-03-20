use sj_ui::start;
use password::{retrieve_credentials, Operation};
use std::path::PathBuf;

fn get_save_location() -> PathBuf {
    let mut path_buf = std::env::current_exe().expect("unable to get path of current executable");
    path_buf.pop();
    path_buf
}

fn main() {
    let save_location = get_save_location().with_file_name(".pianobar");
    // TODO: Check if credentials are still valid, if not reset
    start(retrieve_credentials(&save_location, Operation::Silent))
}
