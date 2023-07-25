use crate::audio::decoder::PacketDecoder;
use std::fs::File;
use std::path::PathBuf;

pub struct Player {}

impl Player {
    pub fn new() -> Self {
        Self {}
    }

    pub fn play(path: &PathBuf) {
        if let Ok(f) = File::open(path) {
            let mut decoder = PacketDecoder::build(path).unwrap();
            decoder.decode(&mut None);
        }
    }
}
