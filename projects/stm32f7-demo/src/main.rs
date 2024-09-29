#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _; 
use stm32f7xx_hal as hal;
use hal::{prelude::*, stm32, delay::Delay};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    // Set up the LED on the Nucleo-F767ZI
    let gpiob = dp.GPIOB.split();
    let mut led = gpiob.pb7.into_push_pull_output();

    // Set up system clock
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();

    // Set up delay for toggling
    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        led.set_high().unwrap();
        delay.delay_ms(500_u32);
        led.set_low().unwrap();
        delay.delay_ms(500_u32);
    }
}