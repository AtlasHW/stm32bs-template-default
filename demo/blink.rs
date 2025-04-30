//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m::delay::Delay;
use {{ pac_name }}::{{ pac_feature }} as stm32;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut delay = Delay::new(cp.SYST, {{ HSI_freq }});
    let gpiob = dp.GPIOB;
    dp.RCC.iopenr.modify(|_, w| w.iopben().set_bit());
    gpiob.moder.modify(|_, w| w.moder5().output());
    gpiob.otyper.modify(|_, w| w.ot5().push_pull());

    loop {
        gpiob.odr.modify(|_, w| w.odr5().set_bit());
        delay.delay_ms(500);
        gpiob.odr.modify(|_, w| w.odr5().clear_bit());
        delay.delay_ms(500);
    }
}
