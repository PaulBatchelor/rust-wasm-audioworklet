cargo build --target wasm32-unknown-unknown --release
wasm-gc target/wasm32-unknown-unknown/release/rust_wasm_audioworklet.wasm dsp.wasm
