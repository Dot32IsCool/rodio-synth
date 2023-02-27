use rodio::OutputStream;
use rodio::source::Source;
// Import synth module
mod synth;

fn main() {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Play a sine wave at 440Hz
    // stream_handle.play_raw(synth::Synth::sine_wave(440.0).amplify(0.10)).unwrap();
    stream_handle.play_raw(synth::Synth::triangle_wave(440.0).amplify(0.10)).unwrap();

    // Sleep for 1 second
    std::thread::sleep(std::time::Duration::from_secs(1));
}
