use embedded_hal_mock::eh1::i2c::Mock as I2cMock;
use embedded_hal_mock::eh1::i2c::Transaction as I2cTransaction;
use tca6424::Port;
// use embedded_hal_mock::eh1::MockError; // Removed unused MockError

// Note: embedded-hal-mock::eh1 does not directly support async traits from embedded-hal-async.
// For async tests, a different mock approach or a dedicated async mock crate might be needed.
// Temporarily skipping async test or using a basic mock that compiles.
// For now, let's keep the structure but acknowledge the limitation.
// A dedicated async mock crate like `embedded-hal-async-mock` might be necessary for full async testing.

#[cfg(not(feature = "async"))]
#[test] // Use standard test attribute for explicit sync test
fn test_new_sync() {
    let expectations = []; // No I2C transactions expected for new()
    let mut i2c_mock = I2cMock::new(&expectations); // Use I2cMock
    let address = 0x22;

    let _tca = tca6424::Tca6424::new(&mut i2c_mock, address).unwrap(); // No .await in sync

    i2c_mock.done(); // Check that all expectations were met
}

#[cfg(feature = "async")]
#[tokio::test] // Use tokio test attribute for explicit async test
async fn test_new_async() { // Renamed for clarity
    let expectations = []; // No I2C transactions expected for new()
    let mut i2c_mock = I2cMock::new(&expectations); // Use I2cMock
    let address = 0x22;

    let _tca = tca6424::Tca6424::new(&mut i2c_mock, address).await.unwrap(); // Keep .await in async

    i2c_mock.done(); // Check that all expectations were met
}

#[cfg(not(feature = "async"))]
#[test]
fn test_set_pin_direction_sync() {
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
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).unwrap(); // Removed .await

    // Perform the operations
    tca.set_pin_direction(tca6424::Pin::P00, tca6424::PinDirection::Output).unwrap(); // Removed .await
    tca.set_pin_direction(tca6424::Pin::P17, tca6424::PinDirection::Input).unwrap(); // Removed .await

    i2c_mock.done(); // Check that all expectations were met
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_set_pin_direction_async() {
    let address = 0x22;
    let initial_config_port0 = 0xFF; // Assume all pins are initially inputs
    let initial_config_port1 = 0xFF; // Assume all pins are initially inputs

    let expectations = [
        // Set P00 to Output (clear bit 0 in Config Port 0)
        I2cTransaction::write_read(address, vec![0x0C], vec![initial_config_port0]), // Read Config Port 0
        I2cTransaction::write(address, vec![0x0C, initial_config_port0 & !(1 << 0)]), // Write Config Port 0 with bit 0 cleared
        // Set P17 to Input (set bit 7 in Config Port 1)
        I2cTransaction::write_read(address, vec![0x0D], vec![initial_config_port1]), // Read Config Port 1
        I2cTransaction::write(address, vec![0x0D, initial_config_port1 | (1 << 7)]), // Write Config Port 1 with bit 7 set
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).await.unwrap(); // Keep .await

    // Perform the operations
    tca.set_pin_direction(tca6424::Pin::P00, tca6424::PinDirection::Output).await.unwrap(); // Keep .await
    tca.set_pin_direction(tca6424::Pin::P17, tca6424::PinDirection::Input).await.unwrap(); // Keep .await

    i2c_mock.done(); // Check that all expectations were met
}


