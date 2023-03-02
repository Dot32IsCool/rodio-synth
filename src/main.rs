use rodio::OutputStream;
use rodio::source::Source;
use rodio::{Decoder, Sink};
use midir::{MidiInput, Ignore};
// Import synth module
mod synth;

fn main() {
    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Create a new midi input
    let mut midi_in = MidiInput::new("midir reading input").unwrap();
    midi_in.ignore(Ignore::None);

    // Get an input port (Automatically choosing the first one)
    let in_port = &midi_in.ports()[0];

    let _conn_in = midi_in.connect(in_port, "midir-read-input", move |stamp, message, _| {
        let hz = 440.0 * 2.0_f32.powf((message[1] as f32 - 69.0) / 12.0);
        let pressure = message[2] as f32 / 127.0;

        if message[0] == 144 {
            sink.stop();
            sink.append(synth::Synth::sawtooth_wave(hz).amplify(pressure));
        }
        if message[0] == 128 {
            sink.stop();
        }
    }, ()).unwrap();

    loop {}
}
