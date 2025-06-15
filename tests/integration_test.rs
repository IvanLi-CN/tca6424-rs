use embedded_hal_mock::eh1::i2c::Mock as I2cMock; // Removed unused Transaction
// use embedded_hal_mock::eh1::MockError; // Removed unused MockError

// Note: embedded-hal-mock::eh1 does not directly support async traits from embedded-hal-async.
// For async tests, a different mock approach or a dedicated async mock crate might be needed.
// Temporarily skipping async test or using a basic mock that compiles.
// For now, let's keep the structure but acknowledge the limitation.
// A dedicated async mock crate like `embedded-hal-async-mock` might be necessary for full async testing.

#[cfg(not(feature = "async"))] // Sync test
#[test]
fn test_new_sync() {
    let expectations = []; // No I2C transactions expected for new()
    let mut i2c_mock = I2cMock::new(&expectations); // Use I2cMock
    let address = 0x22;

    let _tca = tca6424::Tca6424::new(&mut i2c_mock, address).unwrap(); // Pass mutable reference

    i2c_mock.done(); // Check that all expectations were met
}
#[cfg(not(feature = "async"))] // Sync test
#[test]
fn test_set_pin_direction_sync() {
    use embedded_hal_mock::eh1::i2c::Transaction as I2cTransaction; // Import Transaction

    let address = 0x22;
    let initial_config_port0 = 0xFF; // Assume all pins are initially inputs
    let initial_config_port1 = 0xFF; // Assume all pins are initially inputs

    let expectations = [
        // Set P00 to Output (clear bit 0 in Config Port 0)
        I2cTransaction::write_read(address, vec![0x0C], vec![initial_config_port0]).into(), // Read Config Port 0
        I2cTransaction::write(address, vec![0x0C, initial_config_port0 & !(1 << 0)]).into(), // Write Config Port 0 with bit 0 cleared

        // Set P17 to Input (set bit 7 in Config Port 1)
        I2cTransaction::write_read(address, vec![0x0D], vec![initial_config_port1]).into(), // Read Config Port 1
        I2cTransaction::write(address, vec![0x0D, initial_config_port1 | (1 << 7)]).into(), // Write Config Port 1 with bit 7 set
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).unwrap();

    // Perform the operations
    tca.set_pin_direction(tca6424::Pin::P00, tca6424::PinDirection::Output).unwrap();
    tca.set_pin_direction(tca6424::Pin::P17, tca6424::PinDirection::Input).unwrap();

    i2c_mock.done(); // Check that all expectations were met
}


// TODO: Add mock-based tests using embedded-hal-mock
// TODO: Add tests for register access, pin control, etc.
