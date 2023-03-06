# rodio synth
A synthesizer written in Rust, currently supporting multiple voices (you can hold multiple notes at the same time), and envelopes! I use Midir for midi keyboard input and Rodio for talking to the system's sound hardware.

### File breakdown

- `main.rs` - Handles the midi input and calling the various functions required in order to play notes. 55 lines.
- `oscillator.rs` - Creates an oscillator (a sine, square, sawtooth or triangle wave;) that implements Rodio's source trait. 100 lines.
- `synth.rs` - Manages audio sinks and envelopes, includes a .play() function that accepts a Rodio source as input. 112 lines.

Note that you currently need a midi device in order to use the program, otherwise there is no way to input notes and the program panics.
