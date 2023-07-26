use crate::queue::AudioSource;
use rodio::{OutputStream, Sink};

pub struct Player {
    _stream: OutputStream,
    sink: Sink,
}

impl Default for Player {
    fn default() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        Self { _stream, sink }
    }
}

impl Player {
    pub fn play(&mut self, source: AudioSource) {
        self.sink.append(source);
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn resume(&self) {
        self.sink.play();
    }

    pub fn queue_len(&self) -> usize {
        self.sink.len()
    }
}
