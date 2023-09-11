    // These are implicitly used by the spi driver if they are in the correct mode
    let spi_mosi = pins.gpio7.into_function::<hal::gpio::FunctionSpi>();
    let spi_miso = pins.gpio4.into_function::<hal::gpio::FunctionSpi>();
    let spi_sclk = pins.gpio6.into_function::<hal::gpio::FunctionSpi>();
    let spi = hal::spi::Spi::<_, _, _, 8>::new(pac.SPI0, (spi_mosi, spi_miso, spi_sclk));

    // Exchange the uninitialised SPI driver for an initialised one
    let mut spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        16.MHz(),
        embedded_hal::spi::MODE_0,
    );

    // Write out 0, ignore return value
    if spi.write(&[0]).is_ok() {
        // SPI write was succesful
    };

    // write 50, then check the return
    let send_success = spi.send(50);
    match send_success {
        Ok(_) => {
            // We succeeded, check the read value
            if let Ok(_x) = spi.read() {
                // We got back `x` in exchange for the 0x50 we sent.
            };
        }
        Err(_) => todo!(),
    }

    // Do a read+write at the same time. Data in `buffer` will be replaced with
    // the data read from the SPI device.
    let mut buffer: [u8; 4] = [1, 2, 3, 4];
    let transfer_success = spi.transfer(&mut buffer);
    #[allow(clippy::single_match)]
    match transfer_success {
        Ok(_) => {}  // Handle success
        Err(_) => {} // handle errors
    };