use rodio::Decoder;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use tokio::sync::mpsc::Sender;

pub type AudioSource = Decoder<BufReader<File>>;

pub struct Queue {
    sender: Sender<AudioSource>,
}

impl Queue {
    pub fn new(sender: Sender<AudioSource>) -> Self {
        Self { sender }
    }

    pub fn add_audio(&mut self, path: &PathBuf) {
        if let Ok(f) = File::open(path) {
            let file = BufReader::new(f);
            let source = Decoder::new(file).unwrap();

            log::debug!(
                "adding {} to queue",
                path.file_stem().unwrap().to_str().unwrap().to_string()
            );

            let sender = self.sender.clone();
            tokio::spawn(async move { sender.send(source).await.unwrap() });
        }
    }
}
