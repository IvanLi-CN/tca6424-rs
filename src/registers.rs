//! TCA6424 寄存器地址和位定义

use bitflags::bitflags;

/// TCA6424 寄存器地址
#[allow(dead_code)] // 允许在未使用时保留定义
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Register {
    /// Input Port 0
    InputPort0 = 0x00,
    /// Input Port 1
    InputPort1 = 0x01,
    /// Input Port 2
    InputPort2 = 0x02,
    /// Output Port 0
    OutputPort0 = 0x04,
    /// Output Port 1
    OutputPort1 = 0x05,
    /// Output Port 2
    OutputPort2 = 0x06,
    /// Polarity Inversion Port 0
    PolarityInversionPort0 = 0x08,
    /// Polarity Inversion Port 1
    PolarityInversionPort1 = 0x09,
    /// Polarity Inversion Port 2
    PolarityInversionPort2 = 0x0A,
    /// Configuration Port 0
    ConfigurationPort0 = 0x0C,
    /// Configuration Port 1
    ConfigurationPort1 = 0x0D,
    /// Configuration Port 2
    ConfigurationPort2 = 0x0E,
    /// Interrupt Mask Port 0
    InterruptMaskPort0 = 0x10,
    /// Interrupt Mask Port 1
    InterruptMaskPort1 = 0x11,
    /// Interrupt Mask Port 2
    InterruptMaskPort2 = 0x12,
}

bitflags! {
    /// Configuration register bits (Input=1, Output=0)
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ConfigurationFlags: u8 {
        const Px0 = 0b0000_0001;
        const Px1 = 0b0000_0010;
        const Px2 = 0b0000_0100;
        const Px3 = 0b0000_1000;
        const Px4 = 0b0001_0000;
        const Px5 = 0b0010_0000;
        const Px6 = 0b0100_0000;
        const Px7 = 0b1000_0000;
    }
}

bitflags! {
    /// Polarity Inversion register bits (Inverted=1, Original=0)
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PolarityInversionFlags: u8 {
        const Px0 = 0b0000_0001;
        const Px1 = 0b0000_0010;
        const Px2 = 0b0000_0100;
        const Px3 = 0b0000_1000;
        const Px4 = 0b0001_0000;
        const Px5 = 0b0010_0000;
        const Px6 = 0b0100_0000;
        const Px7 = 0b1000_0000;
    }
}

// Input and Output registers directly represent pin state (0 or 1),
// so bitflags are not needed. We can use u8 or define a specific type
// in data_types.rs if needed for clarity.