# hello-rust
Delving into a little rust via the motivation of learning it, webGPU and wasm, planning on using https://bevyengine.org/

## setup
https://bevyengine.org/learn/book/getting-started/setup/#rust-setup

## run
```shell
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name hello_rust --out-dir target --target web target/wasm32-unknown-unknown/release/hello_rust.wasm
cargo install basic-http-server
basic-http-server .
```
