# TCA6424 Rust 驱动库

这是一个用于 Texas Instruments TCA6424 低压 24 位 I2C I/O 扩展器的 Rust 驱动库。它基于 `embedded-hal` 特性，并通过 `maybe-async-cfg` crate 提供同步 (sync) 和异步 (async) 两种操作模式的支持。

## 功能

- 支持 `embedded-hal` 和 `embedded-hal-async` I2C traits。
- 通过 `maybe-async-cfg` 实现同步/异步抽象。
- 提供引脚级别的控制方法：
    - 设置/获取引脚方向 (输入/输出)
    - 设置/获取输出引脚状态 (高/低)
    - 获取输入引脚物理状态 (高/低)
    - 设置/获取引脚极性反转
- 提供端口级别的控制方法：
    - 设置/获取端口方向掩码
    - 设置/获取输出端口状态掩码
    - 获取输入端口物理状态掩码
    - 设置/获取端口极性反转掩码
- 支持寄存器自动递增 (Auto-Increment) 读写操作，用于高效地操作连续寄存器。

## 兼容性

本驱动库设计用于 `no_std` 环境，适用于各种嵌入式系统。通过 `async` feature，可以在支持 `embedded-hal-async` 的异步运行时环境中使用。

## 使用

将 `tca6424` 添加到您的 `Cargo.toml` 文件的 `[dependencies]` 部分：

```toml
tca6424 = "0.1.0" # 或者指向本地路径 { path = "path/to/tca6424-rs" }
```

如果您需要异步支持，请启用 `async` feature：

```toml
tca6424 = { version = "0.1.0", features = ["async"] }
```

如果您使用 `defmt` 进行日志输出，请启用 `defmt` feature：

```toml
tca6424 = { version = "0.1.0", features = ["defmt"] }
```

### 基本用法 (同步示例)

```rust
use embedded_hal::i2c::I2c;
use tca6424::{Pin, PinDirection, PinState, Tca6424, Error};

// 假设您有一个实现了 embedded_hal::i2c::I2c trait 的 I2C 总线实例
// let mut i2c_bus = ...;
// let address = 0x22; // TCA6424 默认 I2C 地址
// let mut tca = Tca6424::new(&mut i2c_bus, address).unwrap();

// // 设置 P00 为输出，并将其设置为高电平
// tca.set_pin_direction(Pin::P00, PinDirection::Output).unwrap();
// tca.set_pin_output(Pin::P00, PinState::High).unwrap();

// // 读取 P01 的输入状态
// let input_state = tca.get_pin_input_state(Pin::P01).unwrap();
// println!("P01 input state: {:?}", input_state);

// // 设置 Port0 的所有引脚方向 (例如：P00-P03 输出, P04-P07 输入)
// let port0_direction_mask = 0b1111_0000;
// tca.set_port_direction(Port::Port0, port0_direction_mask).unwrap();

// // 使用自动递增读取 Port0, Port1, Port2 的输入状态
// let mut input_buffer = [0u8; 3];
// tca.get_ports_input_state_ai(Port::Port0, &mut input_buffer).unwrap();
// println!("Port0-Port2 input states: {:?}", input_buffer);
```

### 示例代码

您可以在 `examples/` 目录中找到更完整的示例：

-   [`examples/sync_example.rs`](examples/sync_example.rs): 使用模拟 I2C 总线展示同步 API 的用法。
-   [`examples/stm32g4/`](examples/stm32g4/): 一个使用 Embassy 框架在 STM32G4 微控制器上运行的异步示例。

要运行同步示例：

```bash
cargo run --example sync_example
```

要构建 STM32G4 示例 (需要安装 `thumbv7em-none-eabihf` 目标和 `rust-src` 组件)：

```bash
cd examples/stm32g4
cargo build
```

要运行 STM32G4 示例 (需要 probe-rs 和兼容的调试探针)：

```bash
cd examples/stm32g4
cargo run # 使用 .cargo/config.toml 中配置的 runner
```

## 数据手册

TCA6424 的详细信息请参考数据手册：[`docs/tca6424.md`](docs/tca6424.md)

## 许可证

本项目根据 MIT 或 Apache-2.0 许可证双重授权。详情请参阅 [LICENSE-MIT](LICENSE-MIT) 和 [LICENSE-APACHE](LICENSE-APACHE) 文件。