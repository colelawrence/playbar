use super::sj;
use crate::prelude::*;
use rodio;

pub enum Play {
    Radio(sj::RadioStationId),
}

pub enum Action {
    Play(Play),
    Stop,
    Pause,
    Skip,
}

pub struct Player {
    api: Addr<sj::SJApi>,
    device: rodio::Device,
}

impl Player {
    pub fn new(api: Addr<sj::SJApi>) -> Self {
        let device =
            rodio::default_output_device().expect("could not choose a default audio output device");
        // Google Play Music 'hi' -> 320kbps = Samples 44100 Hz -> 32 bits per sample
        // TODO: adjust mixer defaults
        // device.supported_input_formats()
        // rodio::dynamic_mixer::mixer(2, sample_rate: u32)
        Player { api, device }
    }
}

impl Actor for Player {
    type Context = Context<Self>;
}

impl Message for Action {
    type Result = Result<()>;
}

impl Handler<Action> for Player {
    type Result = CommandFuture<()>;

    fn handle(&mut self, msg: Action, _ctx: &mut Context<Self>) -> Self::Result {
        // match msg {
        //     Action::Pause => {

        //     }
        // }
        Box::new(future::err(Error::new("unimplemented")))
    }
}
