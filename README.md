# simapi-rs

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/GuillaumeDelorme/simapi-rs#license)
[![Crates.io](https://img.shields.io/crates/v/simapi.svg)](https://crates.io/crates/simapi)
[![Downloads](https://img.shields.io/crates/d/simapi.svg)](https://crates.io/crates/simapi)
[![Docs](https://docs.rs/simapi/badge.svg)](https://docs.rs/simapi/latest/simapi/)

Rust bindings for [simapi](https://github.com/Spacefreak18/simapi).

This workspace currently contains:

- `simapi-sys`: raw FFI bindings to the C library
- `simapi`: a safe Rust wrapper intended for applications such as dashboards, telemetry tools, or overlays

The safe crate follows the same discovery and mapping flow as the upstream C tools, then exposes telemetry as plain Rust types.

**Note:** this project only work on Linux.

## Basic usage

Add the crate:

```toml
[dependencies]
simapi = "0.1.0"
```

Enable serde support when you want to serialize or deserialize telemetry
snapshots:

```toml
[dependencies]
simapi = { version = "0.1.0", features = ["serde"] }
```

Read the current telemetry snapshot:

```rust,no_run
fn main() -> simapi::Result<()> {
    let mut client = simapi::Client::open()?;
    let snapshot = client.snapshot()?;

    println!(
        "sim_on={} status={:?} speed={} rpm={} gear={} car={} track={}",
        snapshot.sim_on,
        snapshot.status,
        snapshot.velocity,
        snapshot.rpms,
        snapshot.gearc,
        snapshot.car,
        snapshot.track,
    );

    Ok(())
}
```

Poll telemetry in a loop:

```rust,no_run
use std::{thread, time::Duration};

fn main() -> simapi::Result<()> {
    let mut client = simapi::Client::open()?;

    loop {
        let snapshot = client.snapshot()?;
        println!("speed={} rpm={}", snapshot.velocity, snapshot.rpms);
        thread::sleep(Duration::from_millis(100));
    }
}
```

If you need the raw generated bindings, use `simapi-sys`. If you are building a normal Rust application, `simapi` is the crate you probably want.

With the `serde` feature enabled, `TelemetrySnapshot` and the snapshot-related
public types it contains implement `serde::Serialize` and
`serde::Deserialize`.

## Example

There is a small example in `crates/simapi/examples/read_telemetry.rs`:

```bash
cargo run -p simapi --example read_telemetry
```

## License

Licensed under either MIT or Apache-2.0, at your option. See `LICENSE-MIT` and `LICENSE-APACHE`.
