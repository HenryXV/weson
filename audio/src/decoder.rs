use crate::queue::AudioSource;
use rodio::Decoder;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub struct WesonDecoder {}

impl WesonDecoder {
    pub fn decode(path: &PathBuf) -> anyhow::Result<AudioSource> {
        let file = BufReader::new(File::open(path)?);
        let source = Decoder::new(file)?;

        Ok(source)
    }
}