#[cfg(not(feature = "async"))]
#[test]
fn test_get_pin_direction_sync() {
    let address = 0x22;
    let config_port0_input = 0xFF; // P00-P07 all inputs
    let config_port0_output = 0xFE; // P00 output, others input

    let expectations = [
        // Get P00 direction (expect Input)
        I2cTransaction::write_read(address, vec![0x0C], vec![config_port0_input]).into(),
        // Get P00 direction (expect Output)
        I2cTransaction::write_read(address, vec![0x0C], vec![config_port0_output]).into(),
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).unwrap();

    // Test P00 as Input
    let direction = tca.get_pin_direction(tca6424::Pin::P00).unwrap();
    assert_eq!(direction, tca6424::PinDirection::Input);

    // Test P00 as Output
    let direction = tca.get_pin_direction(tca6424::Pin::P00).unwrap();
    assert_eq!(direction, tca6424::PinDirection::Output);

    i2c_mock.done();
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_get_pin_direction_async() {
    let address = 0x22;
    let config_port0_input = 0xFF; // P00-P07 all inputs
    let config_port0_output = 0xFE; // P00 output, others input

    let expectations = [
        // Get P00 direction (expect Input)
        I2cTransaction::write_read(address, vec![0x0C], vec![config_port0_input]),
        // Get P00 direction (expect Output)
        I2cTransaction::write_read(address, vec![0x0C], vec![config_port0_output]),
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).await.unwrap();
    assert_eq!(tca.get_pin_direction(tca6424::Pin::P00).await.unwrap(), tca6424::PinDirection::Input);
    assert_eq!(tca.get_pin_direction(tca6424::Pin::P00).await.unwrap(), tca6424::PinDirection::Output);
    i2c_mock.done();
}

#[cfg(not(feature = "async"))]
#[test]
fn test_set_pin_output_sync() {
    let address = 0x22;
    let initial_output_port0 = 0x00; // Assume all pins are initially low

    let expectations = [
        // Set P00 to High (set bit 0 in Output Port 0)
        I2cTransaction::write_read(address, vec![0x04], vec![initial_output_port0]).into(), // Read Output Port 0
        I2cTransaction::write(address, vec![0x04, initial_output_port0 | (1 << 0)]).into(), // Write Output Port 0 with bit 0 set
        // Set P00 to Low (clear bit 0 in Output Port 0)
        I2cTransaction::write_read(address, vec![0x04], vec![initial_output_port0 | (1 << 0)]).into(), // Read Output Port 0 (after previous write)
        I2cTransaction::write(address, vec![0x04, initial_output_port0 & !(1 << 0)]).into(), // Write Output Port 0 with bit 0 cleared
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).unwrap();

    // Set P00 to High
    tca.set_pin_output(tca6424::Pin::P00, tca6424::PinState::High).unwrap();
    // Set P00 to Low
    tca.set_pin_output(tca6424::Pin::P00, tca6424::PinState::Low).unwrap();

    i2c_mock.done();
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_set_pin_output_async() {
    let address = 0x22;
    let initial_output_port0 = 0x00; // Assume all pins are initially low

    let expectations = [
        // Set P00 to High (set bit 0 in Output Port 0)
        I2cTransaction::write_read(address, vec![0x04], vec![initial_output_port0]), // Read Output Port 0
        I2cTransaction::write(address, vec![0x04, initial_output_port0 | (1 << 0)]), // Write Output Port 0 with bit 0 set
        // Set P00 to Low (clear bit 0 in Output Port 0)
        I2cTransaction::write_read(address, vec![0x04], vec![initial_output_port0 | (1 << 0)]), // Read Output Port 0 (after previous write)
        I2cTransaction::write(address, vec![0x04, initial_output_port0 & !(1 << 0)]), // Write Output Port 0 with bit 0 cleared
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).await.unwrap();
    tca.set_pin_output(tca6424::Pin::P00, tca6424::PinState::High).await.unwrap();
    tca.set_pin_output(tca6424::Pin::P00, tca6424::PinState::Low).await.unwrap();
    i2c_mock.done();
}

#[cfg(not(feature = "async"))]
#[test]
fn test_get_pin_output_state_sync() {
    let address = 0x22;
    let output_port0_high = 0x01; // P00 high, others low
    let output_port0_low = 0x00; // P00 low, others low

    let expectations = [
        // Get P00 output state (expect High)
        I2cTransaction::write_read(address, vec![0x04], vec![output_port0_high]).into(),
        // Get P00 output state (expect Low)
        I2cTransaction::write_read(address, vec![0x04], vec![output_port0_low]).into(),
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).unwrap();

    // Test P00 output state as High
    let state = tca.get_pin_output_state(tca6424::Pin::P00).unwrap();
    assert_eq!(state, tca6424::PinState::High);

    // Test P00 output state as Low
    let state = tca.get_pin_output_state(tca6424::Pin::P00).unwrap();
    assert_eq!(state, tca6424::PinState::Low);

    i2c_mock.done();
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_get_pin_output_state_async() {
    let address = 0x22;
    let output_port0_high = 0x01; // P00 high, others low
    let output_port0_low = 0x00; // P00 low, others low

    let expectations = [
        // Get P00 output state (expect High)
        I2cTransaction::write_read(address, vec![0x04], vec![output_port0_high]),
        // Get P00 output state (expect Low)
        I2cTransaction::write_read(address, vec![0x04], vec![output_port0_low]),
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).await.unwrap();
    assert_eq!(tca.get_pin_output_state(tca6424::Pin::P00).await.unwrap(), tca6424::PinState::High);
    assert_eq!(tca.get_pin_output_state(tca6424::Pin::P00).await.unwrap(), tca6424::PinState::Low);
    i2c_mock.done();
}

#[cfg(not(feature = "async"))]
#[test]
fn test_get_pin_input_state_sync() {
    let address = 0x22;
    let input_port0_high = 0x01; // P00 high, others low
    let input_port0_low = 0x00; // P00 low, others low

    let expectations = [
        // Get P00 input state (expect High)
        I2cTransaction::write_read(address, vec![0x00], vec![input_port0_high]).into(),
        // Get P00 input state (expect Low)
        I2cTransaction::write_read(address, vec![0x00], vec![input_port0_low]).into(),
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).unwrap();

    // Test P00 input state as High
    let state = tca.get_pin_input_state(tca6424::Pin::P00).unwrap();
    assert_eq!(state, tca6424::PinState::High);

    // Test P00 input state as Low
    let state = tca.get_pin_input_state(tca6424::Pin::P00).unwrap();
    assert_eq!(state, tca6424::PinState::Low);

    i2c_mock.done();
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_get_pin_input_state_async() {
    let address = 0x22;
    let input_port0_high = 0x01; // P00 high, others low
    let input_port0_low = 0x00; // P00 low, others low

    let expectations = [
        // Get P00 input state (expect High)
        I2cTransaction::write_read(address, vec![0x00], vec![input_port0_high]),
        // Get P00 input state (expect Low)
        I2cTransaction::write_read(address, vec![0x00], vec![input_port0_low]),
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).await.unwrap();
    assert_eq!(tca.get_pin_input_state(tca6424::Pin::P00).await.unwrap(), tca6424::PinState::High);
    assert_eq!(tca.get_pin_input_state(tca6424::Pin::P00).await.unwrap(), tca6424::PinState::Low);
    i2c_mock.done();
}

#[cfg(not(feature = "async"))]
#[test]
fn test_set_pin_polarity_inversion_sync() {
    let address = 0x22;
    let initial_polarity_port0 = 0x00; // Assume no inversion initially

    let expectations = [
        // Set P00 to invert (set bit 0 in Polarity Inversion Port 0)
        I2cTransaction::write_read(address, vec![0x08], vec![initial_polarity_port0]).into(), // Read Polarity Port 0
        I2cTransaction::write(address, vec![0x08, initial_polarity_port0 | (1 << 0)]).into(), // Write Polarity Port 0 with bit 0 set
        // Set P00 to not invert (clear bit 0 in Polarity Inversion Port 0)
        I2cTransaction::write_read(address, vec![0x08], vec![initial_polarity_port0 | (1 << 0)]).into(), // Read Polarity Port 0 (after previous write)
        I2cTransaction::write(address, vec![0x08, initial_polarity_port0 & !(1 << 0)]).into(), // Write Polarity Port 0 with bit 0 cleared
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).unwrap();

    // Set P00 to invert
    tca.set_pin_polarity_inversion(tca6424::Pin::P00, true).unwrap();
    // Set P00 to not invert
    tca.set_pin_polarity_inversion(tca6424::Pin::P00, false).unwrap();

    i2c_mock.done();
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_set_pin_polarity_inversion_async() {
    let address = 0x22;
    let initial_polarity_port0 = 0x00; // Assume no inversion initially

    let expectations = [
        // Set P00 to invert (set bit 0 in Polarity Inversion Port 0)
        I2cTransaction::write_read(address, vec![0x08], vec![initial_polarity_port0]), // Read Polarity Port 0
        I2cTransaction::write(address, vec![0x08, initial_polarity_port0 | (1 << 0)]), // Write Polarity Port 0 with bit 0 set
        // Set P00 to not invert (clear bit 0 in Polarity Inversion Port 0)
        I2cTransaction::write_read(address, vec![0x08], vec![initial_polarity_port0 | (1 << 0)]), // Read Polarity Port 0 (after previous write)
        I2cTransaction::write(address, vec![0x08, initial_polarity_port0 & !(1 << 0)]), // Write Polarity Port 0 with bit 0 cleared
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).await.unwrap();
    tca.set_pin_polarity_inversion(tca6424::Pin::P00, true).await.unwrap();
    tca.set_pin_polarity_inversion(tca6424::Pin::P00, false).await.unwrap();
    i2c_mock.done();
}

#[cfg(not(feature = "async"))]
#[test]
fn test_get_pin_polarity_inversion_sync() {
    let address = 0x22;
    let polarity_port0_inverted = 0x01; // P00 inverted, others not
    let polarity_port0_not_inverted = 0x00; // P00 not inverted, others not

    let expectations = [
        // Get P00 polarity (expect Inverted)
        I2cTransaction::write_read(address, vec![0x08], vec![polarity_port0_inverted]).into(),
        // Get P00 polarity (expect Not Inverted)
        I2cTransaction::write_read(address, vec![0x08], vec![polarity_port0_not_inverted]).into(),
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).unwrap();

    // Test P00 polarity as Inverted
    let inverted = tca.get_pin_polarity_inversion(tca6424::Pin::P00).unwrap();
    assert_eq!(inverted, true);

    // Test P00 polarity as Not Inverted
    let inverted = tca.get_pin_polarity_inversion(tca6424::Pin::P00).unwrap();
    assert_eq!(inverted, false);

    i2c_mock.done();
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_get_pin_polarity_inversion_async() {
    let address = 0x22;
    let polarity_port0_inverted = 0x01; // P00 inverted, others not
    let polarity_port0_not_inverted = 0x00; // P00 not inverted, others not

    let expectations = [
        // Get P00 polarity (expect Inverted)
        I2cTransaction::write_read(address, vec![0x08], vec![polarity_port0_inverted]),
        // Get P00 polarity (expect Not Inverted)
        I2cTransaction::write_read(address, vec![0x08], vec![polarity_port0_not_inverted]),
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).await.unwrap();
    assert!(tca.get_pin_polarity_inversion(tca6424::Pin::P00).await.unwrap());
    assert!(!tca.get_pin_polarity_inversion(tca6424::Pin::P00).await.unwrap());
    i2c_mock.done();
}

#[test]
fn simple_sync_test() {
    assert_eq!(1 + 1, 2);
}

// --- Port-based tests ---

#[cfg(not(feature = "async"))]
#[test]
fn test_set_port_direction_sync() {
    let address = 0x22;
    let _initial_config_port0 = 0xFF; // Assume all pins are initially inputs
    let new_direction_mask = 0xAA; // Example: P00, P02, P04, P06 as Output, others Input

    let expectations = [
        // Set Port0 direction
        I2cTransaction::write(address, vec![0x0C, new_direction_mask]).into(), // Write Config Port 0
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).unwrap();

    tca.set_port_direction(Port::Port0, new_direction_mask).unwrap();

    i2c_mock.done();
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_set_port_direction_async() {
    let address = 0x22;
    let _initial_config_port0 = 0xFF; // Assume all pins are initially inputs
    let new_direction_mask = 0xAA; // Example: P00, P02, P04, P06 as Output, others Input

    let expectations = [
        // Set Port0 direction
        I2cTransaction::write(address, vec![0x0C, new_direction_mask]), // Write Config Port 0
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).await.unwrap();

    tca.set_port_direction(Port::Port0, new_direction_mask).await.unwrap();

    i2c_mock.done();
}

#[cfg(not(feature = "async"))]
#[test]
fn test_get_port_direction_sync() {
    let address = 0x22;
    let config_port0_value = 0xAA; // Example configuration

    let expectations = [
        // Get Port0 direction
        I2cTransaction::write_read(address, vec![0x0C], vec![config_port0_value]).into(), // Read Config Port 0
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).unwrap();

    let direction_mask = tca.get_port_direction(Port::Port0).unwrap();
    assert_eq!(direction_mask, config_port0_value);

    i2c_mock.done();
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_get_port_direction_async() {
    let address = 0x22;
    let config_port0_value = 0xAA; // Example configuration

    let expectations = [
        // Get Port0 direction
        I2cTransaction::write_read(address, vec![0x0C], vec![config_port0_value]), // Read Config Port 0
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).await.unwrap();

    let direction_mask = tca.get_port_direction(Port::Port0).await.unwrap();
    assert_eq!(direction_mask, config_port0_value);

    i2c_mock.done();
}

#[cfg(not(feature = "async"))]
#[test]
fn test_set_port_output_sync() {
    let address = 0x22;
    let new_output_mask = 0x55; // Example: P00, P02, P04, P06 as Low, others High

    let expectations = [
        // Set Port0 output
        I2cTransaction::write(address, vec![0x04, new_output_mask]).into(), // Write Output Port 0
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).unwrap();

    tca.set_port_output(Port::Port0, new_output_mask).unwrap();

    i2c_mock.done();
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_set_port_output_async() {
    let address = 0x22;
    let new_output_mask = 0x55; // Example: P00, P02, P04, P06 as Low, others High

    let expectations = [
        // Set Port0 output
        I2cTransaction::write(address, vec![0x04, new_output_mask]), // Write Output Port 0
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).await.unwrap();

    tca.set_port_output(Port::Port0, new_output_mask).await.unwrap();

    i2c_mock.done();
}

#[cfg(not(feature = "async"))]
#[test]
fn test_get_port_output_state_sync() {
    let address = 0x22;
    let output_port0_value = 0x55; // Example output state

    let expectations = [
        // Get Port0 output state
        I2cTransaction::write_read(address, vec![0x04], vec![output_port0_value]).into(), // Read Output Port 0
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).unwrap();

    let output_mask = tca.get_port_output_state(Port::Port0).unwrap();
    assert_eq!(output_mask, output_port0_value);

    i2c_mock.done();
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_get_port_output_state_async() {
    let address = 0x22;
    let output_port0_value = 0x55; // Example output state

    let expectations = [
        // Get Port0 output state
        I2cTransaction::write_read(address, vec![0x04], vec![output_port0_value]), // Read Output Port 0
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).await.unwrap();

    let output_mask = tca.get_port_output_state(Port::Port0).await.unwrap();
    assert_eq!(output_mask, output_port0_value);

    i2c_mock.done();
}

#[cfg(not(feature = "async"))]
#[test]
fn test_get_port_input_state_sync() {
    let address = 0x22;
    let input_port0_value = 0xC3; // Example input state

    let expectations = [
        // Get Port0 input state
        I2cTransaction::write_read(address, vec![0x00], vec![input_port0_value]).into(), // Read Input Port 0
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).unwrap();

    let input_mask = tca.get_port_input_state(Port::Port0).unwrap();
    assert_eq!(input_mask, input_port0_value);

    i2c_mock.done();
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_get_port_input_state_async() {
    let address = 0x22;
    let input_port0_value = 0xC3; // Example input state

    let expectations = [
        // Get Port0 input state
        I2cTransaction::write_read(address, vec![0x00], vec![input_port0_value]), // Read Input Port 0
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).await.unwrap();

    let input_mask = tca.get_port_input_state(Port::Port0).await.unwrap();
    assert_eq!(input_mask, input_port0_value);

    i2c_mock.done();
}

#[cfg(not(feature = "async"))]
#[test]
fn test_set_port_polarity_inversion_sync() {
    let address = 0x22;
    let new_polarity_mask = 0xF0; // Example: P04-P07 inverted, others not

    let expectations = [
        // Set Port0 polarity inversion
        I2cTransaction::write(address, vec![0x08, new_polarity_mask]).into(), // Write Polarity Inversion Port 0
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).unwrap();

    tca.set_port_polarity_inversion(Port::Port0, new_polarity_mask).unwrap();

    i2c_mock.done();
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_set_port_polarity_inversion_async() {
    let address = 0x22;
    let new_polarity_mask = 0xF0; // Example: P04-P07 inverted, others not

    let expectations = [
        // Set Port0 polarity inversion
        I2cTransaction::write(address, vec![0x08, new_polarity_mask]), // Write Polarity Inversion Port 0
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).await.unwrap();

    tca.set_port_polarity_inversion(Port::Port0, new_polarity_mask).await.unwrap();

    i2c_mock.done();
}

#[cfg(not(feature = "async"))]
#[test]
fn test_get_port_polarity_inversion_sync() {
    let address = 0x22;
    let polarity_port0_value = 0xF0; // Example polarity inversion state

    let expectations = [
        // Get Port0 polarity inversion state
        I2cTransaction::write_read(address, vec![0x08], vec![polarity_port0_value]).into(), // Read Polarity Inversion Port 0
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).unwrap();

    let polarity_mask = tca.get_port_polarity_inversion(Port::Port0).unwrap();
    assert_eq!(polarity_mask, polarity_port0_value);

    i2c_mock.done();
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_get_port_polarity_inversion_async() {
    let address = 0x22;
    let polarity_port0_value = 0xF0; // Example polarity inversion state

    let expectations = [
        // Get Port0 polarity inversion state
        I2cTransaction::write_read(address, vec![0x08], vec![polarity_port0_value]), // Read Polarity Inversion Port 0
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).await.unwrap();

    let polarity_mask = tca.get_port_polarity_inversion(Port::Port0).await.unwrap();
    assert_eq!(polarity_mask, polarity_port0_value);

    i2c_mock.done();
}

// --- Auto-Increment Tests ---

#[cfg(not(feature = "async"))]
#[test]
fn test_set_ports_direction_ai_sync() {
    let address = 0x22;
    let direction_masks = [0xAA, 0x55, 0xCC]; // Masks for Port0, Port1, Port2

    let expectations = [
        // Set Port0, Port1, Port2 directions using AI (Config Port 0 is 0x0C, AI bit is 0x80)
        I2cTransaction::write(address, vec![0x0C | 0x80, 0xAA, 0x55, 0xCC]).into(),
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).unwrap();

    tca.set_ports_direction_ai(Port::Port0, &direction_masks).unwrap();

    i2c_mock.done();
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_set_ports_direction_ai_async() {
    let address = 0x22;
    let direction_masks = [0xAA, 0x55, 0xCC]; // Masks for Port0, Port1, Port2

    let expectations = [
        // Set Port0, Port1, Port2 directions using AI (Config Port 0 is 0x0C, AI bit is 0x80)
        I2cTransaction::write(address, vec![0x0C | 0x80, 0xAA, 0x55, 0xCC]),
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).await.unwrap();

    tca.set_ports_direction_ai(Port::Port0, &direction_masks).await.unwrap();

    i2c_mock.done();
}

#[cfg(not(feature = "async"))]
#[test]
fn test_get_ports_direction_ai_sync() {
    let address = 0x22;
    let expected_direction_masks = [0xAA, 0x55, 0xCC]; // Expected masks for Port0, Port1, Port2

    let expectations = [
        // Get Port0, Port1, Port2 directions using AI (Config Port 0 is 0x0C, AI bit is 0x80)
        I2cTransaction::write_read(address, vec![0x0C | 0x80], expected_direction_masks.to_vec()).into(),
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).unwrap();

    let mut read_buffer = [0u8; 3];
    tca.get_ports_direction_ai(Port::Port0, &mut read_buffer).unwrap();
    assert_eq!(read_buffer, expected_direction_masks);

    i2c_mock.done();
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_get_ports_direction_ai_async() {
    let address = 0x22;
    let expected_direction_masks = [0xAA, 0x55, 0xCC]; // Expected masks for Port0, Port1, Port2

    let expectations = [
        // Get Port0, Port1, Port2 directions using AI (Config Port 0 is 0x0C, AI bit is 0x80)
        I2cTransaction::write_read(address, vec![0x0C | 0x80], expected_direction_masks.to_vec()),
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).await.unwrap();

    let mut read_buffer = [0u8; 3];
    tca.get_ports_direction_ai(Port::Port0, &mut read_buffer).await.unwrap();
    assert_eq!(read_buffer, expected_direction_masks);

    i2c_mock.done();
}

#[cfg(not(feature = "async"))]
#[test]
fn test_set_ports_output_ai_sync() {
    let address = 0x22;
    let output_masks = [0x11, 0x22, 0x33]; // Masks for Port0, Port1, Port2

    let expectations = [
        // Set Port0, Port1, Port2 outputs using AI (Output Port 0 is 0x04, AI bit is 0x80)
        I2cTransaction::write(address, vec![0x04 | 0x80, 0x11, 0x22, 0x33]).into(),
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).unwrap();

    tca.set_ports_output_ai(Port::Port0, &output_masks).unwrap();

    i2c_mock.done();
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_set_ports_output_ai_async() {
    let address = 0x22;
    let output_masks = [0x11, 0x22, 0x33]; // Masks for Port0, Port1, Port2

    let expectations = [
        // Set Port0, Port1, Port2 outputs using AI (Output Port 0 is 0x04, AI bit is 0x80)
        I2cTransaction::write(address, vec![0x04 | 0x80, 0x11, 0x22, 0x33]),
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).await.unwrap();

    tca.set_ports_output_ai(Port::Port0, &output_masks).await.unwrap();

    i2c_mock.done();
}

#[cfg(not(feature = "async"))]
#[test]
fn test_get_ports_output_state_ai_sync() {
    let address = 0x22;
    let expected_output_masks = [0x11, 0x22, 0x33]; // Expected masks for Port0, Port1, Port2

    let expectations = [
        // Get Port0, Port1, Port2 output states using AI (Output Port 0 is 0x04, AI bit is 0x80)
        I2cTransaction::write_read(address, vec![0x04 | 0x80], expected_output_masks.to_vec()).into(),
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).unwrap();

    let mut read_buffer = [0u8; 3];
    tca.get_ports_output_state_ai(Port::Port0, &mut read_buffer).unwrap();
    assert_eq!(read_buffer, expected_output_masks);

    i2c_mock.done();
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_get_ports_output_state_ai_async() {
    let address = 0x22;
    let expected_output_masks = [0x11, 0x22, 0x33]; // Expected masks for Port0, Port1, Port2

    let expectations = [
        // Get Port0, Port1, Port2 output states using AI (Output Port 0 is 0x04, AI bit is 0x80)
        I2cTransaction::write_read(address, vec![0x04 | 0x80], expected_output_masks.to_vec()),
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).await.unwrap();

    let mut read_buffer = [0u8; 3];
    tca.get_ports_output_state_ai(Port::Port0, &mut read_buffer).await.unwrap();
    assert_eq!(read_buffer, expected_output_masks);

    i2c_mock.done();
}

#[cfg(not(feature = "async"))]
#[test]
fn test_get_ports_input_state_ai_sync() {
    let address = 0x22;
    let expected_input_masks = [0xDD, 0xEE, 0xFF]; // Expected masks for Port0, Port1, Port2

    let expectations = [
        // Get Port0, Port1, Port2 input states using AI (Input Port 0 is 0x00, AI bit is 0x80)
        I2cTransaction::write_read(address, vec![0x00 | 0x80], expected_input_masks.to_vec()).into(),
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).unwrap();

    let mut read_buffer = [0u8; 3];
    tca.get_ports_input_state_ai(Port::Port0, &mut read_buffer).unwrap();
    assert_eq!(read_buffer, expected_input_masks);

    i2c_mock.done();
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_get_ports_input_state_ai_async() {
    let address = 0x22;
    let expected_input_masks = [0xDD, 0xEE, 0xFF]; // Expected masks for Port0, Port1, Port2

    let expectations = [
        // Get Port0, Port1, Port2 input states using AI (Input Port 0 is 0x00, AI bit is 0x80)
        I2cTransaction::write_read(address, vec![0x80], expected_input_masks.to_vec()),
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).await.unwrap();

    let mut read_buffer = [0u8; 3];
    tca.get_ports_input_state_ai(Port::Port0, &mut read_buffer).await.unwrap();
    assert_eq!(read_buffer, expected_input_masks);

    i2c_mock.done();
}

#[cfg(not(feature = "async"))]
#[test]
fn test_set_ports_polarity_inversion_ai_sync() {
    let address = 0x22;
    let inversion_masks = [0x0F, 0xF0, 0xAA]; // Masks for Port0, Port1, Port2

    let expectations = [
        // Set Port0, Port1, Port2 polarity inversions using AI (Polarity Inversion Port 0 is 0x08, AI bit is 0x80)
        I2cTransaction::write(address, vec![0x08 | 0x80, 0x0F, 0xF0, 0xAA]).into(),
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).unwrap();

    tca.set_ports_polarity_inversion_ai(Port::Port0, &inversion_masks).unwrap();

    i2c_mock.done();
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_set_ports_polarity_inversion_ai_async() {
    let address = 0x22;
    let inversion_masks = [0x0F, 0xF0, 0xAA]; // Masks for Port0, Port1, Port2

    let expectations = [
        // Set Port0, Port1, Port2 polarity inversions using AI (Polarity Inversion Port 0 is 0x08, AI bit is 0x80)
        I2cTransaction::write(address, vec![0x08 | 0x80, 0x0F, 0xF0, 0xAA]),
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).await.unwrap();

    tca.set_ports_polarity_inversion_ai(Port::Port0, &inversion_masks).await.unwrap();

    i2c_mock.done();
}

#[cfg(not(feature = "async"))]
#[test]
fn test_get_ports_polarity_inversion_ai_sync() {
    let address = 0x22;
    let expected_inversion_masks = [0x0F, 0xF0, 0xAA]; // Expected masks for Port0, Port1, Port2

    let expectations = [
        // Get Port0, Port1, Port2 polarity inversions using AI (Polarity Inversion Port 0 is 0x08, AI bit is 0x80)
        I2cTransaction::write_read(address, vec![0x08 | 0x80], expected_inversion_masks.to_vec()).into(),
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).unwrap();

    let mut read_buffer = [0u8; 3];
    tca.get_ports_polarity_inversion_ai(Port::Port0, &mut read_buffer).unwrap();
    assert_eq!(read_buffer, expected_inversion_masks);

    i2c_mock.done();
}

#[cfg(feature = "async")]
#[tokio::test]
async fn test_get_ports_polarity_inversion_ai_async() {
    let address = 0x22;
    let expected_inversion_masks = [0x0F, 0xF0, 0xAA]; // Expected masks for Port0, Port1, Port2

    let expectations = [
        // Get Port0, Port1, Port2 polarity inversions using AI (Polarity Inversion Port 0 is 0x08, AI bit is 0x80)
        I2cTransaction::write_read(address, vec![0x08 | 0x80], expected_inversion_masks.to_vec()),
    ];

    let mut i2c_mock = I2cMock::new(&expectations);
    let mut tca = tca6424::Tca6424::new(&mut i2c_mock, address).await.unwrap();

    let mut read_buffer = [0u8; 3];
    tca.get_ports_polarity_inversion_ai(Port::Port0, &mut read_buffer).await.unwrap();
    assert_eq!(read_buffer, expected_inversion_masks);

    i2c_mock.done();
}
