use app_dirs::{AppInfo, app_root, AppDataType};
use sj_ui::start;
use password::{retrieve_credentials, Operation};
use std::path::PathBuf;

fn get_save_location() -> PathBuf {
    let app_info = AppInfo {
        name: "playbar-cli",
        author: "playbar",
    };
    // use app root
    PathBuf::from(app_root(AppDataType::UserConfig, &app_info).expect("Trouble retrieving app directory, perhaps try using the portable version of playbar"))
}

fn main() {
    let save_location = get_save_location().with_file_name(".pianobar");
    start(retrieve_credentials(&save_location, Operation::ResetAll))
}
