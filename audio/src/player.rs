use rodio::{Decoder, OutputStream, Sink};
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::Mutex;
use tokio::sync::Notify;

pub type AudioSource = Decoder<BufReader<File>>;

pub struct Player {
    _stream: OutputStream,
    sink: Sink,
    queue: Mutex<VecDeque<AudioSource>>,
    notify_on_empty: Notify,
}

impl Default for Player {
    fn default() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        Self {
            _stream,
            sink,
            queue: Mutex::new(Default::default()),
            notify_on_empty: Notify::new(),
        }
    }
}

impl Player {
    pub fn add_to_queue(&mut self, path: &PathBuf) {
        if let Ok(f) = File::open(path) {
            let file = BufReader::new(f);
            let source = Decoder::new(file).unwrap();

            log::debug!(
                "adding music {} to queue",
                path.file_stem().unwrap().to_str().unwrap().to_string()
            );

            self.queue.lock().unwrap().push_back(source);
        }
    }

    async fn recv(&mut self) {}

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn resume(&self) {
        self.sink.play();
    }

    pub fn queue(&self) -> &Mutex<VecDeque<AudioSource>> {
        &self.queue
    }
}
