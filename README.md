# TCA6424 Rust Driver Library

This is a Rust driver library for the Texas Instruments TCA6424 low-voltage 24-bit I2C I/O expander. It is based on `embedded-hal` traits and provides support for both synchronous (sync) and asynchronous (async) operation modes via the `maybe-async-cfg` crate.

## Features

- Supports `embedded-hal` and `embedded-hal-async` I2C traits.
- Implements sync/async abstraction using `maybe-async-cfg`.
- Provides pin-level control methods:
  - Set/Get pin direction (input/output)
  - Set/Get output pin state (high/low)
  - Get input pin physical state (high/low)
  - Set/Get pin polarity inversion
- Provides port-level control methods:
  - Set/Get port direction mask
  - Set/Get output port state mask
  - Get input port physical state mask
  - Set/Get port polarity inversion mask
- Supports Auto-Increment read/write operations for efficient manipulation of consecutive registers.

## Compatibility

This driver library is designed for `no_std` environments, suitable for various embedded systems. With the `async` feature, it can be used in asynchronous runtime environments that support `embedded-hal-async`.

## Usage

Add `tca6424` to the `[dependencies]` section of your `Cargo.toml` file:

```toml
tca6424 = "0.1.0" # Or point to a local path { path = "path/to/tca6424-rs" }
```

If you need asynchronous support, enable the `async` feature:

```toml
tca6424 = { version = "0.1.0", features = ["async"] }
```

If you use `defmt` for logging, enable the `defmt` feature:

```toml
tca6424 = { version = "0.1.0", features = ["defmt"] }
```

### Basic Usage (Async Example)

```rust
use defmt::info;
use tca6424::{Tca6424, PinDirection, Port, DEFAULT_ADDRESS};
use embedded_hal_async::i2c::I2c;

// Example of core driver usage. For a complete embedded project setup,
// please refer to `examples/stm32g4/src/main.rs`.
// Assume you have an `i2c_bus` instance, e.g., `let mut i2c_bus = ...;`
// and call this code within an asynchronous context.

let address = DEFAULT_ADDRESS;
let mut tca = Tca6424::new(i2c_bus, address).await.unwrap();
info!("TCA6424 driver instance created successfully.");

let port0_direction_mask = 0b1111_0000;
tca.set_port_direction(Port::Port0, port0_direction_mask).await.unwrap();
info!("Port0 direction set to {:08b}.", port0_direction_mask);

let input_mask = tca.get_port_input_state(Port::Port2).await.unwrap();
info!("Read Port2 input state: {:08b}", input_mask);
```

### Example Code

You can find more complete examples in the `examples/` directory:

- [`examples/stm32g4/`](examples/stm32g4/): An asynchronous example running on an STM32G4 microcontroller using the Embassy framework.

To build the STM32G4 example (requires `thumbv7em-none-eabihf` target and `rust-src` component):

```bash
cd examples/stm32g4
cargo build
```

To run the STM32G4 example (requires probe-rs and a compatible debug probe):

```bash
cd examples/stm32g4
cargo run # Uses the runner configured in .cargo/config.toml
```

## Datasheet

For detailed information on the TCA6424, please refer to the datasheet: [`docs/tca6424.md`](docs/tca6424.md) ([`PDF`](docs/tca6424.pdf))

## License

This project is dual-licensed under the MIT or Apache-2.0 licenses. See the [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) files for details.
