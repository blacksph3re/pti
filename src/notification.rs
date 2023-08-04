use std::fs::File;
use std::io::BufReader;

use notify_rust::{Notification, Timeout};
use rodio::{Decoder, OutputStream, Sink, OutputStreamHandle};

const ALARM_FILE: &str = "alarm.mp3";

#[allow(dead_code)]
pub struct NotificationManager {
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Sink,
}

impl NotificationManager {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Self {
            stream,
            stream_handle,
            sink,
        }
    }

    pub fn notify(&mut self, title: &str, body: &str) {
        // Show a desktop notification
        let _ = Notification::new()
            .summary(title)
            .body(body)
            .appname("pt")
            .timeout(Timeout::Never)
            .show();

        // Play a sound

        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(File::open(ALARM_FILE).unwrap());
        // Decode that sound file into a source
        let source = Decoder::new(file).unwrap();
        // Play the sound directly on the device
        self.sink.append(source);
        self.sink.play();
    }
}
