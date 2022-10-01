//! Blinks an LED
//!
//! This assumes that a LED is connected to GPIO4.
//! Depending on your target and the board you are using you should change the pin.
//! If your board doesn't have on-board LEDs don't forget to add an appropriate resistor.
//!

use core::time;
use std::thread;
use std::time::Duration;

use embedded_hal::digital::v2::OutputPin;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::ledc::config::TimerConfig;
use esp_idf_hal::ledc::{Channel, Timer};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_sys::EspError;

struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

//struct RGBPins {
//r_channel: Channel,
//b_channel: Channel,
//g_channel: Channel,
//}

//impl RGBPins {
//fn new(r_pin: &mut OutputPin, g_pin: &mut OutputPin, b_pin: &mut OutputPin) -> Self {
//RGBPins {
//r_channel: r_pin,
//b_channel: b_pin,
//g_channel: g_pin,
//}
//}

//fn set_rgb(&mut self, &rgb: RGB) {}
//}

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let config = TimerConfig::default().frequency(10.kHz().into());
    let timer = Timer::new(peripherals.ledc.timer0, &config)?;

    let red_pin = peripherals.pins.gpio26.into_output()?;
    // AHHH this can only be an input pin like physically
    // let mut g = peripherals.pins.gpio34.into_output()?;
    let green_pin = peripherals.pins.gpio22.into_output()?;
    let blue_pin = peripherals.pins.gpio13.into_output()?;

    let mut r_channel = Channel::new(peripherals.ledc.channel0, &timer, red_pin)?;
    let mut g_channel = Channel::new(peripherals.ledc.channel1, &timer, green_pin)?;
    let mut b_channel = Channel::new(peripherals.ledc.channel2, &timer, blue_pin)?;

    // let mut rgb_pins = RGBPins::new(&mut red_pin, &mut green_pin, &mut blue_pin);

    let max_duty = r_channel.get_max_duty();
    println!("max duty: {max_duty}");
    for r in 0..=255 {
        r_channel.set_duty(r)?;
        println!("r: {r}");
        thread::sleep(Duration::from_millis(10));
    }

    loop {}

    loop {
        for r in 1..0xFF {
            for g in 1..0xFF {
                for b in 1..0xFF {
                    println!("rgb: {r} {g} {b}");
                    r_channel.set_duty(max_duty * (r / 0xFF))?;
                    g_channel.set_duty(max_duty * (g / 0xFF))?;
                    b_channel.set_duty(max_duty * (b / 0xFF))?;
                    thread::sleep(Duration::from_millis(10));
                }
            }
        }
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
