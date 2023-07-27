use crate::decoder::WesonDecoder;
use rodio::Decoder;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::Sender;

pub type AudioSource = Decoder<BufReader<File>>;
pub type AudioList = Arc<Mutex<VecDeque<String>>>;

pub struct Queue {
    sender: Sender<AudioSource>,
    audio_list: AudioList,
    current_audio: Option<String>,
}

impl Queue {
    pub fn new(sender: Sender<AudioSource>) -> Self {
        Self {
            sender,
            audio_list: Arc::new(Mutex::new(VecDeque::new())),
            current_audio: None,
        }
    }

    pub fn add_audio(&mut self, path: &PathBuf) {
        if let Ok(source) = WesonDecoder::decode(path) {
            let sender = self.sender.clone();
            let audio_list = self.audio_list.clone();
            let path = path.clone();

            tokio::spawn(async move {
                audio_list
                    .lock()
                    .unwrap()
                    .push_back(path.file_stem().unwrap().to_str().unwrap().to_string());
                sender.send(source).await.unwrap();
            });
        }
    }

    pub fn audio_list(&self) -> &AudioList {
        &self.audio_list
    }

    pub fn current_audio(&self) -> &Option<String> {
        &self.current_audio
    }

    pub fn set_current_audio(&mut self, audio: String) {
        self.current_audio = Some(audio);
    }
}
