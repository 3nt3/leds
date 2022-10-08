use std::thread;
use std::time::Duration;

use embedded_hal::PwmPin;
use esp_idf_hal::ledc::config::TimerConfig;
use esp_idf_hal::ledc::{Channel, Timer};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_sys::EspError;

struct RGBPins<R, G, B> {
    r_pin: R,
    g_pin: G,
    b_pin: B,
}

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

    let r_channel = Channel::new(peripherals.ledc.channel0, &timer, red_pin)?;
    let g_channel = Channel::new(peripherals.ledc.channel1, &timer, green_pin)?;
    let b_channel = Channel::new(peripherals.ledc.channel2, &timer, blue_pin)?;

    let max_duty = r_channel.get_max_duty();
    let mut rgb_pins = RGBPins {
        r_pin: r_channel,
        g_pin: g_channel,
        b_pin: b_channel,
    };
    // let mut rgb_pins = RGBPins::new(&mut red_pin, &mut green_pin, &mut blue_pin);

    println!("max duty: {max_duty}");
    loop {
        rgb_pins.set_rgb(0xff, 0, 0);
        thread::sleep(Duration::from_millis(1000));
        rgb_pins.set_rgb(0, 0xff, 0);
        thread::sleep(Duration::from_millis(1000));
        rgb_pins.set_rgb(0, 0, 0xff);
        thread::sleep(Duration::from_millis(1000));
    }
}

impl<R: PwmPin<Duty = u32>, G: PwmPin<Duty = u32>, B: PwmPin<Duty = u32>> RGBPins<R, G, B> {
    fn set_rgb(&mut self, r: u8, g: u8, b: u8) {
        println!("rgb: {r} {g} {b}");
        self.off();
        let max_duty = self.r_pin.get_max_duty();
        self.r_pin
            .set_duty(((max_duty as f32) * (1.0 - (r as f32 / 0xFF as f32))) as u32);
        self.g_pin
            .set_duty(((max_duty as f32) * (1.0 - (g as f32 / 0xFF as f32))) as u32);
        self.b_pin
            .set_duty(((max_duty as f32) * (1.0 - (b as f32 / 0xFF as f32))) as u32);
    }

    fn off(&mut self) {
        let max_duty = self.r_pin.get_max_duty();
        self.r_pin.set_duty(max_duty);
        self.g_pin.set_duty(max_duty);
        self.b_pin.set_duty(max_duty);
    }
}
