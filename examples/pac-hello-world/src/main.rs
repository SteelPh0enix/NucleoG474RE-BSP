#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as semihosting;
extern crate pac;
extern crate panic_semihosting;

use cortex_m::delay::Delay;
use rt::entry;
use semihosting::hprintln;

#[entry]
fn main() -> ! {
    hprintln!("twuj stary");

    let mcu_periphs = unsafe { pac::Peripherals::steal() };
    let cortex_periphs = unsafe { cortex_m::Peripherals::steal() };
    let rcc = mcu_periphs.RCC;
    let gpio = mcu_periphs.GPIOA;

    let mut delay = Delay::new(cortex_periphs.SYST, 16_000_000);

    rcc.rcc_ahb2enr.modify(|_, w| w.gpioaen().set_bit());
    gpio.moder.modify(|_, w| unsafe { w.moder5().bits(0b01) });

    loop {
        gpio.odr.modify(|_, w| w.odr5().set_bit());
        delay.delay_ms(500);

        gpio.odr.modify(|_, w| w.odr5().clear_bit());
        delay.delay_ms(500);
    }
}
