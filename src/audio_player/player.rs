use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub struct Player {
    _stream: OutputStream,
    sink: Sink,
}

impl Player {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        Self { _stream, sink }
    }

    pub fn add(&self, path: &PathBuf) {
        if let Ok(f) = File::open(path) {
            let file = BufReader::new(f);
            let source = Decoder::new(file).unwrap();

            self.sink.append(source);
        }
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn resume(&self) {
        self.sink.play();
    }
}
