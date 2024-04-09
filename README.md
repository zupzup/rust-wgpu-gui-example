# rust-wgpu-gui-example

Example for a very simple GUI built using wgpu

## Setup & Run

Simply run `RUST_LOG=info cargo run` to run the application and a simple GUI will open up, which you can interact with.

To build it for the web (you need a browser that supports WebGPU!), run `wasm-pack build --dev --target web --out-name wasm --out-dir ./static` and then `python3 -m http.server` to serve it at http://localhost:8000/index.html. 

