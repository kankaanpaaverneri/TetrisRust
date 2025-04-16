use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::BufReader;

const FILEPATH: &str = "tetris_theme.mp3";

#[derive(Debug)]
pub enum AudioError {
    Decoder(rodio::decoder::DecoderError),
    Stream(rodio::StreamError),
    Play(rodio::PlayError),
    Io(std::io::Error),
}

impl fmt::Display for AudioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AudioError::Decoder(error) => write!(f, "Decoder error: {}", error),
            AudioError::Stream(error) => write!(f, "Stream error: {}", error),
            AudioError::Play(error) => write!(f, "Play error: {}", error),
            AudioError::Io(error) => write!(f, "Failed to open file: {}", error),
        }
    }
}

impl Error for AudioError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AudioError::Decoder(error) => Some(error),
            AudioError::Stream(error) => Some(error),
            AudioError::Play(error) => Some(error),
            AudioError::Io(error) => Some(error),
        }
    }
}

impl From<rodio::decoder::DecoderError> for AudioError {
    fn from(error: rodio::decoder::DecoderError) -> Self {
        AudioError::Decoder(error)
    }
}

impl From<rodio::StreamError> for AudioError {
    fn from(error: rodio::StreamError) -> Self {
        AudioError::Stream(error)
    }
}

impl From<rodio::PlayError> for AudioError {
    fn from(error: rodio::PlayError) -> Self {
        AudioError::Play(error)
    }
}

impl From<std::io::Error> for AudioError {
    fn from(error: std::io::Error) -> Self {
        AudioError::Io(error)
    }
}

#[allow(dead_code)]
pub struct Audio {
    stream: OutputStream,
    output_stream_handle: OutputStreamHandle,
    sink: Sink,
}

impl Audio {
    pub fn get_sink(&self) -> &Sink {
        &self.sink
    }
}

pub fn play_audio() -> Result<Audio, AudioError> {
    let (stream, output_stream_handle) = OutputStream::try_default()?;
    let buffer_reader = create_buffer_reader()?;
    let source = Decoder::new(buffer_reader)?;
    let sink = Sink::try_new(&output_stream_handle)?;
    sink.append(source.repeat_infinite());
    sink.play();
    Ok(Audio {
        stream,
        output_stream_handle,
        sink,
    })
}

fn create_buffer_reader() -> std::io::Result<BufReader<File>> {
    match File::open(FILEPATH) {
        Ok(file) => {
            return Ok(BufReader::new(file));
        }
        Err(error) => Err(error),
    }
}
