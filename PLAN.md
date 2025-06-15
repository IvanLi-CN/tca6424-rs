# TCA6424 Rust 驱动库高层设计与规划

## 1. 项目简介

本项目旨在为 Texas Instruments TCA6424 低压 24 位 I2C I/O 扩展器提供一个 Rust 驱动库。该库将基于 `embedded-hal` 特性，并通过 `maybe-async-cfg` crate 提供同步 (sync) 和异步 (async) 两种操作模式的支持，以便在不同的嵌入式环境中使用。

驱动库的设计将参考现有的 `bq25730-rs` 项目，借鉴其模块划分、代码风格和 `maybe-async-cfg` 的使用方式。功能实现将严格遵循 TCA6424 数据手册 ([`docs/tca6424.md`](docs/tca6424.md)) 的描述。

## 2. 默认设备地址

根据数据手册 [`docs/tca6424.md`](docs/tca6424.md) 的 Table 3 ([`docs/tca6424.md:373`](docs/tca6424.md:373))，当 ADDR 引脚连接到 GND (逻辑低) 时，TCA6424 的 7 位 I2C 从设备地址为 `0x22` (十六进制) 或 34 (十进制)。这将作为驱动库的默认设备地址。

## 3. 项目结构

参考 `bq25730-rs` 项目结构，建议采用以下模块划分：

```
tca6424-rs/
├── Cargo.toml          # 项目依赖和特性配置
├── README.md           # 项目说明和使用示例
├── src/
│   ├── lib.rs          # 库的入口，包含主要的 Tca6424 结构体和公共 API
│   ├── registers.rs    # TCA6424 寄存器地址、位定义和相关常量
│   ├── data_types.rs   # 定义引脚状态、方向等相关枚举和结构体
│   └── errors.rs       # 定义自定义错误类型
└── docs/
    └── tca6424.md      # 芯片数据手册 (已提供)
```

## 4. 依赖项

核心依赖项将包括：

-   `embedded-hal`: 提供 I2C 总线 trait。
-   `embedded-hal-async`: 提供异步 I2C 总线 trait (通过 `async` feature 启用)。
-   `maybe-async-cfg`: 实现同步/异步代码抽象。
-   `bitflags`: 用于方便地定义和操作寄存器中的位字段 (例如配置寄存器)。
-   `defmt` (可选): 用于嵌入式环境的日志输出 (通过 `defmt` feature 启用)。

`Cargo.toml` 中 `[features]` 部分将包含 `async` 和 `defmt` 等，类似于 `bq25730-rs/Cargo.toml` ([`bq25730-rs/Cargo.toml:46`](bq25730-rs/Cargo.toml:46))。

## 5. 高层设计

-   **核心结构体**: 定义一个 `Tca6424` 结构体，它将持有实现了 `embedded-hal` 或 `embedded-hal-async` 中 I2C trait 的 I2C 总线实例，以及设备的 I2C 地址。
    ```rust
    #[maybe_async_cfg::maybe(sync)]
    pub struct Tca6424<I2C> {
        i2c: I2C,
        address: u8,
    }
    ```
-   **同步/异步抽象**: 使用 `#[maybe_async_cfg::maybe(...)]` 属性标记 `impl` 块或 trait。块内的公共方法将定义为 `async fn`。`maybe-async-cfg` 宏将处理在同步模式下将这些方法转换为非 `async` 函数。内部的 I2C 操作将调用 I2C trait 中相应的 `read`, `write`, `write_read` 或 `transaction` 方法。
-   **寄存器访问**: 实现内部的私有方法来处理 TCA6424 的寄存器读写操作。这些方法需要处理发送命令字节来选择目标寄存器，并考虑数据手册中描述的 Auto-Increment (AI) 特性 ([`docs/tca6424.md:381`](docs/tca6424.md:381))。
    -   写操作: 发送设备地址 (写模式)，命令字节，然后是数据字节。如果 AI 启用，后续数据字节将写入同一组的下一个寄存器。
    -   读操作: 发送设备地址 (写模式)，命令字节，然后发送 Restart 条件和设备地址 (读模式)。设备将返回指定寄存器的数据。如果 AI 启用，后续读操作将返回同一组的下一个寄存器的数据。
-   **引脚控制**: 提供高级方法来控制 24 个 I/O 引脚 (P00-P27)。这些方法将通过内部寄存器访问方法与芯片交互：
    -   配置引脚方向 (输入/输出): 读写 Configuration Registers (寄存器 12, 13, 14)。
    -   设置输出引脚状态: 读写 Output Port Registers (寄存器 4, 5, 6)。
    -   读取输入引脚状态: 读取 Input Port Registers (寄存器 0, 1, 2)。
    -   配置输入引脚极性反转: 读写 Polarity Inversion Registers (寄存器 8, 9, 10)。
-   **错误处理**: 定义一个自定义的错误枚举 (`errors.rs`) 来封装底层的 I2C 错误以及可能的芯片特定错误 (例如，尝试访问保留寄存器)。
-   **中断和复位**: 考虑为中断输出 (`INT`) 和复位输入 (`RESET`) 引脚提供支持。中断输出是开漏的，需要外部上拉电阻。RESET 引脚是低电平有效的。

## 6. 关键功能实现计划

1.  在当前工作区初始化 Rust 库结构。
2.  配置 `Cargo.toml`，添加必要的依赖项和 features。
3.  在 `src/registers.rs` 中定义 TCA6424 的寄存器地址常量和位定义 (使用 `bitflags`)。
4.  在 `src/data_types.rs` 中定义表示引脚方向 (`PinDirection`)、引脚状态 (`PinState`) 等的枚举。
5.  在 `src/errors.rs` 中定义自定义错误类型，包含 I2C 错误。
6.  在 `src/lib.rs` 中定义 `Tca6424` 结构体和 `new` 方法。
7.  实现处理命令字节和 Auto-Increment 的内部寄存器读写方法。
8.  实现配置引脚方向的公共方法 (`set_direction`, `get_direction`)。
9.  实现控制输出引脚的公共方法 (`set_output`, `get_output_state`)。
10. 实现读取输入引脚的公共方法 (`get_input`)。
11. 实现配置极性反转的公共方法 (`set_polarity_inversion`, `get_polarity_inversion`)。
12. 为包含公共方法的 `impl` 块添加 `#[maybe_async_cfg::maybe(sync)]` 属性。
13. 编写单元测试和集成测试，使用 `embedded-hal-mock` 进行模拟测试。
14. 编写使用同步和异步 I2C 实例的示例代码。
15. 更新 `README.md`，提供使用说明和示例。
16. 完善代码注释和文档。

## 7. 潜在的未来增强

-   为中断输出提供更高级的抽象，例如事件驱动或轮询机制。
-   提供控制 RESET 引脚的方法 (如果硬件连接允许)。
-   实现对 ADDR 引脚的软件配置支持 (如果需要动态改变地址)。