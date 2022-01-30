cargo build --target wasm32-unknown-unknown --release

wasm-bindgen --out-dir docs/demo --out-name bevy6tests --target web --no-typescript target/wasm32-unknown-unknown/release/bevy6tests.wasm

cargo install -f wasm-bindgen-cli --version 0.2.78