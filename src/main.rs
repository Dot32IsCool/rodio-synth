use rodio::OutputStream;
use rodio::source::Source;
use rodio::Sink;
use midir::MidiInput;
use std::collections::HashMap;
// Import synth module
mod oscillator;

fn main() {
    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // let sink = Sink::try_new(&stream_handle).unwrap();

    // Store a hashmap of sinks
    let mut sinks: HashMap<u8, Sink> = HashMap::new();

    // Create a new midi input
    let midi_in = MidiInput::new("midir reading input").unwrap();

    // Get an input port (Automatically choosing the first one) 
    // (It will panic if no midi device is connected)
    let in_port = &midi_in.ports()[0];

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(in_port, "midir-read-input", move |_stamp, message, _| {
        // Message is in the format of [event, key, pressure]
        let hz = 440.0 * 2.0_f32.powf((message[1] as f32 - 69.0) / 12.0);
        let pressure = message[2] as f32 / 127.0;

        // Detect whether the key is black or white
        let key = message[1] % 12;
        let is_black = key == 1 || key == 3 || key == 6 || key == 8 || key == 10;

        if message[0] == 144 { // 144 is the event for note on
            // Create a new sink for the key
            let mut sink = Sink::try_new(&stream_handle).unwrap();
            sink.append(oscillator::Oscilator::square_wave(hz).amplify(pressure));
            sinks.insert(message[1], sink);
        }
        if message[0] == 128 { // 128 is the event for note off
            sinks.remove(&message[1]);
        }
    }, ()).unwrap();

    loop {}
}
