#![no_std]
#![no_main]
#![allow(unused_imports)] // 允许未使用的导入，以消除警告

use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    i2c::{self, Config, I2c},
    peripherals::{I2C1, PA15, PB7, DMA1_CH5, DMA1_CH6}, // Added specific peripherals
    time::Hertz,
};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _}; // Import panic_probe for panic handling

// Updated import to tca6424
use tca6424::{
    Tca6424,
    Pin, PinDirection, PinState, Port, DEFAULT_ADDRESS, // Import relevant types and default address
};
use tca6424::errors::Error; // Import Error type

// Updated interrupt binding struct name
bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello from STM32G431CB!");

    let mut config = Config::default();
    config.scl_pullup = true;
    config.sda_pullup = true;

    // Use specific peripheral types from embassy_stm32::peripherals
    let mut i2c = I2c::new(
        p.I2C1,
        p.PA15, // SCL
        p.PB7,  // SDA
        Irqs,
        p.DMA1_CH5, // TX DMA channel
        p.DMA1_CH6, // RX DMA channel
        Hertz(100_000), // I2C clock frequency
        config,
    );

    // Instantiate TCA6424 driver and handle the Result
    // TCA6424 address is typically 0x22
    let address = DEFAULT_ADDRESS;
    let mut tca = match Tca6424::new(&mut i2c, address).await { // Correctly handle the async Result
        Ok(driver) => {
            info!("TCA6424 driver instance created successfully.");
            driver
        }
        Err(e) => {
            error!("Failed to create TCA6424 driver instance: {:?}", e);
            // In a real application, you might want to handle this more gracefully
            // For this example, we'll just loop indefinitely or reset
            loop {
                Timer::after(Duration::from_secs(1)).await;
            }
        }
    };


    // --- TCA6424 Example Operations ---
    info!("--- TCA6424 Example Operations ---");

    // Example 1: Set Port0 direction (P00-P03 Output, P04-P07 Input)
    info!("\n--- Example 1: Set Port0 Direction ---");
    let port0_direction_mask = 0b1111_0000;
    match tca.set_port_direction(Port::Port0, port0_direction_mask).await {
        Ok(_) => info!("Port0 direction set to {:08b}.", port0_direction_mask),
        Err(e) => error!("Failed to set Port0 direction: {:?}", e),
    }

    // Example 2: Set Port1 output state (Alternating Low/High)
    info!("\n--- Example 2: Set Port1 Output State ---");
    let port1_output_mask = 0b0101_0101;
    match tca.set_port_output(Port::Port1, port1_output_mask).await {
        Ok(_) => info!("Port1 output set to {:08b}.", port1_output_mask),
        Err(e) => error!("Failed to set Port1 output: {:?}", e),
    }

    // Example 3: Read Port2 input state
    info!("\n--- Example 3: Read Port2 Input State ---");
    match tca.get_port_input_state(Port::Port2).await {
        Ok(input_mask) => info!("Read Port2 input state: {:08b}", input_mask),
        Err(e) => error!("Failed to read Port2 input state: {:?}", e),
    }

    // Example 4: Set Port0 polarity inversion (Alternating Invert/Original)
    info!("\n--- Example 4: Set Port0 Polarity Inversion ---");
    let port0_polarity_mask = 0b1010_1010;
    match tca.set_port_polarity_inversion(Port::Port0, port0_polarity_mask).await {
        Ok(_) => info!("Port0 polarity inversion set to {:08b}.", port0_polarity_mask),
        Err(e) => error!("Failed to set Port0 polarity inversion: {:?}", e),
    }

    // Example 5: Use auto-increment to set Port0, Port1, Port2 directions
    info!("\n--- Example 5: Set Ports Direction (Auto-Increment) ---");
    let all_ports_direction = [0b1111_0000, 0b0000_1111, 0b1010_1010];
    match tca.set_ports_direction_ai(Port::Port0, &all_ports_direction).await {
        Ok(_) => info!("Set Port0-Port2 directions using auto-increment."),
        Err(e) => error!("Failed to set Port0-Port2 directions (AI): {:?}", e),
    }

    // Example 6: Use auto-increment to read Port0, Port1, Port2 input states
    info!("\n--- Example 6: Read Ports Input State (Auto-Increment) ---");
    let mut input_buffer = [0u8; 3];
    match tca.get_ports_input_state_ai(Port::Port0, &mut input_buffer).await {
        Ok(_) => info!("Read Port0-Port2 input states (AI): {:?}", input_buffer),
        Err(e) => error!("Failed to read Port0-Port2 input states (AI): {:?}", e),
    }

    // Example 7: Set a single pin (P00) direction
    info!("\n--- Example 7: Set Single Pin (P00) Direction ---");
    match tca.set_pin_direction(Pin::P00, PinDirection::Output).await {
        Ok(_) => info!("Pin P00 direction set to Output."),
        Err(e) => error!("Failed to set Pin P00 direction: {:?}", e),
    }

    // Example 8: Get a single pin (P00) input state
    info!("\n--- Example 8: Get Single Pin (P00) Input State ---");
    match tca.get_pin_input_state(Pin::P00).await {
        Ok(state) => info!("Pin P00 input state: {:?}", state),
        Err(e) => error!("Failed to get Pin P00 input state: {:?}", e),
    }


    info!("\n--- TCA6424 Example Complete ---");

    // Loop forever
    loop {
        Timer::after(Duration::from_secs(5)).await;
        info!("Heartbeat...");
    }
}
