//! TCA6424 driver library error types.

use core::fmt::Debug;
#[cfg(feature = "defmt")]
use defmt;

/// Represents possible errors that can occur when interacting with the TCA6424 driver.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<I2cError: Debug> {
    /// An error occurred during an underlying I2C bus operation.
    I2c(I2cError),
    /// An attempt was made to access a reserved register address or an invalid pin.
    InvalidRegisterOrPin,
    // TODO: Add more specific error types as needed, e.g., for invalid arguments
}

// TODO: Implement From trait for I2cError if possible
// impl<I2cError: Debug> From<I2cError> for Error<I2cError> {
//     fn from(err: I2cError) -> Self {
//         Error::I2c(err)
//     }
// }