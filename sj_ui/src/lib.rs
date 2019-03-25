#[macro_use]
extern crate serde_derive;

mod sj;
mod ui;
mod player;
pub mod prelude;

use prelude::*;


use std::iter::Iterator;

const s1: Vec<u8> = base64::decode("VzeC4H4h+T2f0VI180nVX8x+Mb5HiTtGnKgH52Otj8ZCGDz9jRWyHb6QXK0JskSiOgzQfwTY5xgLLSdUSreaLVMsVVWfxfa8Rw==").unwrap();
const s2: Vec<u8> = base64::decode("ZAPnhUkYwQ6y5DdQxWThbvhJHN8msQ1rqJw0ggKdufQjelrKuiGGJI30aswkgCWTDyHkTGK9ynlqTkJ5L4CiGGUabGeo8M6JTQ==").unwrap();
// const STREAM_SIGNING_KEY: &[u8] = s1.iter().;


#[allow(unconditional_recursion)]
pub fn start(token: SJAccess) {
    let sys = System::new("sj_ui");

    let sj_api = sj::SJApi::new(token).start();
    let player = player::Player::new(sj_api).start();

    ui::UI::new(sj_api, player).start();

    sys.run();
}
