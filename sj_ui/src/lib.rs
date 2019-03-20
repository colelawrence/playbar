#[macro_use]
extern crate serde_derive;

use cpal::Format;
use rodio::{self, Source};
use std::thread;
use std::time::Duration;
use futures::future::Future;

use serde_json;

mod ui;
mod wave;
mod sj;

use actix_web::actix;

use wave::{WaveConfig, WaveKind, WaveSource};

pub use sj_token::SJAccess;

#[allow(unconditional_recursion)]
pub fn start(token: SJAccess) {
    let mut sys = actix::System::new("test");

    // start_ui(token, &sys);
    // use std::fs::File;
    // use std::io::BufReader;

    // let device = rodio::devices().find(|d| d.name() == "pulse").expect("pulse device exists");

    // let file = File::open("test/ogg.ogg").unwrap();
    // let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    // let sink = Sink::new(&device);

    // // sink.append(source);
    // // let source = rodio::source::SineWave::new(440).amplify(0.1);
    // sink.append(source);

    // thread::sleep(Duration::from_millis(4500));
    // return;

    let res = sj::query::query(&token, sj::query::SearchParams {
        categories: &[6],
        max_results: 1,
        query: "elephant gun"
    }).inspect(|result| {
        println!("{:?}", serde_json::to_string_pretty(&result))
    });

    match sys.block_on(res) {
        Ok(value) => (),
        Err(err) => eprintln!("Error during search {:?}", err),
    }


    println!("Let's hear a wave!");
    print!("Please input a Hz for a sine wave: ");
    let value: usize = loop {
        let test_value = ui::input();
        match usize::from_str_radix(&test_value, 10u32) {
            Ok(hz) => break hz,
            Err(parse_err) => {
                println!("Couldn't parse as a positive integer, {}", parse_err);
                print!("Please try again: ");
            }
        }
    };

    println!("Okay. {}Hz? Let's go!", value);

    let wave = WaveConfig {
        hz: value,
        phase_hz: 0,
        kind: WaveKind::Sine,
    };
    let device = rodio::default_output_device().expect("to find default output device");

    let format: Format = device
        .default_output_format()
        .expect("device still connected");

    let player = WaveSource::new(wave, Duration::from_secs(2), format.sample_rate.0)
        .amplify(0.2)
        .buffered();

    println!("Playing through {:?}", device.name());

    rodio::play_raw(&device, player);

    thread::sleep(Duration::from_millis(1500));
    // start_ui(token, sys);

    sys.run();
}

// fn start_ui(token: SJAccess, sys: &actix::system::SystemRunner) {
// }
