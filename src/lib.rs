//! Texas Instruments TCA6424 24-bit I2C I/O 扩展器驱动库

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "defmt", feature(defmt))]

#[cfg(feature = "defmt")]
use defmt;

mod data_types;
mod errors;
mod registers;

pub use data_types::*;
pub use errors::Error;

/// TCA6424 驱动结构体
pub struct Tca6424<'a, I2C> {
    // Added lifetime parameter 'a
    i2c: &'a mut I2C, // Changed field type to mutable reference with lifetime
    address: u8,
}

impl<'a, I2C> Tca6424<'a, I2C>
// Added lifetime parameter 'a
where
    I2C: embedded_hal::i2c::I2c,
    I2C::Error: core::fmt::Debug, // Ensure I2C error is Debug
{
    /// 创建一个新的 TCA6424 驱动实例
    ///
    /// `i2c`: 实现了 `embedded-hal` 或 `embedded-hal-async` I2C trait 的总线实例
    /// `address`: TCA6424 的 I2C 从设备地址
    pub fn new(i2c: &'a mut I2C, address: u8) -> Result<Self, Error<I2C::Error>> {
        // Added lifetime to parameter
        // TODO: Add initialization logic if needed
        Ok(Self { i2c: i2c, address }) // Explicitly assign parameter to field
    }
    /// 向指定的寄存器写入一个字节
    ///
    /// 这是内部辅助函数，不处理自动递增 (Auto-Increment)。
    fn write_register(
        &mut self,
        register: registers::Register,
        value: u8,
    ) -> Result<(), Error<I2C::Error>> {
        let mut buffer = [register as u8, value];
        self.i2c
            .write(self.address, &mut buffer) // Use &mut buffer for write
            .map_err(Error::I2c)
    }
    /// 从指定的寄存器读取一个字节
    ///
    /// 这是内部辅助函数，不处理自动递增 (Auto-Increment)。
    fn read_register(&mut self, register: registers::Register) -> Result<u8, Error<I2C::Error>> {
        let mut read_buffer = [0u8];
        self.i2c
            .write_read(self.address, &[register as u8], &mut read_buffer)
            .map_err(Error::I2c)?;
        Ok(read_buffer[0])
    }

    // TODO: Implement internal register read/write methods (handling AI)
    // TODO: Implement public methods for pin control (set_direction, set_output, etc.)
    /// 设置指定引脚的方向 (输入或输出)
    ///
    /// `pin`: 要设置的引脚 (P00-P27)
    /// `direction`: 引脚方向 (Input 或 Output)
    pub fn set_pin_direction(
        &mut self,
        pin: Pin,
        direction: PinDirection,
    ) -> Result<(), Error<I2C::Error>> {
        let pin_index = pin as u8;
        let port_index = pin_index / 8;
        let bit_index = pin_index % 8;

        // 确定配置寄存器地址
        let config_register = match port_index {
            0 => registers::Register::ConfigurationPort0,
            1 => registers::Register::ConfigurationPort1,
            2 => registers::Register::ConfigurationPort2,
            _ => return Err(Error::InvalidRegisterOrPin), // Should not happen with valid Pin enum
        };

        // 读取当前配置寄存器的值
        let mut config_value = self.read_register(config_register)?;

        // 根据方向设置或清除对应的位
        // TCA6424 配置寄存器: 1 = Input, 0 = Output
        match direction {
            PinDirection::Input => {
                config_value |= 1 << bit_index; // 设置位为 1 (输入)
            }
            PinDirection::Output => {
                config_value &= !(1 << bit_index); // 清除位为 0 (输出)
            }
        }

        // 将修改后的值写回配置寄存器
        self.write_register(config_register, config_value)
    }
}

// TODO: Add mock-based tests using embedded-hal-mock (in tests/integration_test.rs)
// TODO: Add tests for register access, pin control, etc.
