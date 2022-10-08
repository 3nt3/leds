use std::thread;
use std::time::Duration;

use esp_idf_hal::ledc::config::TimerConfig;
use esp_idf_hal::ledc::{Channel, Timer};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let config = TimerConfig::default().frequency(1.kHz().into());
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

    loop {
        for r in 1..=0xFF {
            //println!("rgb: {r} 0 0");
            r_channel.set_duty(((max_duty as f32) * (1.0 - (r as f32 / 0xFF as f32))) as u32)?;
            thread::sleep(Duration::from_millis(10));
        }
        for g in 1..=0xFF {
            // println!("rgb: 255 {g} 0");
            g_channel.set_duty(((max_duty as f32) * (1.0 - (g as f32 / 0xFF as f32))) as u32)?;
            thread::sleep(Duration::from_millis(10));
        }
        for b in 1..=0xFF {
            // println!("rgb: 255 255 {b}");
            b_channel.set_duty(((max_duty as f32) * (1.0 - (b as f32 / 0xFF as f32))) as u32)?;
            thread::sleep(Duration::from_millis(10));
        }
        thread::sleep(Duration::from_millis(100));
    }
}
