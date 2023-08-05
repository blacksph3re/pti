use std::fs::read;
use std::io::Cursor;

use notify_rust::{Notification, Timeout};
use rodio::{Decoder, OutputStream, Sink, OutputStreamHandle};
use crate::constants::{ALARM_FILE, get_full_path};

#[allow(dead_code)]
pub struct NotificationManager {
    alarm_sound: Vec<u8>,
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Sink,
}

impl NotificationManager {
    pub fn new() -> Self {
        let alarm_path = get_full_path(ALARM_FILE);
        let alarm_sound = read(alarm_path.as_path()).expect("Failed to read alarm file.");
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Self {
            alarm_sound,
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
        let reader = Cursor::new(self.alarm_sound.clone());
        // Decode that sound file into a source
        let source = Decoder::new(reader).unwrap();
        // Play the sound directly on the device
        self.sink.append(source);
        self.sink.play();
    }
}
