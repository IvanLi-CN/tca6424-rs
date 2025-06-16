# BQ25730 Charger Example for STM32G4

This example demonstrates how to configure and interact with the BQ25730 battery charger using an STM32G4 microcontroller.

## Configuration

The following parameters are configured in this example:

* **I2C Address:** 0x6B
* **Charge Current:** 512 mA (Note: The BQ25730 has a 128mA LSB for charge current with 5mΩ sense resistor, so 500mA is rounded up to 512mA).
* **Charge Voltage:** 18000 mV (Configured for 5-cell LiFePO4 battery, 3.6V per cell).
* **VBUS Sense Resistor (RAC):** 10 mΩ
* **VBAT Sense Resistor (RSR):** 5 mΩ
* **Ship Mode:** Disabled
* **ADC Measurements:** All ADC channels are enabled for continuous conversion, and their values are periodically read and printed.

## Usage

To run this example, ensure you have the necessary Rust toolchain and `embassy-stm32` dependencies installed.

1. **Build the project:**

    ```bash
    cargo build --example stm32g4
    ```

2. **Flash to your STM32G4 board:**
    (Instructions for flashing will depend on your specific setup, e.g., using `probe-rs` or STM32CubeProgrammer)

    ```bash
    # Example with probe-rs (adjust target and chip as needed)
    cargo flash --example stm32g4 --chip STM32G431CBUx --probe-runner stm32cubeprogrammer
    ```

3. **Monitor serial output:**
    Use a serial terminal (e.g., `minicom`, `screen`, or VS Code's serial monitor) to view the `defmt` logs.

4. **Reset and Attach to MCU:**

    ```bash
    probe-rs reset --chip STM32G431CBUx
    probe-rs attach --chip STM32G431CBUx target/thumbv7em-none-eabihf/debug/bq25730_stm32g431cbu6_example
    ```

## Important Notes

* Ensure your hardware connections for I2C (SCL, SDA) and the BQ25730 are correct.
* The charge current is rounded up to the nearest valid LSB value.
* This example assumes a 5-cell LiFePO4 battery configuration. Adjust `ChargeVoltage` if using a different battery chemistry or cell count.
* The `enter_ship_mode()` function call has been removed from this example to prevent unintended entry into ship mode.
