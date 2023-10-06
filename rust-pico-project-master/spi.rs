//! # SPI Example
//!
//! This application demonstrates how to use the SPI Driver to talk to a remote
//! SPI device.
//!
//!
//! It may need to be adapted to your particular board layout and/or pin
//! assignment.
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]
extern crate panic_halt;
extern crate embedded_hal;
extern crate rp2040_hal;
extern crate fugit;
extern crate cortex_m;
extern crate log;
// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
//use panic_halt as _;

// Alias for our HAL crate
use rp2040_hal as hal;

// Some traits we need
use cortex_m::prelude::*;
use fugit::RateExtU32;
use rp2040_hal::clocks::Clock;
// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use hal::pac;
use embedded_hal::digital::v2::OutputPin;
//use cortex_m::delay::Delay

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
/// Note: This boot block is not necessary when using a rp-hal based BSP
/// as the BSPs already perform this step.
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;
/// Entry point to our bare-metal application.
///
/// The `#[rp2040_hal::entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables and the spinlock are initialised.
///
/// The function configures the RP2040 peripherals, then performs some example
/// SPI transactions, then goes to sleep.
#[rp2040_hal::entry]
fn main() -> ! {
    // Grab our singleton objects
    //defmt::println!("Connected!");
    let mut pac = pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    //blinky utils
    let core = pac::CorePeripherals::take().unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let mut led_pin = pins.gpio16.into_push_pull_output();

    led_pin.set_high().unwrap();
    delay.delay_ms(500);
    led_pin.set_low().unwrap();
    delay.delay_ms(500);

    // These are implicitly used by the spi driver if they are in the correct mode
    let _spi_mosi = pins.gpio7.into_mode::<hal::gpio::FunctionSpi>();
    let _spi_miso = pins.gpio4.into_mode::<hal::gpio::FunctionSpi>();
    let _spi_sclk = pins.gpio6.into_mode::<hal::gpio::FunctionSpi>();
    //let spi = hal::spi::Spi::<_, _, _, 8>::new(pac.SPI0, (spi_mosi, spi_miso, spi_sclk));
    let spi = hal::spi::Spi::<_, _, 8>::new(pac.SPI0);

    // Exchange the uninitialised SPI driver for an initialised one
    let mut spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        16.MHz(),
        &embedded_hal::spi::MODE_0,
    );

    // Write out 0, ignore return value
    if spi.write(&[0]).is_ok() {
        // SPI write was succesful
        log::info!("write worked");
    }
    else {log::error!("write failed");};

    // write 50, then check the return
    let send_success = spi.send(50);
    match send_success {
        Ok(_) => {
            // We succeeded, check the read value
            if let Ok(x) = spi.read() {
                // We got back `x` in exchange for the 0x50 we sent.
                log::info!("read worked, rec {}", x);
            };
        }
        Err(_) => log::error!("read failed")
    }

    // Do a read+write at the same time. Data in `buffer` will be replaced with
    // the data read from the SPI device.
    //let mut buffer: [u8; 4] = [1, 2, 3, 4];
    let mut buffer = [0x05, 0x01, 0x01, 0x02];
    let transfer_success = spi.transfer(&mut buffer);
    #[allow(clippy::single_match)]
    match transfer_success {
        Ok(out) => {
            for i in 0..out.len() {
                led_pin.set_high().unwrap();
                delay.delay_ms(500);
                led_pin.set_low().unwrap();
                delay.delay_ms(500);
                for _ in 0..out[i] {
                    led_pin.set_high().unwrap();
                    delay.delay_ms(500);
                    led_pin.set_low().unwrap();
                    delay.delay_ms(500);
                }
                delay.delay_ms(2000);
            }
        }  // Handle success
        Err(_) => {log::error!("read+write failed")} // handle errors
    };

    loop {
        cortex_m::asm::wfi();
    }
}

// End of file