//! TCA6424 data type definitions.

/// Represents the direction of a TCA6424 pin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinDirection {
    /// Input direction (corresponds to a '1' in the Configuration register).
    Input,
    /// Output direction (corresponds to a '0' in the Configuration register).
    Output,
}

/// Represents the state of a TCA6424 pin (High or Low).
/// Used for both input and output operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PinState {
    /// Low state.
    Low,
    /// High state.
    High,
}

/// Defines the individual pins of the TCA6424 I/O expander (P00-P27).
///
/// Pins are grouped into three 8-bit ports: Port 0 (P00-P07), Port 1 (P10-P17),
/// and Port 2 (P20-P27).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin {
    /// Port 0, Pin 0
    P00 = 0, /// Port 0, Pin 1
    P01 = 1, /// Port 0, Pin 2
    P02 = 2, /// Port 0, Pin 3
    P03 = 3, /// Port 0, Pin 4
    P04 = 4, /// Port 0, Pin 5
    P05 = 5, /// Port 0, Pin 6
    P06 = 6, /// Port 0, Pin 7
    P07 = 7,
    /// Port 1, Pin 0
    P10 = 8, /// Port 1, Pin 1
    P11 = 9, /// Port 1, Pin 2
    P12 = 10, /// Port 1, Pin 3
    P13 = 11, /// Port 1, Pin 4
    P14 = 12, /// Port 1, Pin 5
    P15 = 13, /// Port 1, Pin 6
    P16 = 14, /// Port 1, Pin 7
    P17 = 15,
    /// Port 2, Pin 0
    P20 = 16, /// Port 2, Pin 1
    P21 = 17, /// Port 2, Pin 2
    P22 = 18, /// Port 2, Pin 3
    P23 = 19, /// Port 2, Pin 4
    P24 = 20, /// Port 2, Pin 5
    P25 = 21, /// Port 2, Pin 6
    P26 = 22, /// Port 2, Pin 7
    P27 = 23,
}

/// Defines the 8-bit ports of the TCA6424 I/O expander.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Port {
    /// Port 0 (Pins P00-P07).
    Port0 = 0,
    /// Port 1 (Pins P10-P17).
    Port1 = 1,
    /// Port 2 (Pins P20-P27).
    Port2 = 2,
}