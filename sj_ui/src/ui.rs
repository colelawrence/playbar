use crate::prelude::*;
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

use super::player;
use super::sj;

pub struct UI {
    api: Addr<sj::SJApi>,
    player: Addr<player::Player>,
}

impl UI {
    pub fn new(api: Addr<sj::SJApi>, player: Addr<player::Player>) -> Self {
        UI { api, player }
    }
}

impl Actor for UI {
    type Context = Context<Self>;
}

struct SearchRadioStations;

impl Message for SearchRadioStations {
    type Result = Result<Option<player::Action>>;
}

impl Handler<SearchRadioStations> for UI {
    type Result = CommandFuture<Option<player::Action>>;
    fn handle(&mut self, _msg: SearchRadioStations, _ctx: &mut Context<Self>) -> Self::Result {
        print!("Search stations: ");
        let query = input();
        println!("Searching stations...");

        Box::new(
            self.api
                .send(sj::SearchRadioStations { query: query })
                .from_err()
                .flatten()
                .from_err()
                .and_then(|stations: Vec<sj::RadioStation>| {
                    println!("Result-----");
                    for station in stations {
                        match station.description {
                            Some(desc) => println!("{} - {}", station.name, desc),
                            None => println!("{}", station.name),
                        }
                    }
                    Ok(None)
                }),
        )
    }
}
