//! Blinks an LED
//!
//! This assumes that a LED is connected to GPIO4.
//! Depending on your target and the board you are using you should change the pin.
//! If your board doesn't have on-board LEDs don't forget to add an appropriate resistor.
//!

use core::time;
use std::thread;

use embedded_hal::digital::v2::OutputPin;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let mut led = peripherals.pins.gpio26.into_output()?;

    loop {
        led.set_high()?;
        // we are sleeping here to make sure the watchdog isn't triggered
        thread::sleep(time::Duration::from_millis(1000));

        led.set_low()?;
        thread::sleep(time::Duration::from_millis(1000))
    }
}
