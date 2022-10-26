#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_rt::entry; 
use cortex_m_semihosting::{debug, hprintln};

#[entry]
fn main() -> ! {
    hprintln!("success!");

    // exit QEMU
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
