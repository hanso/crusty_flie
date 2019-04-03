// IMPORTANT the standard `main` interface is not used because it requires nightly
#![no_main]
#![no_std]
#![feature(lang_items)]
extern crate cortex_m_rt;

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate crazy_flie as board;
extern crate embedded_hal as hal;

use cortex_m_rt::entry;

use board::hal::delay::Delay;
use board::hal::prelude::*;
use board::hal::stm32;
use board::gpio;
use board::gpio::gpioc::{PC0, PC1, PC2, PC3};

use cortex_m::peripheral::Peripherals;

struct Leds {
   left_red:    PC0<gpio::Output<gpio::OpenDrain>>,
   left_green:  PC1<gpio::Output<gpio::OpenDrain>>,
   right_green: PC2<gpio::Output<gpio::OpenDrain>>,
   right_red:   PC3<gpio::Output<gpio::OpenDrain>>,
}

// use `main` as the entry point of this application
// `main` is not allowed to return
#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        let gpioc = p.GPIOC.split();

        // Configure LED outputs
        let mut leds = Leds {
          left_red: gpioc.pc0.into_open_drain_output(),
          left_green: gpioc.pc1.into_open_drain_output(),
          right_green: gpioc.pc2.into_open_drain_output(),
          right_red: gpioc.pc3.into_open_drain_output(),
        };

        // Constrain clock registers
        let mut rcc = p.RCC.constrain();

        // Configure clock to 168 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();

        // Get delay provider
        let mut delay = Delay::new(cp.SYST, clocks);

        loop {
            // Turn LED on
            leds.left_red.set_high();

            // Delay twice for half a second due to limited timer resolution
            delay.delay_ms(500_u16);
            delay.delay_ms(500_u16);

            // Turn LED off
            leds.left_red.set_low();

            // Delay twice for half a second due to limited timer resolution
            delay.delay_ms(500_u16);
            delay.delay_ms(500_u16);
        }
    }

    loop {}
}
