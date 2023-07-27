use audio::decoder::WesonDecoder;
use audio::queue::AudioSource;
use std::fs::{DirEntry, Metadata};
use std::path::PathBuf;

#[derive(Clone, Debug, Default)]
pub struct AudioEntry {
    path: PathBuf,
    name: String,
    metadata: Option<Metadata>,
}

impl AudioEntry {
    fn from(dir_entry: DirEntry) -> Option<Self> {
        Option::from(Self {
            path: dir_entry.path().to_path_buf(),
            name: dir_entry.file_name().to_str().unwrap().to_string(),
            metadata: Some(dir_entry.metadata().unwrap()),
        })
    }

    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn metadata(&self) -> &Option<Metadata> {
        &self.metadata
    }
}

impl AudioEntry {
    pub fn get_audio_source(&self) -> anyhow::Result<AudioSource> {
        WesonDecoder::decode(&self.path)
    }
}
