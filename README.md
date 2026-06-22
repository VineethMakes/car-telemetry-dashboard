# Car Telemetry Dashboard

Car Telemetry Dashboard is a Rust-powered vehicle data viewer by Vineeth Velmurugan under **Vineeth Makes**. It turns simulated OBD-II style readings into trip metrics, health checks, maintenance signals, and a browser-rendered dashboard.

The goal is to make car data useful at a glance: speed, RPM, temperatures, battery voltage, fuel level, diagnostic codes, and driving efficiency in one interactive view.

## What It Does

- Models vehicle telemetry samples with speed, RPM, throttle, temperatures, fuel, voltage, and location
- Computes trip summaries for distance, efficiency, average speed, and peak engine load
- Flags diagnostics for overheating, low voltage, low fuel, and stored trouble codes
- Produces a time-series snapshot for dashboards and WASM consumers
- Renders a browser demo with gauges, health cards, diagnostic alerts, and a route trace

## Stack

- Rust for the telemetry model, diagnostics, trip analytics, tests, and WASM export
- `serde`/`serde_json` for structured snapshots
- `wasm-bindgen`/`wasm-pack` for browser integration
- Three.js for the live telemetry dashboard visualization
- GitHub Actions for Rust tests and clippy

## Run The Rust Checks

```sh
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

## Run The Browser Demo

Build the Rust crate to WASM:

```sh
wasm-pack build crates/car-telemetry --target web --out-dir ../../web/pkg --features wasm
```

Serve the web folder:

```sh
cd web
python3 -m http.server 8082
```

Then open:

```text
http://localhost:8082
```

## Project Shape

```text
.
├── crates/car-telemetry  # Rust telemetry model, diagnostics, trip logic, WASM export
├── web                   # Three.js dashboard consuming the Rust snapshot
└── .github/workflows     # CI
```

## Roadmap

- Add CSV and OBD-II adapter imports behind the current mock trip
- Persist trip history for comparisons over time
- Add maintenance intervals and cost estimates
- Add screenshots or a short walkthrough video to the README
- Turn alert thresholds into configurable vehicle profiles
