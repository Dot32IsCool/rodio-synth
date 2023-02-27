use rodio::OutputStream;
use rodio::source::Source;
// Import synth module
mod synth;

fn main() {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // // Play a sine wave at 220Hz
    // stream_handle.play_raw(synth::Synth::new(220.0)).unwrap();

    // Play a sine wave at 440Hz
    stream_handle.play_raw(synth::Synth::new(440.0).amplify(0.10)).unwrap();

    // // Play another sine wave at 880Hz
    // stream_handle.play_raw(synth::Synth::new(880.0)).unwrap();

    // // Play another sine wave at 1760Hz
    // stream_handle.play_raw(synth::Synth::new(1760.0)).unwrap();

    // // Play another sine wave at 3520Hz
    // stream_handle.play_raw(synth::Synth::new(3520.0)).unwrap();

    // Sleep for 1 second
    std::thread::sleep(std::time::Duration::from_secs(1));
}
