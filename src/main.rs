#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let _y = 2;
    loop {
        let _x = 3;
    }
}
