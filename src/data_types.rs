//! TCA6424 数据类型定义

/// 引脚方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinDirection {
    /// 输入 (对应配置寄存器中的 1)
    Input,
    /// 输出 (对应配置寄存器中的 0)
    Output,
}

/// 引脚状态 (用于输入和输出)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinState {
    /// 低电平
    Low,
    /// 高电平
    High,
}

/// TCA6424 引脚定义 (P00-P27)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Pin {
    P00 = 0, P01 = 1, P02 = 2, P03 = 3, P04 = 4, P05 = 5, P06 = 6, P07 = 7,
    P10 = 8, P11 = 9, P12 = 10, P13 = 11, P14 = 12, P15 = 13, P16 = 14, P17 = 15,
    P20 = 16, P21 = 17, P22 = 18, P23 = 19, P24 = 20, P25 = 21, P26 = 22, P27 = 23,
}

// TODO: Add more data types if needed, e.g., for port-level operations