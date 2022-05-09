# hello-rust
Delving into rust via the motivation of learning more about wasm, webgpu and wgsl using https://bevyengine.org/

## setup

https://bevyengine.org/learn/book/getting-started/setup/#rust-setup
```shell
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo install basic-http-server
```


## run debug locally
```shell
cargo run
```
(optional fast builds)
```shell
cargo run --features bevy/dynamic
```

## run debug wasm
```shell
cargo build --target wasm32-unknown-unknown && wasm-bindgen --out-name hello_rust --out-dir target --target web target/wasm32-unknown-unknown/debug/hello_rust.wasm
basic-http-server .
```

## run release wasm
```shell
cargo build --release --target wasm32-unknown-unknown && wasm-bindgen --out-name hello_rust --out-dir target --target web target/wasm32-unknown-unknown/release/hello_rust.wasm
basic-http-server .
```
