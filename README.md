# Car Telemetry Dashboard

Car Telemetry Dashboard is a Rust-powered vehicle diagnostic simulation and visualization tool by Vineeth Velmurugan under **Vineeth Makes**. It models dynamic telemetry such as speed, RPM, temperatures, and tire pressures, and streams live data into a 3D digital twin interface in the browser.

## Features

- **Dynamic Rust Simulation:** Core logic for simulated driving physics, engine heat, and dynamic tire leaks.
- **Diagnostic Evaluation:** Real-time checking of safety thresholds (e.g., low tire pressure, overheating, battery warnings).
- **WebAssembly Integration:** Rust core compiled to WASM for seamless browser execution.
- **3D Visualization:** Real-time Three.js rendering that reacts to telemetry data and alerts.

## Tech Stack

- Rust (`serde`, `wasm-bindgen`)
- Three.js
- GitHub Actions for CI

## Run the Web Demo

First, ensure you have Rust and `wasm-pack` installed.

1. Build the Rust crate to WebAssembly:
   ```sh
   wasm-pack build crates/car-telemetry --target web --out-dir ../../web/pkg
   ```

2. Serve the web folder:
   ```sh
   cd web
   python3 -m http.server 8082
   ```

3. Open `http://localhost:8082` in your browser.

## Run Rust Tests

```sh
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

## License
MIT
