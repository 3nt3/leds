#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::Pwm;
use stm32f1xx_hal::{delay::Delay, pac, prelude::*, pwm::Channel, time::U32Ext};
use panic_halt as _;
use rtt_target::rprintln;
use stm32f1xx_hal::timer::{Tim2NoRemap, Timer};

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(8.mhz()).freeze(&mut flash.acr);

    let mut afio = p.AFIO.constrain(&mut rcc.apb2);

    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);

    // TIM2
    let c1 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let c2 = gpioa.pa1.into_alternate_push_pull(&mut gpioa.crl);
    let c3 = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let pins = (c1, c2, c3);

    let mut pwm = Timer::tim2(p.TIM2, &clocks, &mut rcc.apb1).pwm::<Tim2NoRemap, _, _, _>(
        pins,
        &mut afio.mapr,
        1.khz(),
    );

    pwm.set_period(1.khz());

    let mut delay = Delay::new(cp.SYST, clocks);
    let max = pwm.get_max_duty();

    loop {
        for r in 0u8..0xFF {
            for g in 0u8..0xFF {
                for b in 0u8..0xFF {
                    pwm.set_duty(Channel::C1, ((r / 0xFF) as f32 * max as f32) as u16);
                    pwm.set_duty(Channel::C2, ((g / 0xFF) as f32 * max as f32) as u16);
                    pwm.set_duty(Channel::C3, ((b / 0xFF) as f32 * max as f32) as u16);

                    rprintln!("rgb: {r} {g} {b}");

                    delay.delay_us(10u16);
                }
            }
        }
    }
}