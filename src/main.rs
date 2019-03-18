use cpal::Format;
use rodio::{self, Sample, Sink, Source};
use std::thread;
use std::time::Duration;

mod input;
mod wave;

use input::input;
use wave::{WaveConfig, WaveKind, WaveSource};

fn main() {
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
    println!("Let's hear a wave!");
    print!("Please input a Hz for a sine wave: ");
    let value: usize = loop {
        let test_value = input();
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
        .default_input_format()
        .expect("device still connected");

    let player = WaveSource::new(wave, Duration::from_secs(2), format.sample_rate.0)
        .amplify(0.2)
        .buffered();

    println!("Playing through {:?}", device.name());

    rodio::play_raw(&device, player);

    thread::sleep(Duration::from_millis(1500));
    main()
}
