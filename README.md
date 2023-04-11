# Rust WASM Audio Worklet Example
A small example demonstrating how Audio DSP code implemented
can be compiled down Web Assembly and then loaded inside of
an AudioWorklet.

I wanted to keep this fairly minimal, but I indulged a
little bit. It is possible to use this with `rustc` directly
(see the `build.sh` script), but I used Cargo instead,
because it was more practical. Typically these sorts of
"hello audio" examples feature things like noise and tone
generators. This example is essentially a tone generator, but
I couldn't resist spicing things up a bit by turning
my sine tone into a [phase distortion oscillator](https://en.wikipedia.org/wiki/Phase_distortion_synthesis), modulated using an LFO generated using
the [magic circle algorithm](https://ccrma.stanford.edu/~jos/pasp/Digital_Sinusoid_Generators.html). The resulting sound is a moderate improvement over a sinusoid.

## Building and Running

First install the prerequisites.

`wasm-gc` is a utility used to remove unused bits

```sh
cargo install --git https://github.com/alexcrichton/wasm-gc
```

Make sure the webassembly target is added.

```sh
rustup target add wasm32-unknown-unknown
```

Then, build the project with:

```sh
sh build.sh
```

Spin up a local http server with:

```sh
python3 -m http.server
```

Then, in a browser, open up `http://localhost:8000`, and
click the "BEGIN" button to start the audio.

## How it Works

This example works by implementing the
`AudioWorkletProcessor` components in JavaScript, and
then making calls to WebAssembly inside of the process
callback to fill up the buffers.

Rust code implements just the DSP code, and provides
a callback for filling a single buffer of audio which
is exported as a C-style wrapper function using `#[no_mangle]`.

When it is compiled, it generates a wasm file which is further
reduced in size by `wasm-gc`.

AudioWorkletProcessors are somewhat restricted in terms of
what they can access or do, and figuring out how to actually
get wasm code to reach the callback took a little bit of trial
and error. The approach I took was to load up the bytecode
and then send that to the AudioWorklet constructor. From
there, I was able to synchronously instantiate the WebAssembly.

There's a little bit of work invovled in getting Rust to write
to buffers that Javascript can read. A lot of the logic there
was adapted from the Glicol engine. The actual process
callback that writes audio to a buffer requires unsafe Rust
code.

## Resources

Minimal rust + wasm. A really great no-frills way to get Rust
code working inside of a browser: https://www.hellorust.com/demos/add/index.html

Glicol, a projet with an audio DSP engine in Rust, exported
to an audio worklet. It was nice to study how an existing
approach worked: https://github.com/chaosprint/glicol

AudioWorkletProcessor reference. Some helpful details here:
https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletProcessor

The Rust and WebAssembly Book. I actually couldn't get the bindings 
generated by `wasm-pack` to work inside of an AudioWorklet,
BUT it was still a very helpful tutorial to follow, and it
was helpful studying the generated JavaScript:
https://rustwasm.github.io/docs/book/print.html

GoogleChromeLabs Web Audio Samples had some really nice
small self-contained snippets for Audio Worklets:
https://github.com/GoogleChromeLabs/web-audio-samples

The FAUST webaudio architecture was a useful study for how
WASM could be used inside of an AudioWorklets.

This was from a webaudio and Rust tutorial I found online.
Another useful point of reference for study:
https://github.com/peter-suggate/wasm-audio-app/