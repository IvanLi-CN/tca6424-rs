//! TCA6424 驱动库错误类型

use core::fmt::Debug;

/// TCA6424 驱动库错误枚举
#[derive(Debug)]
pub enum Error<I2cError: Debug> {
    /// 底层 I2C 总线错误
    I2c(I2cError),
    /// 尝试访问保留寄存器或无效引脚
    InvalidRegisterOrPin,
    // TODO: Add more specific error types as needed
}

// TODO: Implement From trait for I2cError if possible
// impl<I2cError: Debug> From<I2cError> for Error<I2cError> {
//     fn from(err: I2cError) -> Self {
//         Error::I2c(err)
//     }
// }