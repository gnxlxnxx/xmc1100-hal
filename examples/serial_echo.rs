#![no_main]
#![no_std]

#[allow(unused)]
use panic_semihosting;

use xmc1100_hal as hal;

use crate::hal::delay::Delay;
use crate::hal::prelude::*;
use crate::hal::scu::Scu;
use crate::hal::serial::Serial;
use crate::hal::time::Bps;
use crate::hal::xmc1100;
use core::fmt::Write;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (xmc1100::Peripherals::take(), Peripherals::take()) {
        cortex_m::interrupt::free(move |cs| {
            let port2 = p.PORT2.split();

            let mut scu = Scu::new(p.SCU_GENERAL, p.SCU_CLK);

            let rx = port2.p2_2.into_floating_input(&cs);
            let tx = port2.p2_0.into_alternate_af6(&cs);
            let tx = port2.p2_1.into_alternate_af6(&cs);

            // Create usart
            let mut serial = Serial::usic0_ch0(p.USIC0_CH0, ((), ()), Bps(9600), &mut scu);
            loop {
                // Wait for reception of a single byte
                let received = nb::block!(serial.read()).unwrap();

                // Send back previously received byte and wait for completion
                nb::block!(serial.write(received)).ok();
            }
        });
    }

    loop {
        continue;
    }
}
