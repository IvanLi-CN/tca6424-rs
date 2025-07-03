//! Texas Instruments TCA6424 24-bit I2C I/O Expander Driver Library
//!
//! This crate provides a platform-agnostic driver for the TCA6424 I/O expander,
//! compatible with the `embedded-hal` and `embedded-hal-async` traits.
//!
//! It supports both synchronous and asynchronous operation via the `maybe-async-cfg` crate.
//!
//! ## Features
//!
//! - `default`: Enables the `std` feature.
//! - `std`: Enables standard library support (for `std::error::Error` implementation).
//! - `async`: Enables asynchronous support using `embedded-hal-async`.
//! - `defmt`: Enables `defmt::Format` implementations for data types and errors.
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! tca6424 = "0.1.0" # Or specify a git path/rev
//! embedded-hal = "0.2" # Or "1.0" depending on your needs
//! # For async support:
//! # embedded-hal-async = "0.2"
//! # maybe-async-cfg = "0.3"
//! # For defmt support:
//! # defmt = "0.3"
//! ```
//!
//! ### Asynchronous Example (using `embedded-hal-mock`)
//!
//! ```no_run
//! #[cfg(feature = "async")]
//! async fn async_main() {
//!     use tca6424::{Tca6424, Pin, PinDirection, PinState, Port};
//!     use embedded_hal_mock::eh_async::i2c::{MockI2c, Transaction};
//!     use embedded_hal_async::i2c::{Read, Write, WriteRead};
//!
//!     // Define expected I2C transactions for the mock
//!     let expectations = [
//!         // Example: Set P00 as output
//!         Transaction::write_read(0x74, &[0x06], &[0xFF]), // Read Configuration Port 0
//!         Transaction::write(0x74, &[0x06, 0xFE]),       // Write Configuration Port 0 (clear bit 0)
//!         // Example: Set P00 high
//!         Transaction::write_read(0x74, &[0x02], &[0x00]), // Read Output Port 0
//!         Transaction::write(0x74, &[0x02, 0x01]),       // Write Output Port 0 (set bit 0)
//!     ];
//!
//!     let mut i2c = MockI2c::new(&expectations);
//!     let mut expander = Tca6424::new(&mut i2c, 0x74).await.unwrap();
//!
//!     // Set P00 as output
//!     expander.set_pin_direction(Pin::P00, PinDirection::Output).await.unwrap();
//!
//!     // Set P00 high
//!     expander.set_pin_output(Pin::P00, PinState::High).await.unwrap();
//!
//!     // Check that all transactions were executed
//!     i2c.done();
//! }
//!
//! #[cfg(feature = "async")]
//! #[tokio::main] // Or other async runtime
//! async fn main() {
//!     async_main().await;
//! }
//!
//! #[cfg(not(feature = "async"))]
//! fn main() {
//!     // Async feature not enabled, do nothing or provide a sync main
//! }
//! ```
//!
//! For a real hardware example, see `examples/stm32g4`.
//!
//! ## License
//!
//! This project is licensed under either of
//!
//! * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
//! * MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//! ### Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in the work by you, as defined in the Apache-2.0 license, shall
//! be dual licensed as above, without any additional terms or conditions.
//!
//! ## Contributing
//!
//! Contributions are welcome! Please see the [repository](https://github.com/your-username/tca6424-rs) for details.
//!
//! ## Changelog
//!
//! See [CHANGELOG.md](CHANGELOG.md) (Not yet created)
//!
//! ## ToDo
//!
//! See [PLAN.md](PLAN.md)

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c;

mod data_types;
pub mod errors;
mod registers;

use crate::errors::Error;
pub use data_types::*;

/// Default I2C address for the TCA6424 (when ADDR pins are tied low).
/// Default I2C address for the TCA6424 (when ADDR pins are tied low).
/// According to PLAN.md and datasheet Table 3 (ADDR=L).
pub const DEFAULT_ADDRESS: u8 = 0x22;

