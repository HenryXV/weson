use crate::audio::output;
use log::{error, info};
use std::fs::File;
use std::io::stdin;
use std::ops::Deref;
use std::path::PathBuf;
use symphonia::core::codecs::{Decoder, FinalizeResult};
use symphonia::core::errors::{Error, Result};
use symphonia::core::formats::{FormatOptions, FormatReader};
use symphonia::core::io::{MediaSource, MediaSourceStream, ReadOnlySource};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

pub struct PacketReader {
    reader: Box<dyn FormatReader>,
    track_id: u32,
}

impl PacketReader {
    fn new(reader: Box<dyn FormatReader>, track_id: u32) -> Self {
        Self { reader, track_id }
    }

    pub fn build(path: &PathBuf) -> Result<PacketReader> {
        let mut hint = Hint::new();

        let source = if let Some(extension) = path.extension() {
            if let Some(extension_str) = extension.to_str() {
                hint.with_extension(extension_str);
            }

            Box::new(File::open(path)?)
        } else {
            Box::new(ReadOnlySource::new(stdin())) as Box<dyn MediaSource>
        };

        let mss = MediaSourceStream::new(source, Default::default());

        let format_opts = FormatOptions {
            enable_gapless: false,
            ..Default::default()
        };

        let metadata_opts: MetadataOptions = Default::default();

        let probe =
            symphonia::default::get_probe().format(&hint, mss, &format_opts, &metadata_opts)?;

        let track_id = probe.format.default_track().unwrap().id;

        Ok(PacketReader::new(probe.format, track_id))
    }

    pub fn reader(&self) -> &dyn FormatReader {
        self.reader.deref()
    }

    pub fn track_id(&self) -> u32 {
        self.track_id
    }
}

pub struct PacketDecoder {
    packet_reader: PacketReader,
    decoder: Box<dyn Decoder>,
}

impl PacketDecoder {
    fn new(packet_reader: PacketReader, decoder: Box<dyn Decoder>) -> Self {
        Self {
            packet_reader,
            decoder,
        }
    }

    pub fn build(path: &PathBuf) -> Result<PacketDecoder> {
        let packet_reader = PacketReader::build(path)?;

        let track = packet_reader.reader.default_track().unwrap();

        let decoder =
            symphonia::default::get_codecs().make(&track.codec_params, &Default::default())?;

        Ok(PacketDecoder::new(packet_reader, decoder))
    }

    pub fn decode(
        &mut self,
        audio_output: &mut Option<Box<dyn output::AudioOutput>>,
    ) -> Result<i32> {
        let result = loop {
            let packet = match self.packet_reader.reader.next_packet() {
                Ok(packet) => packet,
                Err(err) => break Err(err),
            };

            if packet.track_id() != self.packet_reader.track_id {
                continue;
            }

            match self.decoder.decode(&packet) {
                Ok(decoded) => {
                    if audio_output.is_none() {
                        let spec = decoded.spec();

                        let duration = decoded.capacity() as u64;

                        println!("{:?} {}", spec.clone(), duration);
                        audio_output.replace(
                            output::cpal::CpalAudioOutput::try_open(*spec, duration).unwrap(),
                        );
                    }

                    if packet.ts() >= 0.0 as u64 {
                        if let Some(audio_output) = audio_output {
                            audio_output.write(decoded).unwrap();
                        }
                    }
                }
                Err(Error::DecodeError(err)) => eprintln!("decode error: {}", err),
                Err(err) => break Err(err),
            }
        };

        PacketDecoder::ignore_end_of_stream_error(result)?;

        PacketDecoder::do_verification(self.decoder.finalize())
    }

    fn ignore_end_of_stream_error(result: Result<()>) -> Result<()> {
        match result {
            Err(Error::IoError(err))
                if err.kind() == std::io::ErrorKind::UnexpectedEof
                    && err.to_string() == "end of stream" =>
            {
                Ok(())
            }
            _ => result,
        }
    }

    fn do_verification(finalization: FinalizeResult) -> Result<i32> {
        match finalization.verify_ok {
            Some(is_ok) => {
                println!("verification: {}", if is_ok { "passed" } else { "failed" });

                Ok(i32::from(!is_ok))
            }

            _ => Ok(0),
        }
    }
    pub fn packet_reader(&self) -> &PacketReader {
        &self.packet_reader
    }

    pub fn decoder(&self) -> &dyn Decoder {
        self.decoder.deref()
    }
}
