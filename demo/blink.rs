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
    let gpio{{ port | downcase }} = dp.GPIO{{ port | upcase }};
	
{% if pac_name == 'stm32g0' %}

    dp.RCC.iopenr.modify(|_, w| w.iop{{ port | downcase }}en().set_bit());
    gpio{{ port | downcase }}.moder.modify(|_, w| w.moder{{ pin }}().output());
    gpio{{ port | downcase }}.otyper.modify(|_, w| w.ot{{ pin }}().push_pull());

{% elsif pac_name == 'stm32f0' %}
	{% if pin >= 8 %}
	//High bits
	{% else %}
	// Low bits
	{% endif %}
{% endif %}

    loop {
        gpio{{ port | downcase }}.odr.modify(|_, w| w.odr{{ pin }}().set_bit());
        delay.delay_ms(500);
        gpio{{ port | downcase }}.odr.modify(|_, w| w.odr{{ pin }}().clear_bit());
        delay.delay_ms(500);
    }
}