/// Driver for the Texas Instruments TCA6424 24-bit I2C I/O Expander.
///
/// This struct provides methods to interact with the TCA6424 via an I2C bus,
/// allowing control over pin direction, output state, input state, and polarity inversion.
///
/// It is generic over the I2C bus implementation, supporting both synchronous
/// and asynchronous `embedded-hal` traits via `maybe-async-cfg`.
pub struct Tca6424<'a, I2C> {
    i2c: &'a mut I2C,
    address: u8,
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), self = "Tca6424",),
    async(feature = "async", keep_self)
)]
impl<'a, I2C> Tca6424<'a, I2C>
where
    I2C: I2c,
    I2C::Error: core::fmt::Debug,
{
    /// Creates a new TCA6424 driver instance.
    ///
    /// This function is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `i2c` - A mutable reference to the I2C bus instance, implementing
    ///           `embedded-hal::i2c::I2c` (sync) or `embedded-hal-async::i2c::I2c` (async).
    /// * `address` - The I2C slave address of the TCA6424 device.
    ///
    /// # Returns
    ///
    /// Returns `Ok(Self)` on success, or an `Error` if the I2C bus operation fails.
    pub fn new(i2c: &'a mut I2C, address: u8) -> Result<Self, Error<I2C::Error>> {
        Ok(Self { i2c, address })
    }

    /// Writes a single byte to the specified register.
    ///
    /// This is a low-level internal method. It handles sending the command byte
    /// but does not use the auto-increment feature.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `register` - The target register.
    /// * `value` - The byte value to write.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the I2C bus operation fails.
    async fn write_register(
        &mut self,
        register: registers::Register,
        value: u8,
    ) -> Result<(), Error<I2C::Error>> {
        // Command byte: AI=0 (Bit 7), Register address (Bit 0-6)
        let command_byte = register as u8; // AI=0 by default from enum value
        let buffer = [command_byte, value];
        self.i2c.write(self.address, &buffer).await.map_err(Error::I2c)
    }

    /// Reads a single byte from the specified register.
    ///
    /// This is a low-level internal method. It handles sending the command byte
    /// and the repeated start condition, but does not use the auto-increment feature.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `register` - The target register.
    ///
    /// # Returns
    ///
    /// Returns `Ok(u8)` containing the read byte on success, or an `Error` if the I2C bus operation fails.
    async fn read_register(
        &mut self,
        register: registers::Register,
    ) -> Result<u8, Error<I2C::Error>> {
        // Command byte: AI=0 (Bit 7), Register address (Bit 0-6)
        let command_byte = register as u8; // AI=0 by default from enum value
        let mut read_buffer = [0u8];
        // Send command byte (write mode), then repeated start and read data (read mode)
        self.i2c
            .write_read(self.address, &[command_byte], &mut read_buffer).await
            .map_err(Error::I2c)?;
        Ok(read_buffer[0])
    }

    /// Writes multiple consecutive bytes starting from the specified register, enabling auto-increment.
    ///
    /// This is a low-level internal method. It sets the auto-increment bit in the command byte.
    /// The TCA6424 automatically increments the register address after each byte transfer.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `start_register` - The starting register address.
    /// * `values` - A slice of bytes to write. The number of bytes written will be
    ///              limited to the number of registers available from `start_register`
    ///              to the end of the register map (max 3 for a port group).
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the I2C bus operation fails.
    async fn write_registers_ai(
        &mut self,
        start_register: registers::Register,
        values: &[u8],
    ) -> Result<(), Error<I2C::Error>> {
        // Command byte: AI=1 (Bit 7), Register address (Bit 0-6)
        let command_byte = (start_register as u8) | 0x80; // Set AI bit
        let mut buffer = [0u8; 1 + 3]; // Max 3 bytes for a port group + 1 command byte
        buffer[0] = command_byte;
        let len = core::cmp::min(values.len(), 3); // TCA6424 has 3 registers per group
        buffer[1..len + 1].copy_from_slice(&values[..len]);

        self.i2c
            .write(self.address, &buffer[..len + 1]).await
            .map_err(Error::I2c)
    }

    /// Reads multiple consecutive bytes starting from the specified register, enabling auto-increment.
    ///
    /// This is a low-level internal method. It sets the auto-increment bit in the command byte
    /// and handles the repeated start condition. The TCA6424 automatically increments the
    /// register address after each byte transfer.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `start_register` - The starting register address.
    /// * `buffer` - A mutable slice to store the read bytes. The number of bytes read
    ///              is determined by the length of this buffer.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the I2C bus operation fails.
    async fn read_registers_ai(
        &mut self,
        start_register: registers::Register,
        buffer: &mut [u8],
    ) -> Result<(), Error<I2C::Error>> {
        // Command byte: AI=1 (Bit 7), Register address (Bit 0-6)
        let command_byte = (start_register as u8) | 0x80; // Set AI bit
        // Send command byte (write mode), then repeated start and read data (read mode)
        self.i2c
            .write_read(self.address, &[command_byte], buffer).await
            .map_err(Error::I2c)
    }

    /// Sets the direction of a single pin (Input or Output).
    ///
    /// This method reads the current configuration register for the pin's port,
    /// modifies the bit corresponding to the pin, and writes the value back.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `pin` - The target pin (P00-P27).
    /// * `direction` - The desired pin direction (`PinDirection::Input` or `PinDirection::Output`).
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if an I2C bus operation fails or
    /// if an invalid pin is provided (though the `Pin` enum should prevent this).
    pub async fn set_pin_direction(
        &mut self,
        pin: Pin,
        direction: PinDirection,
    ) -> Result<(), Error<I2C::Error>> {
        let pin_index = pin as u8;
        let port_index = pin_index / 8;
        let bit_index = pin_index % 8;
        let config_register = match port_index {
            0 => registers::Register::ConfigurationPort0,
            1 => registers::Register::ConfigurationPort1,
            2 => registers::Register::ConfigurationPort2,
            _ => return Err(Error::InvalidRegisterOrPin), // Should not happen with valid Pin enum
        };
        let mut config_value = self.read_register(config_register).await?;
        match direction {
            PinDirection::Input => {
                config_value |= 1 << bit_index; // Set bit to 1 (Input)
            }
            PinDirection::Output => {
                config_value &= !(1 << bit_index); // Clear bit to 0 (Output)
            }
        }
        self.write_register(config_register, config_value).await
    }

    /// Gets the current direction of a single pin (Input or Output).
    ///
    /// This method reads the configuration register for the pin's port and
    /// extracts the bit corresponding to the pin.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `pin` - The target pin (P00-P27).
    ///
    /// # Returns
    ///
    /// Returns `Ok(PinDirection)` on success, or an `Error` if an I2C bus operation fails or
    /// if an invalid pin is provided.
    pub async fn get_pin_direction(&mut self, pin: Pin) -> Result<PinDirection, Error<I2C::Error>> {
        let pin_index = pin as u8;
        let port_index = pin_index / 8;
        let bit_index = pin_index % 8;
        let config_register = match port_index {
            0 => registers::Register::ConfigurationPort0,
            1 => registers::Register::ConfigurationPort1,
            2 => registers::Register::ConfigurationPort2,
            _ => return Err(Error::InvalidRegisterOrPin), // Should not happen with valid Pin enum
        };
        let config_value = self.read_register(config_register).await?;
        if (config_value >> bit_index) & 1 == 1 {
            Ok(PinDirection::Input)
        } else {
            Ok(PinDirection::Output)
        }
    }

    /// Sets the output state of a single pin (High or Low).
    ///
    /// This method reads the current output register for the pin's port,
    /// modifies the bit corresponding to the pin, and writes the value back.
    ///
    /// Note: This method only affects pins configured as outputs.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `pin` - The target pin (P00-P27).
    /// * `state` - The desired pin state (`PinState::High` or `PinState::Low`).
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if an I2C bus operation fails or
    /// if an invalid pin is provided.
    pub async fn set_pin_output(
        &mut self,
        pin: Pin,
        state: PinState,
    ) -> Result<(), Error<I2C::Error>> {
        let pin_index = pin as u8;
        let port_index = pin_index / 8;
        let bit_index = pin_index % 8;
        let output_register = match port_index {
            0 => registers::Register::OutputPort0,
            1 => registers::Register::OutputPort1,
            2 => registers::Register::OutputPort2,
            _ => return Err(Error::InvalidRegisterOrPin), // Should not happen with valid Pin enum
        };
        let mut output_value = self.read_register(output_register).await?;
        match state {
            PinState::High => {
                output_value |= 1 << bit_index; // Set bit to 1 (High)
            }
            PinState::Low => {
                output_value &= !(1 << bit_index); // Clear bit to 0 (Low)
            }
        }
        self.write_register(output_register, output_value).await
    }

    /// Gets the current state of a single pin from the Output Port register.
    ///
    /// This method reads the output register for the pin's port and extracts
    /// the bit corresponding to the pin.
    ///
    /// Note: This method reads the register value, not the actual physical pin state.
    /// The register value reflects the actual pin state only when the pin is configured as an output.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `pin` - The target pin (P00-P27).
    ///
    /// # Returns
    ///
    /// Returns `Ok(PinState)` on success, or an `Error` if an I2C bus operation fails or
    /// if an invalid pin is provided.
    pub async fn get_pin_output_state(&mut self, pin: Pin) -> Result<PinState, Error<I2C::Error>> {
        let pin_index = pin as u8;
        let port_index = pin_index / 8;
        let bit_index = pin_index % 8;
        let output_register = match port_index {
            0 => registers::Register::OutputPort0,
            1 => registers::Register::OutputPort1,
            2 => registers::Register::OutputPort2,
            _ => return Err(Error::InvalidRegisterOrPin), // Should not happen with valid Pin enum
        };
        let output_value = self.read_register(output_register).await?;
        if (output_value >> bit_index) & 1 == 1 {
            Ok(PinState::High)
        } else {
            Ok(PinState::Low)
        }
    }

    /// Gets the current physical state of a single pin (High or Low).
    ///
    /// This method reads the input register for the pin's port and extracts
    /// the bit corresponding to the pin.
    ///
    /// Note: This method reads the Input Port register, which reflects the actual
    /// physical state of the pin, regardless of its configuration (input or output).
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `pin` - The target pin (P00-P27).
    ///
    /// # Returns
    ///
    /// Returns `Ok(PinState)` on success, or an `Error` if an I2C bus operation fails or
    /// if an invalid pin is provided.
    pub async fn get_pin_input_state(&mut self, pin: Pin) -> Result<PinState, Error<I2C::Error>> {
        let pin_index = pin as u8;
        let port_index = pin_index / 8;
        let bit_index = pin_index % 8;
        let input_register = match port_index {
            0 => registers::Register::InputPort0,
            1 => registers::Register::InputPort1,
            2 => registers::Register::InputPort2,
            _ => return Err(Error::InvalidRegisterOrPin), // Should not happen with valid Pin enum
        };
        let input_value = self.read_register(input_register).await?;
        if (input_value >> bit_index) & 1 == 1 {
            Ok(PinState::High)
        } else {
            Ok(PinState::Low)
        }
    }

    /// Sets the polarity inversion state for a single pin.
    ///
    /// This method reads the current polarity inversion register for the pin's port,
    /// modifies the bit corresponding to the pin, and writes the value back.
    ///
    /// If inversion is enabled (the corresponding bit in the Polarity Inversion register is 1),
    /// the input value from the Input Port register is inverted before being read.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `pin` - The target pin (P00-P27).
    /// * `invert` - `true` to enable polarity inversion, `false` to disable.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if an I2C bus operation fails or
    /// if an invalid pin is provided.
    pub async fn set_pin_polarity_inversion(
        &mut self,
        pin: Pin,
        invert: bool,
    ) -> Result<(), Error<I2C::Error>> {
        let pin_index = pin as u8;
        let port_index = pin_index / 8;
        let bit_index = pin_index % 8;
        let polarity_register = match port_index {
            0 => registers::Register::PolarityInversionPort0,
            1 => registers::Register::PolarityInversionPort1,
            2 => registers::Register::PolarityInversionPort2,
            _ => return Err(Error::InvalidRegisterOrPin), // Should not happen with valid Pin enum
        };
        let mut polarity_value = self.read_register(polarity_register).await?;
        if invert {
            polarity_value |= 1 << bit_index; // Set bit to 1 (Invert)
        } else {
            polarity_value &= !(1 << bit_index); // Clear bit to 0 (Original)
        }
        self.write_register(polarity_register, polarity_value).await
    }

    /// Gets the current polarity inversion state for a single pin.
    ///
    /// This method reads the polarity inversion register for the pin's port and
    /// extracts the bit corresponding to the pin.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `pin` - The target pin (P00-P27).
    ///
    /// # Returns
    ///
    /// Returns `Ok(bool)` where `true` indicates inversion is enabled, `false` otherwise,
    /// or an `Error` if an I2C bus operation fails or if an invalid pin is provided.
    pub async fn get_pin_polarity_inversion(
        &mut self,
        pin: Pin,
    ) -> Result<bool, Error<I2C::Error>> {
        let pin_index = pin as u8;
        let port_index = pin_index / 8;
        let bit_index = pin_index % 8;
        let polarity_register = match port_index {
            0 => registers::Register::PolarityInversionPort0,
            1 => registers::Register::PolarityInversionPort1,
            2 => registers::Register::PolarityInversionPort2,
            _ => return Err(Error::InvalidRegisterOrPin), // Should not happen with valid Pin enum
        };
        let polarity_value = self.read_register(polarity_register).await?;
        Ok(((polarity_value >> bit_index) & 1) == 1)
    }

    /// Sets the direction of all 8 pins on a specific port simultaneously.
    ///
    /// This method writes directly to the configuration register for the specified port.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `port` - The target port (`Port::Port0`, `Port::Port1`, or `Port::Port2`).
    /// * `direction_mask` - An 8-bit mask where each bit corresponds to a pin on the port.
    ///                      A bit value of `1` sets the corresponding pin as an input,
    ///                      and `0` sets it as an output.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the I2C bus operation fails.
    pub async fn set_port_direction(
        &mut self,
        port: Port,
        direction_mask: u8,
    ) -> Result<(), Error<I2C::Error>> {
        let config_register = match port {
            Port::Port0 => registers::Register::ConfigurationPort0,
            Port::Port1 => registers::Register::ConfigurationPort1,
            Port::Port2 => registers::Register::ConfigurationPort2,
        };
        self.write_register(config_register, direction_mask).await
    }

    /// Gets the current direction configuration mask for a specific port.
    ///
    /// This method reads the configuration register for the specified port.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `port` - The target port (`Port::Port0`, `Port::Port1`, or `Port::Port2`).
    ///
    /// # Returns
    ///
    /// Returns `Ok(u8)` containing an 8-bit mask on success, where each bit corresponds
    /// to a pin on the port (`1` = Input, `0` = Output), or an `Error` if the I2C
    /// bus operation fails.
    pub async fn get_port_direction(&mut self, port: Port) -> Result<u8, Error<I2C::Error>> {
        let config_register = match port {
            Port::Port0 => registers::Register::ConfigurationPort0,
            Port::Port1 => registers::Register::ConfigurationPort1,
            Port::Port2 => registers::Register::ConfigurationPort2,
        };
        self.read_register(config_register).await
    }

    /// Sets the output state of all 8 pins on a specific port simultaneously.
    ///
    /// This method writes directly to the output register for the specified port.
    ///
    /// Note: This only affects pins configured as outputs. Pins configured as inputs
    /// will retain their output register value, but it will not drive the physical pin.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `port` - The target port (`Port::Port0`, `Port::Port1`, or `Port::Port2`).
    /// * `output_mask` - An 8-bit mask where each bit corresponds to a pin on the port.
    ///                   A bit value of `1` sets the corresponding pin's output to High,
    ///                   and `0` sets it to Low.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the I2C bus operation fails.
    pub async fn set_port_output(
        &mut self,
        port: Port,
        output_mask: u8,
    ) -> Result<(), Error<I2C::Error>> {
        let output_register = match port {
            Port::Port0 => registers::Register::OutputPort0,
            Port::Port1 => registers::Register::OutputPort1,
            Port::Port2 => registers::Register::OutputPort2,
        };
        self.write_register(output_register, output_mask).await
    }

    /// Gets the current output state mask for a specific port from the Output Port register.
    ///
    /// This method reads the output register for the specified port.
    ///
    /// Note: This reads the register value, not the actual physical pin state.
    /// The register value reflects the actual pin state only when the pin is configured as an output.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `port` - The target port (`Port::Port0`, `Port::Port1`, or `Port::Port2`).
    ///
    /// # Returns
    ///
    /// Returns `Ok(u8)` containing an 8-bit mask on success, where each bit corresponds
    /// to a pin on the port (`1` = High, `0` = Low), or an `Error` if the I2C
    /// bus operation fails.
    pub async fn get_port_output_state(&mut self, port: Port) -> Result<u8, Error<I2C::Error>> {
        let output_register = match port {
            Port::Port0 => registers::Register::OutputPort0,
            Port::Port1 => registers::Register::OutputPort1,
            Port::Port2 => registers::Register::OutputPort2,
        };
        self.read_register(output_register).await
    }

    /// Gets the current physical state mask for all 8 pins on a specific port.
    ///
    /// This method reads the Input Port register for the specified port.
    ///
    /// Note: This reads the Input Port register, which reflects the actual
    /// physical state of the pins, regardless of their configuration (input or output).
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `port` - The target port (`Port::Port0`, `Port::Port1`, or `Port::Port2`).
    ///
    /// # Returns
    ///
    /// Returns `Ok(u8)` containing an 8-bit mask on success, where each bit corresponds
    /// to a pin on the port (`1` = High, `0` = Low), or an `Error` if the I2C
    /// bus operation fails.
    pub async fn get_port_input_state(&mut self, port: Port) -> Result<u8, Error<I2C::Error>> {
        let input_register = match port {
            Port::Port0 => registers::Register::InputPort0,
            Port::Port1 => registers::Register::InputPort1,
            Port::Port2 => registers::Register::InputPort2,
        };
        self.read_register(input_register).await
    }

    /// Sets the polarity inversion state for all 8 pins on a specific port simultaneously.
    ///
    /// This method writes directly to the polarity inversion register for the specified port.
    ///
    /// If inversion is enabled (the corresponding bit in the Polarity Inversion register is 1),
    /// the input value from the Input Port register is inverted before being read.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `port` - The target port (`Port::Port0`, `Port::Port1`, or `Port::Port2`).
    /// * `inversion_mask` - An 8-bit mask where each bit corresponds to a pin on the port.
    ///                      A bit value of `1` enables polarity inversion for the corresponding pin,
    ///                      and `0` disables it.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the I2C bus operation fails.
    pub async fn set_port_polarity_inversion(
        &mut self,
        port: Port,
        inversion_mask: u8,
    ) -> Result<(), Error<I2C::Error>> {
        let polarity_register = match port {
            Port::Port0 => registers::Register::PolarityInversionPort0,
            Port::Port1 => registers::Register::PolarityInversionPort1,
            Port::Port2 => registers::Register::PolarityInversionPort2,
        };
        self.write_register(polarity_register, inversion_mask).await
    }

    /// Gets the current polarity inversion state mask for a specific port.
    ///
    /// This method reads the polarity inversion register for the specified port.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `port` - The target port (`Port::Port0`, `Port::Port1`, or `Port::Port2`).
    ///
    /// # Returns
    ///
    /// Returns `Ok(u8)` containing an 8-bit mask on success, where each bit corresponds
    /// to a pin on the port (`1` = Inverted, `0` = Original), or an `Error` if the I2C
    /// bus operation fails.
    pub async fn get_port_polarity_inversion(
        &mut self,
        port: Port,
    ) -> Result<u8, Error<I2C::Error>> {
        let polarity_register = match port {
            Port::Port0 => registers::Register::PolarityInversionPort0,
            Port::Port1 => registers::Register::PolarityInversionPort1,
            Port::Port2 => registers::Register::PolarityInversionPort2,
        };
        self.read_register(polarity_register).await
    }

    // --- Auto-Increment Methods ---

    /// Sets the direction of multiple consecutive ports using the auto-increment feature.
    ///
    /// This method writes to the configuration registers for the specified ports,
    /// starting from `start_port`. The number of ports affected is determined by
    /// the length of the `direction_masks` slice.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `start_port` - The starting port (`Port::Port0`, `Port::Port1`, or `Port::Port2`).
    /// * `direction_masks` - A slice of 8-bit masks. Each mask corresponds to a port,
    ///                       starting from `start_port`. A bit value of `1` sets the
    ///                       corresponding pin as an input, and `0` sets it as an output.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the I2C bus operation fails.
    pub async fn set_ports_direction_ai(
        &mut self,
        start_port: Port,
        direction_masks: &[u8],
    ) -> Result<(), Error<I2C::Error>> {
        let start_register = match start_port {
            Port::Port0 => registers::Register::ConfigurationPort0,
            Port::Port1 => registers::Register::ConfigurationPort1,
            Port::Port2 => registers::Register::ConfigurationPort2,
        };
        self.write_registers_ai(start_register, direction_masks)
            .await
    }

    /// Gets the current direction configuration masks for multiple consecutive ports using the auto-increment feature.
    ///
    /// This method reads from the configuration registers for the specified ports,
    /// starting from `start_port`. The number of ports read is determined by the
    /// length of the provided `buffer`.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `start_port` - The starting port (`Port::Port0`, `Port::Port1`, or `Port::Port2`).
    /// * `buffer` - A mutable slice to store the read 8-bit masks. Each mask corresponds
    ///              to a port, starting from `start_port`. A bit value of `1` indicates
    ///              the corresponding pin is configured as an input, and `0` indicates output.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the I2C bus operation fails.
    pub async fn get_ports_direction_ai(
        &mut self,
        start_port: Port,
        buffer: &mut [u8],
    ) -> Result<(), Error<I2C::Error>> {
        let start_register = match start_port {
            Port::Port0 => registers::Register::ConfigurationPort0,
            Port::Port1 => registers::Register::ConfigurationPort1,
            Port::Port2 => registers::Register::ConfigurationPort2,
        };
        self.read_registers_ai(start_register, buffer).await
    }

    /// Sets the output state of multiple consecutive ports using the auto-increment feature.
    ///
    /// This method writes to the output registers for the specified ports,
    /// starting from `start_port`. The number of ports affected is determined by
    /// the length of the `output_masks` slice.
    ///
    /// Note: This only affects pins configured as outputs.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `start_port` - The starting port (`Port::Port0`, `Port::Port1`, or `Port::Port2`).
    /// * `output_masks` - A slice of 8-bit masks. Each mask corresponds to a port,
    ///                    starting from `start_port`. A bit value of `1` sets the
    ///                    corresponding pin's output to High, and `0` sets it to Low.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the I2C bus operation fails.
    pub async fn set_ports_output_ai(
        &mut self,
        start_port: Port,
        output_masks: &[u8],
    ) -> Result<(), Error<I2C::Error>> {
        let start_register = match start_port {
            Port::Port0 => registers::Register::OutputPort0,
            Port::Port1 => registers::Register::OutputPort1,
            Port::Port2 => registers::Register::OutputPort2,
        };
        self.write_registers_ai(start_register, output_masks).await
    }

    /// Gets the current output state masks for multiple consecutive ports using the auto-increment feature.
    ///
    /// This method reads from the output registers for the specified ports,
    /// starting from `start_port`. The number of ports read is determined by the
    /// length of the provided `buffer`.
    ///
    /// Note: This reads the register values, not the actual physical pin states.
    /// The register values reflect the actual pin states only when the pins are configured as outputs.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `start_port` - The starting port (`Port::Port0`, `Port::Port1`, or `Port::Port2`).
    /// * `buffer` - A mutable slice to store the read 8-bit masks. Each mask corresponds
    ///              to a port, starting from `start_port`. A bit value of `1` indicates
    ///              the corresponding pin's output is High, and `0` indicates Low.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the I2C bus operation fails.
    pub async fn get_ports_output_state_ai(
        &mut self,
        start_port: Port,
        buffer: &mut [u8],
    ) -> Result<(), Error<I2C::Error>> {
        let start_register = match start_port {
            Port::Port0 => registers::Register::OutputPort0,
            Port::Port1 => registers::Register::OutputPort1,
            Port::Port2 => registers::Register::OutputPort2,
        };
        self.read_registers_ai(start_register, buffer).await
    }

    /// Gets the current physical state masks for multiple consecutive ports using the auto-increment feature.
    ///
    /// This method reads from the input registers for the specified ports,
    /// starting from `start_port`. The number of ports read is determined by the
    /// length of the provided `buffer`.
    ///
    /// Note: This reads the Input Port registers, which reflect the actual
    /// physical state of the pins, regardless of their configuration (input or output).
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `start_port` - The starting port (`Port::Port0`, `Port::Port1`, or `Port::Port2`).
    /// * `buffer` - A mutable slice to store the read 8-bit masks. Each mask corresponds
    ///              to a port, starting from `start_port`. A bit value of `1` indicates
    ///              the corresponding pin is High, and `0` indicates Low.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the I2C bus operation fails.
    pub async fn get_ports_input_state_ai(
        &mut self,
        start_port: Port,
        buffer: &mut [u8],
    ) -> Result<(), Error<I2C::Error>> {
        let start_register = match start_port {
            Port::Port0 => registers::Register::InputPort0,
            Port::Port1 => registers::Register::InputPort1,
            Port::Port2 => registers::Register::InputPort2,
        };
        self.read_registers_ai(start_register, buffer).await
    }

    /// Sets the polarity inversion state for multiple consecutive ports using the auto-increment feature.
    ///
    /// This method writes to the polarity inversion registers for the specified ports,
    /// starting from `start_port`. The number of ports affected is determined by
    /// the length of the `inversion_masks` slice.
    ///
    /// If inversion is enabled (the corresponding bit in the Polarity Inversion register is 1),
    /// the input value from the Input Port register is inverted before being read.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `start_port` - The starting port (`Port::Port0`, `Port::Port1`, or `Port::Port2`).
    /// * `inversion_masks` - A slice of 8-bit masks. Each mask corresponds to a port,
    ///                       starting from `start_port`. A bit value of `1` enables
    ///                       polarity inversion for the corresponding pin, and `0` disables it.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the I2C bus operation fails.
    pub async fn set_ports_polarity_inversion_ai(
        &mut self,
        start_port: Port,
        inversion_masks: &[u8],
    ) -> Result<(), Error<I2C::Error>> {
        let start_register = match start_port {
            Port::Port0 => registers::Register::PolarityInversionPort0,
            Port::Port1 => registers::Register::PolarityInversionPort1,
            Port::Port2 => registers::Register::PolarityInversionPort2,
        };
        self.write_registers_ai(start_register, inversion_masks)
            .await
    }

    /// Gets the current polarity inversion state masks for multiple consecutive ports using the auto-increment feature.
    ///
    /// This method reads from the polarity inversion registers for the specified ports,
    /// starting from `start_port`. The number of ports read is determined by the
    /// length of the provided `buffer`.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `start_port` - The starting port (`Port::Port0`, `Port::Port1`, or `Port::Port2`).
    /// * `buffer` - A mutable slice to store the read 8-bit masks. Each mask corresponds
    ///              to a port, starting from `start_port`. A bit value of `1` indicates
    ///              polarity inversion is enabled for the corresponding pin, and `0` indicates disabled.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the I2C bus operation fails.
    pub async fn get_ports_polarity_inversion_ai(
        &mut self,
        start_port: Port,
        buffer: &mut [u8],
    ) -> Result<(), Error<I2C::Error>> {
        let start_register = match start_port {
            Port::Port0 => registers::Register::PolarityInversionPort0,
            Port::Port1 => registers::Register::PolarityInversionPort1,
            Port::Port2 => registers::Register::PolarityInversionPort2,
        };
        self.read_registers_ai(start_register, buffer).await
    }
    /// Sets the interrupt mask state for a single pin.
    ///
    /// When a pin is configured as an input, its corresponding interrupt mask bit
    /// can be set to `1` to mask (disable) the interrupt, or `0` to enable it.
    ///
    /// This method reads the current interrupt mask register for the pin's port,
    /// modifies the bit corresponding to the pin, and writes the value back.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `pin` - The target pin (P00-P27).
    /// * `mask` - `true` to mask (disable) the interrupt, `false` to enable.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if an I2C bus operation fails or
    /// if an invalid pin is provided.
    pub async fn set_pin_interrupt_mask(
        &mut self,
        pin: Pin,
        mask: bool,
    ) -> Result<(), Error<I2C::Error>> {
        let pin_index = pin as u8;
        let port_index = pin_index / 8;
        let bit_index = pin_index % 8;
        let mask_register = match port_index {
            0 => registers::Register::InterruptMaskPort0,
            1 => registers::Register::InterruptMaskPort1,
            2 => registers::Register::InterruptMaskPort2,
            _ => return Err(Error::InvalidRegisterOrPin), // Should not happen with valid Pin enum
        };
        let mut mask_value = self.read_register(mask_register).await?;
        if mask {
            mask_value |= 1 << bit_index; // Set bit to 1 (Mask/Disable Interrupt)
        } else {
            mask_value &= !(1 << bit_index); // Clear bit to 0 (Enable Interrupt)
        }
        self.write_register(mask_register, mask_value).await
    }

    /// Gets the current interrupt mask state for a single pin.
    ///
    /// This method reads the interrupt mask register for the pin's port and
    /// extracts the bit corresponding to the pin.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `pin` - The target pin (P00-P27).
    ///
    /// # Returns
    ///
    /// Returns `Ok(bool)` where `true` indicates the interrupt is masked (disabled), `false` otherwise,
    /// or an `Error` if an I2C bus operation fails or if an invalid pin is provided.
    pub async fn get_pin_interrupt_mask(&mut self, pin: Pin) -> Result<bool, Error<I2C::Error>> {
        let pin_index = pin as u8;
        let port_index = pin_index / 8;
        let bit_index = pin_index % 8;
        let mask_register = match port_index {
            0 => registers::Register::InterruptMaskPort0,
            1 => registers::Register::InterruptMaskPort1,
            2 => registers::Register::InterruptMaskPort2,
            _ => return Err(Error::InvalidRegisterOrPin), // Should not happen with valid Pin enum
        };
        let mask_value = self.read_register(mask_register).await?;
        Ok(((mask_value >> bit_index) & 1) == 1)
    }

    /// Sets the interrupt mask state for all 8 pins on a specific port simultaneously.
    ///
    /// This method writes directly to the interrupt mask register for the specified port.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `port` - The target port (`Port::Port0`, `Port::Port1`, or `Port::Port2`).
    /// * `mask_value` - An 8-bit mask where each bit corresponds to a pin on the port.
    ///                  A bit value of `1` masks (disables) the interrupt for the corresponding pin,
    ///                  and `0` enables it.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the I2C bus operation fails.
    pub async fn set_port_interrupt_mask(
        &mut self,
        port: Port,
        mask_value: u8,
    ) -> Result<(), Error<I2C::Error>> {
        let mask_register = match port {
            Port::Port0 => registers::Register::InterruptMaskPort0,
            Port::Port1 => registers::Register::InterruptMaskPort1,
            Port::Port2 => registers::Register::InterruptMaskPort2,
        };
        self.write_register(mask_register, mask_value).await
    }

    /// Gets the current interrupt mask state mask for a specific port.
    ///
    /// This method reads the interrupt mask register for the specified port.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `port` - The target port (`Port::Port0`, `Port::Port1`, or `Port::Port2`).
    ///
    /// # Returns
    ///
    /// Returns `Ok(u8)` containing an 8-bit mask on success, where each bit corresponds
    /// to a pin on the port (`1` = Masked/Disabled, `0` = Enabled), or an `Error` if the I2C
    /// bus operation fails.
    pub async fn get_port_interrupt_mask(&mut self, port: Port) -> Result<u8, Error<I2C::Error>> {
        let mask_register = match port {
            Port::Port0 => registers::Register::InterruptMaskPort0,
            Port::Port1 => registers::Register::InterruptMaskPort1,
            Port::Port2 => registers::Register::InterruptMaskPort2,
        };
        self.read_register(mask_register).await
    }

    /// Sets the interrupt mask state for multiple consecutive ports using the auto-increment feature.
    ///
    /// This method writes to the interrupt mask registers for the specified ports,
    /// starting from `start_port`. The number of ports affected is determined by
    /// the length of the `mask_masks` slice.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `start_port` - The starting port (`Port::Port0`, `Port::Port1`, or `Port::Port2`).
    /// * `mask_masks` - A slice of 8-bit masks. Each mask corresponds to a port,
    ///                  starting from `start_port`. A bit value of `1` masks (disables)
    ///                  the interrupt for the corresponding pin, and `0` enables it.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the I2C bus operation fails.
    pub async fn set_ports_interrupt_mask_ai(
        &mut self,
        start_port: Port,
        mask_masks: &[u8],
    ) -> Result<(), Error<I2C::Error>> {
        let start_register = match start_port {
            Port::Port0 => registers::Register::InterruptMaskPort0,
            Port::Port1 => registers::Register::InterruptMaskPort1,
            Port::Port2 => registers::Register::InterruptMaskPort2,
        };
        self.write_registers_ai(start_register, mask_masks).await
    }

    /// Gets the current interrupt mask state masks for multiple consecutive ports using the auto-increment feature.
    ///
    /// This method reads from the interrupt mask registers for the specified ports,
    /// starting from `start_port`. The number of ports read is determined by the
    /// length of the provided `buffer`.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `start_port` - The starting port (`Port::Port0`, `Port::Port1`, or `Port::Port2`).
    /// * `buffer` - A mutable slice to store the read 8-bit masks. Each mask corresponds
    ///              to a port, starting from `start_port`. A bit value of `1` indicates
    ///              the interrupt is masked (disabled) for the corresponding pin, and `0` indicates enabled.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the I2C bus operation fails.
    pub async fn get_ports_interrupt_mask_ai(
        &mut self,
        start_port: Port,
        buffer: &mut [u8],
    ) -> Result<(), Error<I2C::Error>> {
        let start_register = match start_port {
            Port::Port0 => registers::Register::InterruptMaskPort0,
            Port::Port1 => registers::Register::InterruptMaskPort1,
            Port::Port2 => registers::Register::InterruptMaskPort2,
        };
        self.read_registers_ai(start_register, buffer).await
    }
    /// Sets the initial output state for all three ports (Port0, Port1, Port2).
    ///
    /// This method writes the provided masks to the Output Port Registers (0x04, 0x05, 0x06)
    /// using the auto-increment feature, starting from Output Port 0.
    ///
    /// This is useful for configuring the power-up default state of the output pins.
    ///
    /// This method is `async` when the `async` feature is enabled, and synchronous otherwise.
    ///
    /// # Arguments
    ///
    /// * `port0_mask` - The 8-bit output mask for Port 0.
    /// * `port1_mask` - The 8-bit output mask for Port 1.
    /// * `port2_mask` - The 8-bit output mask for Port 2.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if the I2C bus operation fails.
    pub async fn set_initial_output_state(
        &mut self,
        port0_mask: u8,
        port1_mask: u8,
        port2_mask: u8,
    ) -> Result<(), Error<I2C::Error>> {
        let masks = [port0_mask, port1_mask, port2_mask];
        self.write_registers_ai(registers::Register::OutputPort0, &masks)
            .await
    }
}

// TODO: Add mock-based tests using embedded-hal-mock (in tests/integration_test.rs)
// TODO: Add tests for register access, pin control, etc.
// TODO: Implement Output Port Configuration register methods (power-up default) - DONE
// TODO: Implement Interrupt Mask register methods - DONE
