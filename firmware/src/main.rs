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
use esp_idf_sys::EspError;

struct RGB {
    r: u8,
    g: u8,
    b: u8
}

struct RGBPins {
    r_pin: OutputPin,
    b_pin: OutputPin,
    g_pin: OutputPin
}

impl RGBPins {
    fn new(&mut self,  r_pin: &mut OutputPin,  g_pin:&mut OutputPin,  b_pin: &mut OutputPin) {
        self.r_pin = r_pin;
        self.g_pin = g_pin;
        self.b_pin = b_pin;
    }

    fn set_rgb(&mut self, &rgb: RGB) {
    }
}

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let mut r = peripherals.pins.gpio26.into_output()?;
    // AHHH this can only be an input pin like physically
    // let mut g = peripherals.pins.gpio34.into_output()?;
    let mut b = peripherals.pins.gpio13.into_output()?;

    let mut rgb_pins 

    loop {
        println!("blinking red");
        blink_pin_thrice(&mut r)?;
        // println!("blinking green");
        // blink_pin_thrice(&mut g);
        println!("blinking blue");
        blink_pin_thrice(&mut b)?;
    }
}

fn blink_pin_thrice(pin: &mut dyn OutputPin<Error = EspError>) -> Result<(), EspError> {
    for _ in 1..3 {
        pin.set_high()?;
        thread::sleep(time::Duration::from_millis(1000));

        pin.set_low()?;
        thread::sleep(time::Duration::from_millis(1000));
    }
    Ok(())
}

fn set_rgb()
