// IMPORTANT the standard `main` interface is not used because it requires nightly
#![no_main]
#![no_std]
#![feature(lang_items)]
extern crate cortex_m_rt;


#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate stm32f4;
use stm32f4::stm32f405;

// This function is "public to the root" meaning that it's available to external
// crates linking against this one.
pub fn init_and_set_bit() 
{
	let peripherals = stm32f405::Peripherals::take().unwrap();
	let gpioc = &peripherals.GPIOC;
	gpioc.odr.modify(|_, w| w.odr0().set_bit());
}

// makes `panic!` print messages to the host stderr using semihosting
//extern crate panic_semihosting;

use cortex_m_rt::entry;

// use `main` as the entry point of this application
// `main` is not allowed to return
#[entry]
fn main() -> ! {
    // initialization
    init_and_set_bit();
    loop {
        // application logic
    }
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
