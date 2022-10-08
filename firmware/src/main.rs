use std::thread;
use std::time::Duration;

use embedded_hal::PwmPin;
use esp_idf_hal::ledc::config::TimerConfig;
use esp_idf_hal::ledc::{Channel, Timer};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;

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

    println!("max duty: {max_duty}");
    loop {
        rgb_pins.trans();
        rgb_pins.off();
        thread::sleep(Duration::from_millis(3000));
    }
}

impl<R: PwmPin<Duty = u32>, G: PwmPin<Duty = u32>, B: PwmPin<Duty = u32>> RGBPins<R, G, B> {
    fn set_rgb(&mut self, r: u8, g: u8, b: u8) {
        self.off();

        println!("rgb {} {} {}", scale_down(r), scale_down(g), scale_down(b));
        let r_lin = to_linear(scale_down(r));
        let g_lin = to_linear(scale_down(g));
        let b_lin = to_linear(scale_down(b));
        println!("rgb lin {r_lin} {g_lin} {b_lin}");

        let max_duty = self.r_pin.get_max_duty();
        self.r_pin
            .set_duty((max_duty as f32 * (1.0 - r_lin)) as u32);
        self.g_pin
            .set_duty((max_duty as f32 * (1.0 - g_lin)) as u32);
        self.b_pin
            .set_duty((max_duty as f32 * (1.0 - b_lin)) as u32);
    }
    fn off(&mut self) {
        let max_duty = self.r_pin.get_max_duty();
        self.r_pin.set_duty(max_duty);
        self.g_pin.set_duty(max_duty);
        self.b_pin.set_duty(max_duty);
    }

    fn trans(&mut self) {
        self.off();

        let blue_r = 91;
        let blue_g = 206;
        let blue_b = 250;

        let pink_r = 245;
        let pink_g = 245;
        let pink_b = 184;

        self.set_rgb(blue_r, blue_g, blue_b);
        thread::sleep(Duration::from_millis(1000));
        self.set_rgb(pink_r, pink_g, pink_b);
        thread::sleep(Duration::from_millis(1000));
        self.set_rgb(0xff, 0xff, 0xff);
        thread::sleep(Duration::from_millis(1000));
        self.set_rgb(pink_r, pink_g, pink_b);
        thread::sleep(Duration::from_millis(1000));
        self.set_rgb(blue_r, blue_g, blue_b);
        thread::sleep(Duration::from_millis(1000));
    }
}

fn to_linear(x: f32) -> f32 {
    // if x.lt_eq(&T::from_f64(0.04045)) => T::from_f64(1.0 / 12.92) * &x,
    // else => x.clone().mul_add(T::from_f64(1.0 / 1.055), T::from_f64(0.055 / 1.055)).powf(T::from_f64(2.4)),
    if x <= 0.04045 {
        1.0 / 12.92 * x
    } else {
        x.mul_add(1.0 / 1.055, 0.055 / 1.055).powf(2.4)
    }
}

fn scale_down(x: u8) -> f32 {
    (x as f32) * (1.0 / 255.0)
}
